use std::collections::HashMap;

use petgraph::{graph::NodeIndex, Direction, Graph};

use crate::rawdata::RawData;

#[derive(Debug)]
pub struct RecipeGraph<'a> {
    data: &'a RawData,
    pub graph: Graph<&'a str, &'a str>,
    indices: HashMap<&'a str, NodeIndex>,
}

impl<'a> RecipeGraph<'a> {
    pub fn new(data: &'a RawData) -> Self {
        let mut graph = Graph::new();
        let mut indices = HashMap::new();
        for (name, _item) in &data.items {
            let idx = graph.add_node(name.as_str());
            indices.insert(name.as_str(), idx);
        }
        for (name, recipe) in &data.recipes {
            if recipe.produced_in.is_empty() || recipe.produced_in[0] == "Desc_Converter_C" {
                continue;
            }
            for p in &recipe.products {
                if let Some(product_idx) = indices.get(p.item.as_str()) {
                    for ingredient in &recipe.ingredients {
                        let ingredient_idx = indices[ingredient.item.as_str()];
                        graph.add_edge(ingredient_idx, *product_idx, name.as_str());
                    }
                }
            }
        }

        Self {
            data,
            graph,
            indices,
        }
    }

    pub fn find_input_leaves(&self) -> Vec<String> {
        let mut leaves = Vec::new();
        for (name, idx) in &self.indices {
            if self
                .graph
                .neighbors_directed(*idx, Direction::Incoming)
                .count()
                == 0
                && self
                    .graph
                    .neighbors_directed(*idx, Direction::Outgoing)
                    .count()
                    > 0
            {
                leaves.push(name.to_string());
            }
        }
        leaves
    }
}
