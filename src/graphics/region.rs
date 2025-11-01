use crate::Region;

impl Region {
    pub fn clip_path_id(&self) -> String {
        format!("clip-{}-{}", self.start, self.end)
    }
}
