use crate::core::process::*;
pub struct Data {
    pub payload: Vec<Vec<f64>>,
}

impl Data {
    pub fn start(&self) {
        let runner = Maai::new(self.payload.clone());
        runner.run();
    }
}
