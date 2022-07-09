#[path = "./process.rs"]
mod process;

pub struct Data {
    pub payload: Vec<Vec<f64>>,
}

impl Data {
    pub fn start(&self) {
        let runner = process::Max::new(self.payload.clone());
        runner.run();
    }
}
