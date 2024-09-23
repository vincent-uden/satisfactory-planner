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
    let tier_0 = vec![
        "Desc_OreGold_C".to_string(),
        "Desc_OreIron_C".to_string(),
        "Desc_OreCopper_C".to_string(),
        "Desc_RawQuartz_C".to_string(),
        "Desc_Coal_C".to_string(),
        "Desc_Stone_C".to_string(),
        "Desc_LiquidOil_C".to_string(),
        "Desc_Sulfur_C".to_string(),
        "Desc_OreUranium_C".to_string(),
        "Desc_OreBauxite_C".to_string(),
        "Desc_Water_C".to_string(),
        "Desc_NitrogenGas_C".to_string(),
    ];
    println!("Input leaves: {:#?}", tier_0);
    let tiers = graph.find_n_tiers(9, &tier_0);
    let mut i = 0;
    for tier in &tiers {
        println!("Tier {}:", i);
        for slug in tier {
            println!("  {}", data.items[slug].name);
        }
        println!("");
        i += 1;
    }
}
