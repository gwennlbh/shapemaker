use crate::video::engine::EngineOutput;
use anyhow::Result;

pub mod ffmpeg;
pub mod vgv;

pub trait Encoder {
    fn name(&self) -> String;
    fn encode_frame(&mut self, output: EngineOutput) -> Result<()>;
    fn finish(&mut self) -> Result<()>;
    fn finish_message(&self, time_elapsed: std::time::Duration) -> String;
    fn progress_message(&self, current: u64, total: u64) -> String {
        format!("{}/{} frames", current, total,)
    }
}
