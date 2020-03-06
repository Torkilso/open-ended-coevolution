use std::collections::HashMap;

use crate::generic_neat::genome::Genome;
use crate::generic_neat::node::NodeRef;
use crate::network::activation::Activation;
use crate::network::order;

#[derive(Clone, Debug)]
pub enum Action {
    Link(usize, usize, f64),
    // from, to, weight
    Activation(usize, f64, Activation), // node, bias, activation
}

pub struct NeuralNetwork {
    values: Vec<f64>,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
    actions: Vec<Action>,
}

impl NeuralNetwork {
    /*pub fn new(
        length: usize,
        inputs: Vec<usize>,
        outputs: Vec<usize>,
        actions: Vec<Action>,
    ) -> NeuralNetwork {



        NeuralNetwork {
            values: vec![0.0; length],
            inputs,
            outputs,
            actions,
        }
    }*/

    pub fn new(
        genome: &Genome
    ) -> NeuralNetwork {
        let input_length = genome.inputs.len();
        let cumulative_hidden_length = input_length + genome.hidden_nodes.len(); // Length of input and hidden
        let cumulative_output_length = cumulative_hidden_length + genome.outputs.len(); // Length of input, hidden and output

        let mut input_keys: Vec<NodeRef> = genome.inputs.keys().cloned().collect();
        input_keys.sort();
        let mut output_keys: Vec<NodeRef> = genome.outputs.keys().cloned().collect();
        output_keys.sort();

        let node_mapper: HashMap<NodeRef, usize> = input_keys
            .iter()
            .enumerate()
            .map(|(i, node_ref)| (*node_ref, i))
            .chain(
                genome
                    .hidden_nodes
                    .keys()
                    .enumerate()
                    .map(|(i, node_ref)| (*node_ref, i + input_length)),
            )
            .chain(
                output_keys
                    .iter()
                    .enumerate()
                    .map(|(i, node_ref)| (*node_ref, i + cumulative_hidden_length)),
            )
            .collect();

        let actions = genome
            .order
            .iter()
            .map(|action| match action {
                order::Action::Link(from, to) => Action::Link(
                    *node_mapper.get(from).unwrap(),
                    *node_mapper.get(to).unwrap(),
                    genome.links.get(&(*from, *to)).unwrap().weight,
                ),
                order::Action::Activation(node) => Action::Activation(
                    *node_mapper.get(node).unwrap(),
                    genome.get_bias(node),
                    genome.get_activation(node),
                ),
            })
            .collect();

        NeuralNetwork {
            values: vec![0.0; cumulative_output_length],
            inputs: input_keys.iter().map(|node| node.id() as usize).collect(),
            outputs: output_keys
                .iter()
                .map(|node| node.id() as usize + cumulative_hidden_length)
                .collect(),
            actions,
        }
    }

    /// Evaluate network, takes input node values, returns output node values
    pub fn activate(&mut self, inputs: &Vec<f64>) -> Vec<f64> {
        for i in 0..self.values.len() {
            self.values[i] = 0.0;
        }

        // Clear network
        // Same as loop above, but this unsafe implementation is faster
        /*unsafe {
            libc::memset(
                self.values.as_mut_ptr() as _,
                0,
                self.values.len() * mem::size_of::<f64>(),
            );
        }*/

        // Copy inputs into values
        for (i, index) in self.inputs.iter().enumerate() {
            self.values[i] = inputs[*index];
        }

        // Do forward pass
        for action in self.actions.iter() {
            match action {
                Action::Link(from, to, weight) => {
                    self.values[*to] += self.values[*from] * weight;
                }
                Action::Activation(node, bias, activation) => {
                    self.values[*node] = activation.activate(self.values[*node] + *bias)
                }
            }
        }

        // Collect output
        self.outputs
            .iter()
            .map(|o| {
                if self.values[*o].is_finite() {
                    self.values[*o]
                } else {
                    0.0
                }
            })
            .collect()
    }
}
