use std::fs;

use petgraph::dot::{Config, Dot};
use rawdata::{RawData, Recipe};
use recipe_graph::RecipeGraph;

mod rawdata;
mod recipe_graph;

fn find_recipes_with_one_input(data: &RawData) -> Vec<&Recipe> {
    data.recipes
        .values()
        .filter(|r| r.ingredients.len() == 1 && r.in_machine)
        .collect::<Vec<&Recipe>>()
}

fn main() {
    println!("Hello, world!");
    let data = RawData::load();
    let graph = RecipeGraph::new(&data);
    fs::write(
        "out.txt",
        format!(
            "{:?}",
            Dot::with_config(&graph.graph, &[Config::EdgeNoLabel])
        ),
    )
    .unwrap();
    println!(
        "{:?} nodes {:?} edges",
        graph.graph.node_count(),
        graph.graph.edge_count()
    );
    println!("Input leaves: {:?}", graph.find_input_leaves());
}
