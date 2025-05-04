//This file has analysis functions for the health graph's health trends and statistics

use crate::graph::{HealthGraph, PersonNode};
use std::collections::{HashMap, VecDeque};

//computes the number of connections(degree) for each node in the graph and returns a vector of tuples
pub fn compute_degrees(graph: &HealthGraph) -> Vec<(usize, usize)> {
    graph.nodes.keys().map(|id| (*id, graph.degree(*id))).collect()
}
//computes the average degree for all of the nodes in the graph and returns it
pub fn average_degree(graph: &HealthGraph) -> f64 {
    let total_degree: usize = graph.nodes.keys().map(|id| graph.degree(*id)).sum(); //sum of all degrees
    let num_nodes = graph.nodes.len();
    if num_nodes == 0 {
        0.0
    }
    else {
        total_degree as f64 / num_nodes as f64
    }
}
//computes the node with the highest degree in the graph and returns a tuple of the node id and its degree
pub fn node_w_highest_degree(graph: &HealthGraph) -> Option<(usize, usize)> {
    compute_degrees(graph).into_iter().max_by_key(|&(_, degree)| degree)
}
//computes the average shortest path length in the graph and returns it
pub fn average_shortest_path_length(graph: &HealthGraph) -> f64 {
    let mut total_distance = 0;
    let mut count = 0;
    let ids: Vec<usize> = graph.nodes.keys().copied().collect(); //collect the ids of all nodes
    for &start_id in &ids { 
        let distances = bfs_distances(graph, start_id); //compute distances from start_id to all other nodes
        for &dist in distances.values() {
            if dist > 0 { //ignore the distance to itself
                total_distance += dist;
                count += 1;
            }
        }
    }
    if count == 0 {
        0.0
    } 
    else {
        total_distance as f64 / count as f64
    }
}
//computes the distances from a starting node to all other nodes in the graph using BFS and returns a hashmap of distances
fn bfs_distances(graph: &HealthGraph, start: usize) -> std::collections::HashMap<usize, usize> {
    let mut distances = std::collections::HashMap::new();
    let mut queue = VecDeque::new();
    distances.insert(start, 0); //distance to itself is 0
    queue.push_back(start);
    while let Some(current) = queue.pop_front() { //pop the first element from the queue
        let current_distance = distances[&current];
        if let Some(neighbors) = graph.neighbors(current) { //get the neighbors of the current node
            for &neighbor in neighbors {
                if !distances.contains_key(&neighbor) { //if the neighbor has not been visited
                    distances.insert(neighbor, current_distance + 1); //set the distance to the neighbor
                    queue.push_back(neighbor); //add the neighbor to the queue
                }
            }
        }
    }
    distances
}
//analyzes the health conditions of people based on their income and food security and prints the results
pub fn analyze_health_by_income_and_food_security(people: &[PersonNode]) {
    let mut groups: HashMap<(u8, u8), Vec<&PersonNode>> = HashMap::new(); //group people by income and food security
    for person in people {
        if person.total_income != 9 && person.food_security != 9 { //ignore invalid values
            groups
                .entry((person.total_income, person.food_security)) //create a new entry for the group
                .or_default() //if the group does not exist, create it
                .push(person); //add the person to the group
        }
    }
    println!("\n--- Health Conditions by Income and Food Security ---"); 
    for ((income, food_security), group) in groups { //iterate over the groups
        let count = group.len() as f32; //count the number of people in the group
        if count == 0.0 { continue;} //skip empty groups
        let mut high_bp = 0;
        let mut high_cholesterol = 0;
        let mut diabetic = 0;
        for person in group {
            if person.high_bp { high_bp += 1; } //High blood pressure
            if person.high_cholesterol { high_cholesterol += 1; } //High cholesterol
            if person.diabetic { diabetic += 1; } //Diabetic
        }
        println!("Income: {}, Food Security: {} | n = {} | High BP: {:.1}%, High Cholesterol: {:.1}%, Diabetic: {:.1}%", //print the results
            income,
            food_security,
            count,
            (high_bp as f32 / count) * 100.0,
            (high_cholesterol as f32 / count) * 100.0,
            (diabetic as f32 / count) * 100.0
        );
    }
}