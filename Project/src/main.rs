// This is the entry point of the program
//It loads the dataset and parses it into Person records
//It creates a graph of the people and their connections based on similarity and performs analysis

mod parser;
mod graph;
mod similarity;
mod analysis;
#[cfg(test)]
mod tests;

use parser::load_people;
use graph::HealthGraph;
use similarity::is_similar;
use analysis::{average_degree, node_w_highest_degree, average_shortest_path_length};
use rand::seq::SliceRandom;
use rand::thread_rng;
use analysis::analyze_health_by_income_and_food_security;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //load the dataset from CSV file
    let mut people = load_people("health_dataset.csv")?;
    println!("{} people were loaded.", people.len());
    //shuffle the dataset and take a sample of 10000 people
    let mut rng = thread_rng();
    people.shuffle(&mut rng); //shuffle the dataset
    let people = people.into_iter().take(10000).collect::<Vec<_>>();
    println!("{} people were selected for the graph.", people.len());
    //create a graph of the people and their connections based on similarity
    let mut graph = HealthGraph::new();
    for person in &people {
        graph.add_node(person.clone());
    }
    println!("{} nodes were added to the graph.", graph.nodes.len());
    //add edges to the graph based on similarity
    let ids: Vec<_> = graph.nodes.keys().copied().collect(); //collect the ids of all nodes
    for i in 0..ids.len() { //iterate over all nodes
        for j in (i+1)..ids.len() { //compare each node with all other nodes
            if is_similar(&graph.nodes[&ids[i]], &graph.nodes[&ids[j]]) { //check if the two nodes are similar
                graph.add_edge(ids[i], ids[j]); //add an edge between the two nodes
            }
        }
    }
    println!("{} edges and {} nodes were added to the graph.", graph.total_edges(), graph.nodes.len());
    //analyze the graph
    println!("\n--- Graph Analysis ---");
    let avg_deg = average_degree(&graph);
    println!("Average connections per person: {:.2}", avg_deg);
    if let Some((id, deg)) = node_w_highest_degree(&graph) { //find the node with the highest degree
        println!("Person {} has the highest degree: {}", id, deg);
    }
    else {
        println!("No nodes in the graph or error finding a node with the highest degree.");
    }
    let avg_shortest_path = average_shortest_path_length(&graph);
    println!("Average shortest path length: {:.2}", avg_shortest_path);
    //analyze the health conditions of people based on their income and food security
    analyze_health_by_income_and_food_security(&people);
    Ok(())
}
