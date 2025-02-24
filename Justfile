export RUST_BACKTRACE := "1"

build:
    cargo build --bin shapemaker
    cp target/debug/shapemaker .

vst:
    cargo xtask bundle shapemaker --release
    gsudo cp "target/bundled/Shapemaker VST.vst3/Contents/x86_64-win/Shapemaker VST.vst3" "C:/Program Files/Common Files/VST3/Shapemaker VST.vst3"

web:
    wasm-pack build --target web -d web
    echo "" >> web/.gitignore
    echo "!index.html" >> web/.gitignore

start-web:
    just web
    python3 -m http.server --directory web

install:
    cp shapemaker ~/.local/bin/

example-video out="out.mp4" args='':
    RUST_BACKTRACE=1 ./shapemaker video --colors colorschemes/palenight.css {{out}} --sync-with fixtures/schedule-hell.midi --audio fixtures/schedule-hell.flac --grid-size 16x10 --resolution 480 {{args}} --duration 10

example-image out="out.png" args='':
    ./shapemaker image --colors colorschemes/palenight.css --resolution 1400 {{out}}   {{args}}
