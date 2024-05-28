use diesel::prelude::*;

#[derive(Queryable, Selectable,Debug)]
#[diesel(table_name = crate::schema::posts1)]
pub struct Post1 {
    pub id: i32,
    pub url: String,
    pub langue: String,
    pub name: String,
    pub date: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::posts2)]
pub struct Post2 {
    pub i: i32,
    pub key: String,
    pub idofsite: i32,
}
