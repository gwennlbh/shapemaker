import os
from pathlib import Path
from subprocess import run
from sys import argv
from time import time_ns

from rich.console import Console
from rich.table import Table

here = Path(__file__).parent

ignored_tasks = []

compare_with = ""
if len(argv) > 1:
    compare_with = Path(argv[1]).read_text(encoding="utf-8").strip()
    compare_with = {
        (line.split(",")[0], float(line.split(",")[1]), int(line.split(",")[2]))
        for line in compare_with.splitlines()[1:]
    }
    print(compare_with)


def avg(numbers: list[float]):
    return sum(numbers) / len(numbers)


end = 0
start = 0

if not Path("timings.log").exists():
    start = time_ns()
    result = run(
        ["just", "schedule-hell", here.parent / "out.mp4", "--duration 5", "--resolution 320"],
        capture_output=True,
        env=os.environ | {"RUST_LOG": "debug", "RUST_BACKTRACE": "full"},
    )
    end = time_ns()

    Path("timings.log").write_bytes(result.stdout + result.stderr)

timings = [
    line.split(" took ")
    for line in Path("timings.log").read_text(encoding="utf-8").splitlines()
    if " took " in line
]


def parse_duration(duration_string: str) -> float:
    if " " in duration_string:
        return sum(parse_duration(part) for part in duration_string.split(" "))
    try:
        figure = float(duration_string.strip("msµ"))
    except ValueError:
        return None

    if "µs" in duration_string:
        return figure * 1e-3
    if "ms" in duration_string:
        return figure
    if "s" in duration_string:
        return figure * 1e3
    else:
        return figure

    raise ValueError(f"Duration string {duration_string!r} has unsupported unit")


timings = [
    (label.split("] ")[1], parse_duration(timing))
    for label, timing in timings
    if parse_duration(timing)
]

per_function: dict[str, list[float]] = {function: [] for function, _ in timings}

for function, timing in timings:
    per_function[function].append(timing)

averages: list[tuple[str, float, int]] = [
    (function, avg(timings), len(timings)) for function, timings in per_function.items()
] + [
    ("_Total_", (end - start) / 1e6, 1),
]

averages.sort(key=lambda item: item[1])

header = ["Tâche", "Durée [ms]", "#"]

if compare_with:
    formatted_results = []
    for function, timing_after, count_after in averages:
        if function not in {function for function, _, _ in compare_with}:
            continue
        timing_before = next(
            (timing for fn, timing, _ in compare_with if fn == function),
            None,
        )
        count_before = next(
            (count for fn, _, count in compare_with if fn == function),
            None,
        )
        if timing_before is None or count_before is None:
            continue
        if function in ignored_tasks:
            continue

        formatted_results.append(
            [
                function,
                f"{timing_after:.3f}",
                f"{timing_before:.3f}",
                (
                    f"{timing_after - timing_before:+.3f}"
                    if f"{timing_after - timing_before:.3f}" != "-0.000"
                    else "±0"
                ),
                f"{count_after}",
                f"{count_before}",
                (
                    f"{count_after - count_before:+}"
                    if count_after != count_before
                    else "±0"
                ),
            ]
        )

    header = [
        "Tâche",
        "Durée [ms]",
        "Durée [ms] avant",
        "Différence [±ms]",
        "# après",
        "# avant",
        "Différence",
    ]
else:
    formatted_results = [
        [function, f"{timing:.3f}", f"{count}"] for function, timing, count in averages
    ]


def to_csv(lists: list[list[str]]):
    return "\r\n".join(
        ",".join(cell.replace(",", "_") for cell in line) for line in lists
    )


table = Table(*header)
for row in formatted_results:
    table.add_row(*row)
Console().print(table)

Path("results.csv").write_text(
    to_csv(
        [
            header,
            *[
                [task, *things]
                for task, *things in formatted_results
                if task not in ignored_tasks
            ],
        ]
    ),
    encoding="utf8",
)
