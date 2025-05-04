// this module defines the similarity function to check if two PersonNode instances are similar

use crate::graph::{PersonNode, ActivityLevel};

/// Check if two PersonNode instances are similar based on various health and lifestyle attributes.
/// Inputs two PersonNode structs
/// Returns true if they are similar, false otherwise.
/// The similarity is determined by comparing their weight state, activity level, life satisfaction, health state, income, food security, and health conditions.

pub fn is_similar(p1: &PersonNode, p2: &PersonNode) -> bool {
    same_weight(p1, p2) &&
    same_activity_level(p1, p2) &&
    close_life_satisfaction(p1, p2) &&
    close_health_state(p1, p2) &&
    same_income(p1, p2) &&
    same_food_security(p1, p2) &&
    same_health_conditions(p1, p2)
}

//returns true if weight state is the same and not unknown
fn same_weight(p1: &PersonNode, p2: &PersonNode) -> bool {
    p1.weight_state == p2.weight_state && p1.weight_state != 9
}
//returns true if activity level is the same and not unknown
fn same_activity_level(p1: &PersonNode, p2: &PersonNode) -> bool {
    p1.activity_level == p2.activity_level && p1.activity_level != ActivityLevel::Unknown
}
fn close_life_satisfaction(p1: &PersonNode, p2: &PersonNode) -> bool {
    // Check if life satisfaction is within 1 point of each other and not greater than 10
    let diff = (p1.life_satisfaction as i8 - p2.life_satisfaction as i8).abs(); // calculate the difference
    diff <= 1 && p1.life_satisfaction <= 10 && p2.life_satisfaction <= 10
}
//returns true if health state is within 1 point of each other and not greater than 5
fn close_health_state(p1: &PersonNode, p2: &PersonNode) -> bool {
    let diff = (p1.gen_health_state as i8 - p2.gen_health_state as i8).abs(); // calculate the difference
    diff <= 1 && p1.gen_health_state <= 5 && p2.gen_health_state <= 5
}
//returns true if income is the same and not 9
fn same_income(p1: &PersonNode, p2: &PersonNode) -> bool {
    p1.total_income == p2.total_income && p1.total_income != 9
}
//returns true if food security is the same and not 9
fn same_food_security(p1: &PersonNode, p2: &PersonNode) -> bool {
    p1.food_security == p2.food_security && p1.food_security != 9
}
//returns true if at least one health condition matches
fn same_health_conditions(p1: &PersonNode, p2: &PersonNode) -> bool {
    // at least one health condition must match
    (p1.high_bp == p2.high_bp) ||
    (p1.high_cholesterol == p2.high_cholesterol) ||
    (p1.diabetic == p2.diabetic)
}