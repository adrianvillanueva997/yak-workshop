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
}
