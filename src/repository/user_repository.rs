use crate::db::establish_connection;
use crate::models::user::{NewUser, UpdateUser, Users};
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;

pub fn create_user(new_user: &NewUser) -> Result<Users, Error> {
    let connection = &mut establish_connection();

    diesel::insert_into(users)
        .values(new_user)
        .get_result(connection)
}

pub fn read_user(user_id: i32) -> Result<Users, Error> {
    let connection = &mut establish_connection();

    users.find(user_id).get_result(connection)
}

pub fn search_all_users() -> Result<Vec<Users>, Error> {
    let connection = &mut establish_connection();

    users.load(connection)
}

pub fn update_user(user_id: i32, updated_user: &UpdateUser) -> Result<Users, Error> {
    let connection = &mut establish_connection();

    diesel::update(users.find(user_id))
        .set(updated_user)
        .get_result(connection)
}

pub fn delete_user(user_id: i32) -> Result<usize, Error> {
    let connection = &mut establish_connection();

    diesel::delete(users.find(user_id)).execute(connection)
}
