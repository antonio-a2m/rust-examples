use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct StudentData {
    student: Student,
}

#[derive(Debug, Deserialize, Serialize)]
struct Student {
    #[serde(rename = "personalInfo")]
    personal_info: PersonalInfo,
}

#[derive(Debug, Deserialize, Serialize)]
struct PersonalInfo {
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    email: String,
    address: Address,
}

#[derive(Debug, Deserialize, Serialize)]
struct Address {
    city: String,
}

#[derive(Debug, Serialize)]
struct ExtractedData {
    first_name: String,
    last_name: String,
    email: String,
    city: String,
}

fn main() {
    // Read the JSON file
    let json_content =
        fs::read_to_string("student_data.json").expect("Failed to read student_data.json");

    // Parse JSON
    let student_data: StudentData =
        serde_json::from_str(&json_content).expect("Failed to parse JSON");

    // Extract the required fields
    let extracted = ExtractedData {
        first_name: student_data.student.personal_info.first_name,
        last_name: student_data.student.personal_info.last_name,
        email: student_data.student.personal_info.email,
        city: student_data.student.personal_info.address.city,
    };

    // Print extracted data
    println!("Extracted Student Information:");
    println!("==============================");
    println!("First Name: {}", extracted.first_name);
    println!("Last Name:  {}", extracted.last_name);
    println!("Email:      {}", extracted.email);
    println!("City:       {}", extracted.city);

    // Optionally, save as JSON
    let json_output =
        serde_json::to_string_pretty(&extracted).expect("Failed to serialize extracted data");

    println!("\nJSON Output:");
    println!("{}", json_output);
}

//------------------- Example JSON (student_data.json) ------------------
