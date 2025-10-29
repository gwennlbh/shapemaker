use crate::{
    Canvas, Video,
    rendering::svg,
    ui,
    video::{encoders::Encoder, engine::EngineOutput},
};
use ::vgv::Transcoder;
use anyhow::Result;
use itertools::Itertools;
use std::path::PathBuf;

#[derive(strum_macros::Display)]
pub enum VGVTranscodeMode {
    ToHTML,
    None,
}

pub struct VGVEncoder {
    destination: PathBuf,
    encoder: ::vgv::Encoder,
    transcode: VGVTranscodeMode,
}

impl<C: Default> Video<C> {
    pub fn setup_vgv_encoder(
        &self,
        transcode: VGVTranscodeMode,
        width: usize,
        height: usize,
        initial_canvas: &Canvas,
        destination: impl Into<PathBuf>,
    ) -> Result<VGVEncoder> {
        Ok(VGVEncoder {
            transcode,
            destination: destination.into(),
            encoder: ::vgv::Encoder::new(::vgv::Frame::Initialization {
                w: width as _,
                h: height as _,
                d: (1000.0 / self.fps as f64) as _,
                bg: initial_canvas
                    .background
                    .unwrap_or_default()
                    .render(&initial_canvas.colormap),
                svg: format!(
                    r#"width={w} height={h} viewBox="-{pad} -{pad} {w} {h}""#,
                    w = initial_canvas.width(),
                    h = initial_canvas.height(),
                    pad = initial_canvas.canvas_outer_padding
                ),
            }),
        })
    }
}

impl Encoder for VGVEncoder {
    fn name(&self) -> String {
        "VGV".into()
    }

    fn encode_frame(&mut self, output: EngineOutput) -> Result<()> {
        if let EngineOutput::Frame { ref svg, .. } = output {
            self.encoder.encode_svg(match svg {
                svg::Node::Text(text) => text.to_string(),
                svg::Node::SVG(svg) => svg.to_string(),
                svg::Node::Element(element) => element
                    .children
                    .iter()
                    .map(|child| child.to_string())
                    .join(""),
            });
        }

        Ok(())
    }

    fn finish(&mut self) -> Result<()> {
        match self.transcode {
            VGVTranscodeMode::ToHTML => {
                // FIXME: not good!!
                let frames = self.encoder.frames.clone();
                std::fs::write(
                    self.destination.clone(),
                    vgv::HTMLTranscoder::new()
                        .encode(frames)
                        .expect("Couldn't transcode VGV to HTML")
                        .as_bytes(),
                )?;
            }

            VGVTranscodeMode::None => {
                let mut file = std::fs::File::create(self.destination.clone())?;
                self.encoder.dump(&mut file);
            }
        }
        Ok(())
    }

    fn finish_message(&self, time_elapsed: std::time::Duration) -> String {
        match self.transcode {
            VGVTranscodeMode::None => format!(
                "VGV video to {} in {}",
                ui::format_filepath(&self.destination),
                ui::format_duration(time_elapsed)
            ),
            VGVTranscodeMode::ToHTML => format!(
                "HTML player for VGV video to {} in {}",
                ui::format_filepath(&self.destination),
                ui::format_duration(time_elapsed)
            ),
        }
    }
}
