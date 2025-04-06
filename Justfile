export RUST_BACKTRACE := "1"
install_at := replace(home_directory(), "\\", "/") / ".local/bin"

build:
    cargo build --bin shapemaker
    cp target/debug/shapemaker .

vst:
    cargo xtask bundle shapemaker --release --features vst
    gsudo cp "target/bundled/Shapemaker VST.vst3/Contents/x86_64-win/Shapemaker VST.vst3" "C:/Program Files/Common Files/VST3/Shapemaker VST.vst3"

beacon out="out.mp4" args="":
    ./shapemaker beacon start {{out}} {{args}}

web:
    wasm-pack build --target web -d examples/web --features web --no-default-features
    touch examples/web/.nojekyll
    echo "" >> examples/web/.gitignore
    echo "!index.html" >> examples/web/.gitignore

start-web:
    just web
    python3 -m http.server --directory examples/web

install:
    mkdir -p {{install_at}}
    cp shapemaker {{install_at}}

example-video out="out.mp4" args='':
    RUST_BACKTRACE=full ./shapemaker test-video --colors examples/colorschemes/palenight.css {{out}} --sync-with examples/schedule-hell/schedule-hell.midi --audio examples/schedule-hell/schedule-hell.flac --grid-size 16x10 --resolution 480 {{args}}

[working-directory: 'paper']
paper:
    # just analyze_times  disabled because it needs manual adjustements in the render loop pipeline diagram
    cargo run --package specimen
    cargo run --package dna-analysis-machine
    typstyle format-all ../paper # . does not work, it formats nothing
    typst compile --root .. main.typ

readme:
    cd examples/gallery; ./fill.rb

timings compare_with="":
    just
    python script/debug-performance.py {{compare_with}}
