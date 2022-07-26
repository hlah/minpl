#[derive(Default)]
pub struct TempProvider {
    count: usize,
}

impl TempProvider {
    pub fn get(&mut self) -> String {
        self.count += 1;
        format!("_{}", self.count)
    }
}
