from subprocess import run
from pathlib import Path
from rich.table import Table
from rich.console import Console
import os

ignored_tasks = ["render_frames", "render", "sync_audio_with", "run", "canvas_from_cli"]


def avg(numbers: list[float]):
    return sum(numbers) / len(numbers)


if not Path("timings.log").exists():
    result = run(
        ["just", "example-video", "out.mp4", "--duration 5"],
        capture_output=True,
        env=os.environ | {"RUST_LOG": "debug"},
    )

    Path("timings.log").write_bytes(result.stdout + result.stderr)

timings = [
    line.split(" took ")
    for line in Path("timings.log").read_text().splitlines()
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
]

averages.sort(key=lambda item: item[1])

formatted_results = [
    [function, f"{timing:.3f}", f"{count}"] for function, timing, count in averages
]


def to_csv(lists: list[list[str]]):
    return "\r\n".join(
        ",".join(cell.replace(",", "_") for cell in line) for line in lists
    )


table = Table("task", "time [ms]", "count")
for row in formatted_results:
    table.add_row(*row)
Console().print(table)

Path("results.csv").write_text(
    to_csv(
        [
            ["Tâche", "Durée [ms]", "#"],
            *[
                [task, *things]
                for task, *things in formatted_results
                if task not in ignored_tasks
            ],
        ]
    ),
    encoding="utf8",
)
