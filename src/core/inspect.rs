use crate::core::layer::Layer;

pub fn inspect(layers: &Vec<Layer>) {
    let mut layer_count = 0;
    let mut neuron_count = 0;
    let mut weights_count = 0;

    for layer in layers {
        layer_count += 1;
        for neuron in &layer.neurons {
            neuron_count += 1;
            for _ in 0..neuron.weights_from.len() {
                weights_count += 1;
            }
        }
    }

    println!(
        "Layers: {} | Neurons: {} | Weights: {}",
        layer_count, neuron_count, weights_count
    );
}
