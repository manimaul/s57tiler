use crate::schema::styles;
use serde_json::Value;

#[derive(Queryable, QueryableByName)]
#[table_name = "styles"]
pub struct Style {
    pub id: i64,
    pub name: String,
    pub style: Value,
}

#[derive(Deserialize)]
pub struct PathParam {
    pub name: String,
}
