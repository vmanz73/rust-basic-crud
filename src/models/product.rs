use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub stock: i32,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProductDTO {
    pub name: String,
    pub stock: i32,
    pub description: Option<String>,
}
