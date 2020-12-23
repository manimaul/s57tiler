use crate::schema::styles;

#[derive(Queryable, QueryableByName)]
#[table_name = "styles"]
pub struct Style {
    pub id: i64,
    pub name: String,
    pub style: String,
}
