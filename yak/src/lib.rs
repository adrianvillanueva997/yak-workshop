use serde::{Deserialize, Serialize};

mod tests;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Yak {
    age: f32,
    id: i8,
    name: String,
    age_last_shaved: f32,
}
impl Yak {
    pub fn default(age: f32) -> Self {
        Yak {
            age,
            id: Default::default(),
            name: Default::default(),
            age_last_shaved: Default::default(),
        }
    }
    pub fn is_alive(self) -> bool {
        self.age >= 0.0 && self.age <= 10.0
    }
    pub fn years_to_days(self, days: f32) -> f32 {
        (self.age * 100.0) + days
    }
    pub fn days_to_years(self, days: f32) -> f32 {
        self.years_to_days(days) / 100.0
    }
    pub fn calculate_milk_production(self, day: f32) -> f32 {
        50.0 - (self.years_to_days(day) * 0.03)
    }
    pub fn can_be_shaved(self, day: f32) -> bool {
        if self.age < 1.0 {
            return false;
        }
        let shaving_calculation = day % (8.0 + self.years_to_days(day) * 0.01);
        shaving_calculation.floor() == 0.0
    }
}
