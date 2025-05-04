//This module defines the graph structure and its methods for managing the PersonNodes and edges(relationships).

use std::collections::{HashMap, HashSet};

//represents levels of activity
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivityLevel {
    Low,
    Medium,
    High,
    Unknown,
}
///represents a person in the health graph and their health and lifestyle attributes
#[derive(Debug, Clone)]
pub struct PersonNode {
    pub id: usize,
    pub weight_state: u8,
    pub activity_level: ActivityLevel,
    pub life_satisfaction: u8,
    pub gen_health_state: u8,
    pub total_income: u8,
    pub food_security: u8,
    pub high_bp: bool,
    pub high_cholesterol: bool,
    pub diabetic: bool,
}
//undirected graph representing the similarities between people
pub struct HealthGraph {
    pub nodes: HashMap<usize, PersonNode>, // maps node id to PersonNode
    pub edges: HashMap<usize, HashSet<usize>>, //adjacency list
}

impl HealthGraph {
    /// Creates a new HealthGraph instance.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }
    //adds a new person node to the graph
    pub fn add_node(&mut self, person: PersonNode) {
        let id = person.id;
        self.nodes.insert(id, person); //insert the person into the graph
        self.edges.insert(id, HashSet::new()); //initialize the adjacency list for the new node
    }
    //adds an edge between two nodes in the graph
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if let Some(neighbors1) = self.edges.get_mut(&from) { //get the neighbors of the first node
            neighbors1.insert(to); //insert the second node into the adjacency list of the first node
        }
        if let Some(neighbors2) = self.edges.get_mut(&to) { //get the neighbors of the second node
            neighbors2.insert(from); //insert the first node into the adjacency list of the second node
        }
    }
    //gets the neighbors of a node and returns them
    pub fn neighbors(&self, id: usize) -> Option<&HashSet<usize>> {
        self.edges.get(&id)
    }
    //computes the degree/# of neighbors of a node and returns it
    pub fn degree(&self, id: usize) -> usize {
        self.edges.get(&id).map_or(0, |neighbors| neighbors.len()) //returns the number of neighbors of the node
    }
    //computes the total number of edges in the graph and returns it
    pub fn total_edges(&self) -> usize {
        self.edges.values().map(|s| s.len()).sum::<usize>() / 2 //divided by 2 because the graph is undirected
    }
}