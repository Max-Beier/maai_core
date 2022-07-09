#[path = "./utils/mod.rs"]
mod utils;

pub struct Max {
    pub data: &'static str,
    pub layerCount: i32,
    pub layers: Vec<utils::layer::Layer>,
}

impl Max {
    pub fn new() -> Max {
        let mut max = Max {
            data: "",
            layerCount: 10,
            layers: Vec::new(),
        };

        for i in 0..max.layerCount {
            let layer = utils::layer::Layer::new(i);
            max.layers.push(layer);
        }

        const BIAS: f64 = 10.0;
        for layer in &max.layers {
            if (layer.index == 0) {
                continue;
            };
            let mut sum: f64 = 0.0;
            for weight in &layer.weights {
                for neuron in &layer.neurons {
                    sum += neuron.activation * weight.value;
                }
            }
            sum -= BIAS;
            println!("{:?}", sigmoid(sum));
        }

        return max;
    }

    pub fn run(&self) {
        //println!("{:?}", self.layers);
    }
}

fn sigmoid(x: f64) -> f64 {
    return 1.0 / (1.0 + f64::exp(-x));
}
