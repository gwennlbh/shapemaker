use anyhow::Result;
use shapemaker::*;

#[cfg(feature = "vst")]
#[cfg(feature = "mp4")]
use env_logger;
use measure_time::debug_time;

#[cfg(feature = "cli")]
use shapemaker::cli;

extern crate log;

#[cfg(not(feature = "cli"))]
pub fn main() -> Result<()> {
    use anyhow::Error;

    Err(("Running the command-line program requires the cli feature to be enabled.").into())
}

#[cfg(feature = "cli")]
#[tokio::main]
pub async fn main() -> Result<()> {
    #[cfg(feature = "vst")]
    #[cfg(feature = "mp4")]
    env_logger::init();
    run(cli::cli_args()).await
}

#[cfg(feature = "cli")]
pub async fn run(args: cli::Args) -> Result<()> {
    debug_time!("run");

    if args.cmd_new {
        return cli::new::new_project(args.arg_name);
    }

    if args.cmd_watch {
        cli::watch::watch_project(
            match args.arg_directory.as_str() {
                "" => ".",
                dir => dir,
            }
            .into(),
        )
        .await?;
        return Ok(());
    }

    let canvas = cli::canvas_from_cli(&args);

    if args.cmd_test_video {
        run_video(args, canvas)
    } else if args.cmd_beacon && args.cmd_start {
        run_beacon_start(args, canvas)
    } else {
        Ok(())
    }
}

#[cfg(not(feature = "vst"))]
fn run_beacon_start(_args: cli::Args, _canvas: Canvas) -> Result<()> {
    println!(
        "VST support is disabled. Enable the vst feature to use VST beaconing."
    );
    Ok(())
}

#[cfg(feature = "vst")]
fn run_beacon_start(_args: cli::Args, _canvas: Canvas) -> Result<()> {
    pub use vst::beacon::Beacon;
    Beacon::start()
}

#[cfg(not(feature = "mp4"))]
fn run_video(_args: cli::Args, _canvas: Canvas) -> Result<()> {
    println!(
        "Video rendering is disabled. Enable the mp4 feature to render videos."
    );
    Ok(())
}

#[cfg(feature = "mp4")]
fn run_video(args: cli::Args, canvas: Canvas) -> Result<()> {
    let mut video = Video::<()>::new(canvas);
    video.duration_override = args.flag_duration.map(|seconds| seconds * 1000);
    video.start_rendering_at = args.flag_start.unwrap_or_default() * 1000;
    video.resolution = args.flag_resolution.unwrap_or(1920);
    video.fps = args.flag_fps.unwrap_or(30);
    video.audiofile = args
        .flag_audio
        .expect("Provide audio with --audio to render a video")
        .into();
    video
        .sync_audio_with(
            &args.flag_sync_with.expect(
                "Provide MIDI sync file with --sync-with to render a video",
            ),
        )
        .each_frame(&|canvas, ctx| {
            let center = canvas.world_region.center();
            canvas.root().clear();
            canvas.root().set_object(
                "text",
                Object::CenteredText(center, ctx.timestamp.to_string(), 30.0)
                    .colored(Color::White),
            );
            canvas.root().set_object(
                "beat",
                Object::CenteredText(
                    center.translated(0, 3),
                    format!("beat {}", ctx.beat),
                    30.0,
                )
                .colored(Color::Cyan),
            );
            Ok(())
        })
        .render(args.arg_file)
}
