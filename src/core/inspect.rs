use crate::core::layer::Layer;

pub fn inspect(layers: &Vec<Layer>) {
    let mut layer_count = 0;
    let mut neuron_count = 0;

    for layer in layers {
        layer_count += 1;
        for _ in &layer.neurons {
            neuron_count += 1;
        }
    }

    println!("Layers: {} | Neurons: {}", layer_count, neuron_count);
}
