use std::fs::File;

use serde::{Deserialize, Serialize};

mod tests;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Yak {
    name: String,
    age: u32,
    sex: char,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Herd {
    pub yak: Vec<Yak>,
}

impl Yak {
    pub fn new(name: String, age: u32, sex: char) -> Self {
        Yak { name, age, sex }
    }
    pub fn is_alive(&self) -> bool {
        self.age > 0 && self.age <= 10
    }
    pub fn years_to_days(&self, days: u32) -> u32 {
        (self.age * 100) + days
    }
    pub fn days_to_years(&self, days: u32) -> u32 {
        self.years_to_days(days) / 100
    }
    pub fn calculate_milk_production(&self, day: u32) -> f32 {
        50.0 - (self.years_to_days(day) as f32 * 0.03)
    }
    pub fn can_be_shaved(&self, day: u32) -> bool {
        if self.age < 1 {
            return false;
        }
        let shaving = day as f32 % (8.0 + self.years_to_days(day) as f32 * 0.01);
        shaving.floor() == 0.0
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn age(&self) -> u32 {
        self.age
    }
    pub fn sex(&self) -> char {
        self.sex
    }
}

pub fn read_xml_content(xml_path: String) -> Herd {
    let xml_content = File::open(xml_path).unwrap();
    let herd: Herd = serde_xml_rs::from_reader(xml_content).unwrap();
    herd
}
