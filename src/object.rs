use crate::table::user;

#[derive(Queryable, Eq, Debug, PartialEq, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Eq, Debug, PartialEq)]
#[table_name = "user"]
pub struct UserForInsert {
    pub name: String,
}

#[derive(AsChangeset, Eq, Debug, PartialEq)]
#[table_name = "user"]
pub struct UserForUpdate {
    pub name: String,
}