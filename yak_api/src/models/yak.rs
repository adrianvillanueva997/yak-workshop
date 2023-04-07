use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, FromRow, Clone, Debug, ToSchema)]
pub struct Yak {
    id: i32,
    name: String,
    age: f32,
    age_last_shaved: f32,
}

impl Yak {
    pub fn id(&self) -> i32 {
        self.id
    }
    /// Returns the is alive of this [`Yak`].
    pub fn is_alive(&self) -> bool {
        self.age > 0.0 && self.age < 10.0
    }

    /// .
    fn age_to_days(&self, days: f32) -> f32 {
        self.age * 100.0 + days
    }

    /// .
    fn days_to_years(&self, days: f32) -> f32 {
        self.age_to_days(days) / 100.0
    }

    /// .
    fn milk(&self, days: f32) -> f32 {
        50.0 - (self.days_to_years(days) * 0.03)
    }

    /// .
    pub fn production(&mut self, days: f32) -> (f32, f32) {
        let mut total_milk = 0.0;
        let mut total_wool = 0.0;
        let mut current_day = 0.0;
        while self.is_alive() && current_day < days {
            let milk = self.milk(current_day);
            if self.can_be_shaved(current_day) {
                total_wool += 1.0;
            }
            total_milk += milk;
            current_day += 1.0;
        }
        (total_milk, total_wool)
    }

    /// .
    fn can_be_shaved(&mut self, day: f32) -> bool {
        if self.age < 1.0 {
            return false;
        }
        let shaving = day % (8.0 + self.age_to_days(day) * 0.01);
        if shaving.floor() == 0.0 {
            self.set_age_last_shaved(self.days_to_years(day));
            return true;
        }
        return false;
    }

    /// Sets the age last shaved of this [`Yak`].
    fn set_age_last_shaved(&mut self, age_last_shaved: f32) {
        self.age_last_shaved = age_last_shaved;
    }
}
