export RUST_BACKTRACE := "1"

build:
    cargo build --bin shapemaker --features mp4,vst
    cp target/debug/shapemaker .

vst:
    cargo xtask bundle shapemaker --release
    gsudo cp "target/bundled/Shapemaker VST.vst3/Contents/x86_64-win/Shapemaker VST.vst3" "C:/Program Files/Common Files/VST3/Shapemaker VST.vst3"

web:
    wasm-pack build --target web -d examples/web --features web
    echo "" >> examples/web/.gitignore
    echo "!index.html" >> examples/web/.gitignore

start-web:
    just web
    python3 -m http.server --directory examples/web

install:
    cp shapemaker ~/.local/bin/

example-video out="out.mp4" args='':
    RUST_BACKTRACE=full ./shapemaker video --colors examples/colorschemes/palenight.css {{out}} --sync-with examples/schedule-hell.midi --audio examples/schedule-hell.flac --grid-size 16x10 --resolution 480 {{args}}

example-image out="out.png" args='':
    ./shapemaker image --colors examples/colorschemes/palenight.css --resolution 1400 {{out}}   {{args}}

readme:
    #!/usr/bin/env bash
    cd examples/gallery
    ./fill.rb
