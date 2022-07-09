#[derive(Debug)]
pub struct Neuron {
    pub value: f64,
}

impl Neuron {
    pub fn new() -> Neuron {
        return Neuron { value: 0.0 };
    }
}
