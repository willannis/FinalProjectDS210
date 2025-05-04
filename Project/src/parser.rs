//this module defines the parser for the CSV file containing health data

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use csv::ReaderBuilder;
use crate::graph::{PersonNode, ActivityLevel};

/// Inputs a CSV file and parses it into a vector of PersonNode.
/// Returns a vector of PersonNode or an error if the file cannot be read.
///Uses the csv crate to read the CSV file.

pub fn load_people(path: &str) -> Result<Vec<PersonNode>, Box<dyn Error>> {
    let file = File::open(path)?; //open the file
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(file)); //create a buffered reader(with headers)

    let headers = rdr.headers()?.clone();  //get the headers of the CSV file
    let mut people = Vec::new(); //create a vector to store the PersonNode instances

    for (idx, result) in rdr.records().enumerate() { //iterate over the records
        let record = result?;
        //attempts to parse the CSV record into a PersonNode
        let get = |field: &str| -> &str { //get the value of the field
            match headers.iter().position(|h| h == field) { //find the index of the field
                Some(index) => record.get(index).unwrap_or(""), //get the value of the field
                None => "", //if the field is not found, return an empty string
            }
        };

        let weight_state: u8 = get("Weight_state").parse().unwrap_or(9); //BMI category
        let total_physical_act_time: u16 = get("Total_physical_act_time").parse().unwrap_or(60000); //Physical activity time
        let life_satisfaction: u8 = get("Life_satisfaction").parse().unwrap_or(99); //Life satisfaction(1-10)
        let gen_health_state: u8 = get("Gen_health_state").parse().unwrap_or(9); //General health state(1-5)
        let total_income: u8 = get("Total_income").parse().unwrap_or(9); //Total income(1-9)
        let food_security: u8 = get("Food_security").parse().unwrap_or(9); //Food security(1-9)
        let high_bp: bool = get("High_BP") == "1"; //High blood pressure(1=yes, 0=no)
        let high_cholesterol: bool = get("High_cholestrol") == "1"; //High cholesterol(1=yes, 0=no)
        let diabetic: bool = get("Diabetic") == "1"; //Diabetic(1=yes, 0=no)
        // Determine activity level(low, medium, or high) based on total physical activity time
        let activity_level = match total_physical_act_time {
            0..=149 => ActivityLevel::Low, 
            150..=299 => ActivityLevel::Medium,
            300..=u16::MAX => ActivityLevel::High,
        };
        // Create a new PersonNode with the parsed data
        people.push(PersonNode {
            id: idx,
            weight_state,
            activity_level,
            life_satisfaction,
            gen_health_state,
            total_income,
            food_security,
            high_bp,
            high_cholesterol,
            diabetic,
        });
    }

    Ok(people)
}