use std::collections::HashMap;

use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction, Graph};

use crate::rawdata::{RawData, Recipe};

#[derive(Debug)]
pub struct RecipeGraph<'a> {
    data: &'a RawData,
    pub graph: Graph<&'a str, &'a Recipe>,
    indices: HashMap<&'a str, NodeIndex>,
    rev_indices: HashMap<NodeIndex, &'a str>,
}

impl<'a> RecipeGraph<'a> {
    pub fn new(data: &'a RawData) -> Self {
        let mut graph = Graph::new();
        let mut indices = HashMap::new();
        let mut rev_indices = HashMap::new();
        for (name, _item) in &data.items {
            let idx = graph.add_node(name.as_str());
            indices.insert(name.as_str(), idx);
            rev_indices.insert(idx, name.as_str());
        }
        for (name, recipe) in &data.recipes {
            if recipe.produced_in.is_empty() || recipe.produced_in[0] == "Desc_Converter_C" {
                continue;
            }
            for p in &recipe.products {
                if let Some(product_idx) = indices.get(p.item.as_str()) {
                    for ingredient in &recipe.ingredients {
                        let ingredient_idx = indices[ingredient.item.as_str()];
                        graph.add_edge(ingredient_idx, *product_idx, recipe);
                    }
                }
            }
        }

        Self {
            data,
            graph,
            indices,
            rev_indices,
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

    pub fn find_next_tier(&self, previous_tiers: &Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut next = Vec::new();
        for name in previous_tiers.last().unwrap() {
            let idx = self.indices.get(name.as_str()).unwrap();
            let mut products = Vec::new();
            for e in self.graph.edges_directed(*idx, Direction::Outgoing) {
                if !e.weight().alternate {
                    let mut all_from_previous_tier = true;
                    let mut has_not_appeared_in_previous_tier = true;
                    for input in &e.weight().ingredients {
                        let mut contained_in_any_previous_tier = false;
                        for tier in previous_tiers {
                            if tier.contains(&input.item) {
                                contained_in_any_previous_tier = true;
                            }
                            if tier.contains(&self.rev_indices[&e.target()].to_string()) {
                                has_not_appeared_in_previous_tier = false;
                            }
                        }
                        all_from_previous_tier &= contained_in_any_previous_tier;
                    }
                    if all_from_previous_tier && has_not_appeared_in_previous_tier {
                        products.push(self.graph[e.target()].to_string());
                    }
                }
            }
            products.sort();
            products.dedup();
            products.reverse();
            next.push(products);
        }
        next
    }

    pub fn find_n_tiers(&self, n: usize, start: &Vec<String>) -> Vec<Vec<String>> {
        let mut tiers = Vec::new();
        tiers.push(start.clone());
        for i in 0..n {
            let mut next_tier = self
                .find_next_tier(&tiers)
                .into_iter()
                .flatten()
                .collect::<Vec<String>>();
            next_tier.sort();
            next_tier.dedup();
            println!(
                "Tier {} contains {} items: {:?}",
                i + 1,
                next_tier.len(),
                next_tier
            );
            tiers.push(next_tier);
        }
        tiers
    }
}
