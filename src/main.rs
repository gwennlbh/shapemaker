use anyhow::Result;
use measure_time::info_time;
use shapemaker::{
    cli::{canvas_from_cli, cli_args},
    *,
};

#[macro_use]
extern crate log;

pub fn main() -> Result<()> {
    env_logger::init();
    run(cli_args())
}

pub fn run(args: cli::Args) -> Result<()> {
    info_time!("run");
    let mut canvas = canvas_from_cli(&args);

    if args.cmd_image && !args.cmd_video {
        canvas = examples::title();

        if args.arg_file.ends_with(".svg") {
            std::fs::write(args.arg_file, canvas.render_to_svg()?).unwrap();
        } else {
            match canvas.render_to_png(&args.arg_file, args.flag_resolution.unwrap_or(1000), None) {
                Ok(_) => println!("Image saved to {}", args.arg_file),
                Err(e) => println!("Error saving image: {}", e),
            }
        }
        return Ok(());
    }

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
            &args
                .flag_sync_with
                .expect("Provide MIDI sync file with --sync-with to render a video"),
        )
        .each_frame(&|canvas, ctx| {
            let center = canvas.world_region.center();
            canvas.root().clear();
            canvas.root().add_object(
                "text",
                Object::CenteredText(
                    center,
                    format!(
                        "{} #{} beat {}",
                        ctx.timestamp, ctx.frame, ctx.beat_fractional
                    ),
                    30.0,
                )
                .color(Fill::Solid(Color::White)),
            );
            Ok(())
        })
        .render(args.arg_file)
}
