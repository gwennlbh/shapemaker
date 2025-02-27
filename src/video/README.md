# Shapemaker Videos

Creation of shapemaker videos is implemented through a video engine, that allows library users to hook into moments of the song (see `../synchronization`) and modify the video's `Canvas` (see `../graphics`) to change it for the frames to come, until it is changed again.

The definition of types and methods to register these hooks, is in `engine.rs`.

Some hooks are named "later hooks" because they are triggered manually while running another hook. These are manily used to implement animations, see `animation.rs`

## Rendering logic

The rendering and encoding is done in `encoding.rs`:

The rendering loop goes as follows:

1. Iterate over every millisecond value for the duration of the song
2. For every hook (that were attached to the struct by other method calls, see `engine.rs`), determine if the hook needs to run, and if it does, run it, passing it a mutable reference to the canvas as well as a "Context" struct instance, that gives useful information such as the curent timestamp, current beat, etc.
3. If the current millisecond value corresponds to a new video frame (calculated by taking into account the target FPS), render the canvas to SVG.(see `../rendering`)
4. That string is then rendered to matrix of pixels by the [resvg](https://crates.io/crates/resvg) crate
3. That matrix is a flat 2D array of RGBA values, which we convert to the representation ffmpeg needs (a "Height-Width-Channels" 3-dimensionnal array)
4. These HWC frames are given to the encoder (libx264, used through the [video-rs](https://crates.io/crates/video-rs) crate) to create the final video file
