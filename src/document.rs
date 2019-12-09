use elastic_derive::*;
use serde::{Serialize, Deserialize};

#[derive(ElasticType, Serialize, Deserialize, Debug)]
#[elastic(index = "docs_organizer")]
pub struct Document {
    pub content: String,
    pub directory: String,
}