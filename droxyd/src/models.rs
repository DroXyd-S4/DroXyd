use diesel::prelude::*;

#[derive(Queryable, Selectable,Debug,Clone)]
#[diesel(table_name = crate::schema::posts1)]
pub struct Post1 {
    pub id: i32,
    pub url: String,
    pub langue: String,
    pub name: String,
    pub date: String,
    pub word1: String,
    pub word2: String,
    pub word3: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::posts2)]
pub struct Post2 {
    pub i: i32,
    pub key: String,
    pub idofsite: i32,
}


use crate::schema::posts1;

#[derive(Insertable)]
#[diesel(table_name = posts1)]
pub struct NewPost1<'a> {
    pub url: &'a str,
    pub langue: &'a str,
    pub name: &'a str,
    pub date: &'a str,
    pub word1: &'a str,
    pub word2: &'a str,
    pub word3: &'a str,
}

use crate::schema::posts2;

#[derive(Insertable)]
#[diesel(table_name = posts2)]
pub struct NewPost2<'a> {
    pub key: &'a str,
    pub idofsite: &'a i32,
}
