use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Product{
    pub id: u32,
    pub name: String
}
