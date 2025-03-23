export RUST_BACKTRACE := "1"

build:
    cargo build --bin shapemaker --features mp4,vst
    cp target/debug/shapemaker .

vst:
    cargo xtask bundle shapemaker --release --features vst
    gsudo cp "target/bundled/Shapemaker VST.vst3/Contents/x86_64-win/Shapemaker VST.vst3" "C:/Program Files/Common Files/VST3/Shapemaker VST.vst3"

beacon out="out.mp4" args="":
    ./shapemaker beacon start {{out}} {{args}}

web:
    wasm-pack build --target web -d examples/web --features web
    touch examples/web/.nojekyll
    echo "" >> examples/web/.gitignore
    echo "!index.html" >> examples/web/.gitignore

start-web:
    just web
    python3 -m http.server --directory examples/web

install:
    cp shapemaker ~/.local/bin/

example-video out="out.mp4" args='':
    RUST_BACKTRACE=full ./shapemaker video --colors examples/colorschemes/palenight.css {{out}} --sync-with examples/schedule-hell.midi --audio examples/schedule-hell.flac --grid-size 16x10 --resolution 480 {{args}}

paper:
    just
    # just analyze_times  disabled because it needs manual adjustements in the render loop pipeline diagram
    ./shapemaker examples dna-analysis-machine --resolution 1920 paper/dna-analysis-machine.png
    ./shapemaker examples shapeshed --resolution 1920 paper/shapeshed.svg
    ./shapemaker examples colors-shed --resolution 1920 paper/colorshed.svg
    ./shapemaker examples grid --resolution 1920 paper/grid.svg
    typstyle format-all paper
    typst compile --root . paper/main.typ

readme:
    #!/usr/bin/env bash
    cd examples/gallery
    ./fill.rb

analyze_times:
    just
    rm timings.log
    python script/debug-performance.py
