use anyhow::Result;
#[cfg(feature = "vst")]
#[cfg(feature = "mp4")]
use env_logger;
use measure_time::info_time;
use shapemaker::{
    cli::{canvas_from_cli, cli_args},
    *,
};

extern crate log;

pub fn main() -> Result<()> {
    #[cfg(feature = "vst")]
    #[cfg(feature = "mp4")]
    env_logger::init();
    run(cli_args())
}

pub fn run(args: cli::Args) -> Result<()> {
    info_time!("run");
    let mut canvas = canvas_from_cli(&args);

    if args.cmd_examples {
        canvas = if args.cmd_dna_analysis_machine {
            examples::dna_analysis_machine()
        } else if args.cmd_shapeshed {
            examples::shapes_shed()
        } else if args.cmd_colors_shed {
            examples::colors_shed()
        } else {
            panic!("Specify the example to use")
        };

        if args.arg_file.ends_with(".svg") {
            std::fs::write(
                args.arg_file,
                canvas
                    .render_to_svg(
                        canvas.colormap.clone(),
                        canvas.cell_size,
                        canvas.object_sizes,
                        "",
                    )?
                    .to_string(),
            )
            .unwrap();
        } else {
            match canvas.render_to_png(
                &args.arg_file,
                args.flag_resolution.unwrap_or(1000),
                None,
            ) {
                Ok(_) => println!("Image saved to {}", args.arg_file),
                Err(e) => println!("Error saving image: {}", e),
            }
        }
        Ok(())
    } else if args.cmd_video {
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
        .sync_audio_with(&args.flag_sync_with.expect(
            "Provide MIDI sync file with --sync-with to render a video",
        ))
        .each_frame(&|canvas, ctx| {
            let center = canvas.world_region.center();
            canvas.root().clear();
            canvas.root().add_object(
                "text",
                Object::CenteredText(center, ctx.timestamp.to_string(), 30.0)
                    .color(Fill::Solid(Color::White)),
            );
            canvas.root().add_object(
                "beat",
                Object::CenteredText(
                    center.translated(0, 3),
                    format!("beat {}", ctx.beat),
                    30.0,
                )
                .color(Fill::Solid(Color::Cyan)),
            );
            Ok(())
        })
        .render(args.arg_file)
}
