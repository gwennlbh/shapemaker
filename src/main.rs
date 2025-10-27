use anyhow::Result;
use shapemaker::*;

#[cfg(feature = "vst")]
#[cfg(feature = "video")]
use env_logger;
use measure_time::debug_time;

#[cfg(feature = "cli")]
use shapemaker::cli;

extern crate log;

#[cfg(not(feature = "cli"))]
pub fn main() -> Result<()> {
    panic!(
        "Running the command-line program requires the cli feature to be enabled. Enabled features: {:?}",
        enabled_features()
    );
}

#[cfg(feature = "cli")]
#[tokio::main]
pub async fn main() -> Result<()> {
    #[cfg(feature = "vst")]
    #[cfg(feature = "video")]
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

    if args.cmd_test_video {
        run_video(args)
    } else if args.cmd_beacon && args.cmd_start {
        run_beacon_start(args)
    } else if args.cmd_beacon && args.cmd_ping {
        run_beacon_ping(args)
    } else {
        Ok(())
    }
}

#[cfg(all(feature = "cli", not(feature = "vst")))]
fn run_beacon_start(_args: cli::Args) -> Result<()> {
    println!(
        "VST support is disabled. Enable the vst feature to use VST beaconing."
    );
    Ok(())
}

#[cfg(all(feature = "cli", feature = "vst"))]
fn run_beacon_start(_args: cli::Args) -> Result<()> {
    pub use vst::beacon::Beacon;
    Beacon::start()
}

#[cfg(all(feature = "cli", not(feature = "vst")))]
fn run_beacon_ping(_args: cli::Args) -> Result<()> {
    println!(
        "VST support is disabled. Enable the vst feature to use VST beaconing."
    );
    Ok(())
}

#[cfg(all(feature = "cli", feature = "vst"))]
fn run_beacon_ping(_args: cli::Args) -> Result<()> {
    use rand;
    use vst::remote_probe::RemoteProbe;
    let mut probe = RemoteProbe::new(rand::random());
    Ok(probe.say("ping hehe")?)
}

#[cfg(all(feature = "cli", not(feature = "video")))]
fn run_video(_args: cli::Args) -> Result<()> {
    println!(
        "Video rendering is disabled. Enable the video feature to render videos."
    );
    Ok(())
}

#[cfg(all(feature = "cli", feature = "video"))]
fn run_video(args: cli::Args) -> Result<()> {
    use shapemaker::fonts::FontOptions;

    let mut canvas = cli::canvas_from_cli(&args);
    canvas.set_background(Color::Black);
    canvas.font_options = FontOptions {
        monospace_family: Some("Victor Mono".into()),
        ..Default::default()
    };
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
            canvas
                .root()
                .set("feur", Object::Dot(center).colored(Color::Red));
            canvas.root().set(
                "text",
                Object::CenteredText(center, ctx.timestamp(), 30.0)
                    .colored(Color::White),
            );
            canvas.root().set(
                "beat",
                Object::CenteredText(
                    center.translated(0, 3),
                    format!("beat {}", ctx.beat()),
                    30.0,
                )
                .colored(Color::Cyan),
            );
            Ok(())
        })
        .encode(args.arg_file)
}
