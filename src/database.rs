use crate::dish::{Chef, Dish, Ingredient};
use crate::users::{Login, User};
use wither::model::Model;

#[database("mongodb")]
pub struct Mongo(rocket_contrib::databases::mongodb::db::Database);

pub fn get_dishes(name: Option<String>, ingredients: &[Ingredient], exgredients: &[Ingredient], chef: Option<Chef>, con: std::sync::Arc<mongodb::db::DatabaseInner>) -> Vec<Dish> {
   
    let dishes: Vec<Dish> = Dish::find(con, None, None).unwrap();

    dishes.iter().filter(|dish| chef.as_ref().map_or(true, |chef| dish.was_cooked_by(chef)))
          .filter(|dish| name.as_ref().map_or(true, |name| dish.name_contains(name)))
          .filter(|dish| dish.uses_all_ingredients(ingredients))
          .filter(|dish| dish.uses_non_ingredients(exgredients))
          .cloned().collect()
}

pub fn insert_dish(mut dish: Dish, con: std::sync::Arc<mongodb::db::DatabaseInner>) -> Result<(),rocket::http::Status> {
    dish.save(con, None).map_err(|_| rocket::http::Status::InternalServerError)
}

pub fn delete_dish(dish: Dish, con: std::sync::Arc<mongodb::db::DatabaseInner>) -> Result<(),rocket::http::Status> {
    dish.delete(con).map_err(|_| rocket::http::Status::InternalServerError)
}

pub fn insert_user(mut user: User, con: std::sync::Arc<mongodb::db::DatabaseInner>) -> Result<(),rocket::http::Status> {
    match get_all_users(con.clone())?.iter().find(|u| u.name == user.name) {
        Some(_) => Err(rocket::http::Status::InternalServerError),
        None => user.save(con, None).map_err(|_| rocket::http::Status::InternalServerError),
    }
}

pub fn delete_user(id: mongodb::oid::ObjectId, con: std::sync::Arc<mongodb::db::DatabaseInner>) -> Result<rocket::http::Status, rocket::http::Status> {
    get_all_users(con.clone())?.iter().find(|user| user.has_id(&id)).ok_or(rocket::http::Status::Ok)?.delete(con).map(|_|rocket::http::Status::Ok).map_err(|_| rocket::http::Status::InternalServerError)   
}

pub fn get_all_users(con: std::sync::Arc<mongodb::db::DatabaseInner>) -> Result<Vec<User>, rocket::http::Status> {
    User::find(con, None, None).map_err(|_| rocket::http::Status::InternalServerError)
}

pub fn get_user(user: Login, con: std::sync::Arc<mongodb::db::DatabaseInner>) -> Result<mongodb::oid::ObjectId ,rocket::http::Status>{
    let users = get_all_users(con)?;
    let u = users.iter().find(|u| user.name == u.name).ok_or(rocket::http::Status::Unauthorized)?;
    if u.algorithm.check(user.password, u.salt.clone(), u.hash.clone()) {
        u.id.clone().ok_or(rocket::http::Status::InternalServerError)
    } else {
        Err(rocket::http::Status::Unauthorized)
    }
}
