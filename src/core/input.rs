#[path = "./process.rs"]
mod process;

pub struct Data {
    pub payload: &'static str,
}

impl Data {
    pub fn start(&self) {
        let runner = process::Max::new();
        runner.run();
    }
}
