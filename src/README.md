# Shapemaker's submodules

In approximate order of dependency:

- `geometry`: Types and operations that don't have an inherent graphical representation but are used to define more complex objects: `Point`, `Region`, etc
- `graphics`: Types and operations for objects that can get rendered: all the `Object` variants (lines, circles, etc), the `Canvas`, `Layer`s, etc
- `random`: Methods named `random_...` on various types, to help with semi-random procedural artworks
- `rendering`: Traits that pertain to rendering graphical elements to an SVG tree and their various implementations on most structs from `graphics`
- `synchronization`: Data structures and methods to synchronize various musical sources (audio files, MIDI files, etc) with the video
- `video`: Data structure, user API (mainly adding hooks), rendering and encoding of `Canvas` "frames" into a H.264 MP4 video file
- `vst`: A VST to interact with shapemaker within a DAW directly [wip]
- `wasm`: WASM-specific code to provide a JS API to shapemaker, to, for example, make it interact with the WebMIDI API 

See the individual READMEs in each submodule for more information.
