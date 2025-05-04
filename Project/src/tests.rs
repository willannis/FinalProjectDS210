//This contains the tests for the graph module to check the functionality of the graph and the similarity function

#[cfg(test)]
mod tests {
    use super::super::graph::{HealthGraph, PersonNode, ActivityLevel};
    use super::super::similarity::is_similar;

//Tests adding nodes and an edge to the graph
//makes sure the connections go both ways and the graph size is correct
    #[test]
    fn test_add_node_and_edge() {
        let mut graph = HealthGraph::new();
        // Create two PersonNode instances
        let person1 = PersonNode {
            id: 0,
            weight_state: 1,
            activity_level: ActivityLevel::High,
            life_satisfaction: 8,
            gen_health_state: 2,
            total_income: 5,
            food_security: 1,
            high_bp: false,
            high_cholesterol: false,
            diabetic: false,
        };
        let person2 = PersonNode {
            id: 1,
            weight_state: 1,
            activity_level: ActivityLevel::Medium,
            life_satisfaction: 8,
            gen_health_state: 2,
            total_income: 5,
            food_security: 1,
            high_bp: true,
            high_cholesterol: false,
            diabetic: false,
        };
        // Add nodes and an edge
        graph.add_node(person1);
        graph.add_node(person2);
        graph.add_edge(0, 1);
        // Check if the nodes and edge were added correctly
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.total_edges(), 1);
        let neighbors0 = graph.neighbors(0).unwrap(); //get the neighbors of node 0
        let neighbors1 = graph.neighbors(1).unwrap(); //get the neighbors of node 1
        assert!(neighbors0.contains(&1)); //check if node 1 is a neighbor of node 0
        assert!(neighbors1.contains(&0)); //check if node 0 is a neighbor of node 1
    }
    //Tests that two similar people are correctly regarded as similar
    #[test]
    fn test_similarity_pos() {
        let person1 = PersonNode{
            id: 0,
            weight_state: 1,
            activity_level: ActivityLevel::Medium,
            life_satisfaction: 8,
            gen_health_state: 2,
            total_income: 5,
            food_security: 1,
            high_bp: false,
            high_cholesterol: false,
            diabetic: false,
        };
        let person2 = PersonNode{
            id: 1,
            weight_state: 1,
            activity_level: ActivityLevel::Medium,
            life_satisfaction: 7,
            gen_health_state: 2,
            total_income: 5,
            food_security: 1,
            high_bp: false,
            high_cholesterol: false,
            diabetic: false,
        };
        assert!(is_similar(&person1, &person2));
    }
    //Tests that two dissimilar people are correctly regarded as dissimilar
    #[test]
    fn test_similarity_neg() {
        let person1 = PersonNode{
            id: 0,
            weight_state: 1,
            activity_level: ActivityLevel::Medium,
            life_satisfaction: 8,
            gen_health_state: 2,
            total_income: 5,
            food_security: 1,
            high_bp: false,
            high_cholesterol: false,
            diabetic: false,
        };
        let person2 = PersonNode{
            id: 1,
            weight_state: 2,
            activity_level: ActivityLevel::High,
            life_satisfaction: 5,
            gen_health_state: 1,
            total_income: 1,
            food_security: 0,
            high_bp: true,
            high_cholesterol: true,
            diabetic: true,
        };
        assert!(!is_similar(&person1, &person2));
    }
}