use crate::database::Mongo;
use crate::dish::Dish;
use crate::users::{Admin, Login, User};

use chrono::{Duration,Utc};
use mongodb::oid::ObjectId;
use rocket::{ request::FromRequest, Request, http::{Cookie, Cookies }};
use rocket_contrib::json::Json;
use std::convert::TryInto;

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub id: ObjectId,
    valid_until: i64,
}

impl Token {
    
    pub fn is_expired(&self) -> bool {
        self.valid_until < Utc::now().timestamp()
    }
}

impl From<ObjectId> for Token {
    
    fn from(id: ObjectId) -> Self {
        let valid_until = (Utc::now() + Duration::hours(24)).timestamp();
        Token{id, valid_until}
    }
}

pub fn get_routes() -> std::vec::Vec<rocket::Route> {
    routes![get_dish, post_dish, put_dish, delete_dish, post_login, delete_user, get_users, post_user, get_user_type_admin, get_user_type_unknown, get_user_type_user]
}

#[get("/dish?<name>&<ingredients>&<exgredients>&<chef>&<categorie>")]
fn get_dish(name: Option<String>, ingredients: Option<String>, exgredients: Option<String>, chef: Option<String>, categorie: Option<String>, con: Mongo) -> Json<Vec<Dish>>{
    let chef = chef.map(|ok| ok.into());
    let ingredients = ingredients.map_or(Vec::new(), |i| serde_json::from_str(&i).ok().map_or(Vec::new(),|vec: Vec<String>| vec));
    let ingredients: Vec<_> = ingredients.iter().cloned().map(|s| s.into()).collect(); 
    let exgredients = exgredients.map_or(Vec::new(), |i| serde_json::from_str(&i).ok().map_or(Vec::new(),|vec: Vec<String>| vec));
    let exgredients: Vec<_> = exgredients.iter().cloned().map(|s| s.into()).collect();
    let categorie = categorie.and_then(|categorie| if categorie.is_empty() { None } else { categorie.into() });
    Json(crate::database::get_dishes(name, &ingredients, &exgredients, chef, categorie, (*con).to_owned()))
}

#[post("/dish", format = "application/json", data = "<dish>")]
fn post_dish(dish: Json<Dish>, _user: User, con: Mongo) -> Result<rocket::http::Status, rocket::http::Status> {
    let dish = dish.into_inner();
    if dish.has_id() {
        return Err(rocket::http::Status::BadRequest);
    }
    crate::database::insert_dish(dish, (*con).to_owned()).map(|_| rocket::http::Status::Created)
}

#[put("/dish/<id>", format = "application/json", data = "<dish>")]
fn put_dish(id: String, dish: Json<Dish>, _user: User, con: Mongo) -> Result<rocket::http::Status, rocket::http::Status> {
    let oid = mongodb::oid::ObjectId::with_string(&id).map_err(|_| rocket::http::Status::BadRequest)?;
    let mut dish = dish.into_inner();
    dish.set_id(oid);
    crate::database::insert_dish(dish, (*con).to_owned()).map(|_| rocket::http::Status::Ok)
}

#[delete("/dish/<id>")]
fn delete_dish(id: String, _user: User, con: Mongo) -> Result<rocket::http::Status, rocket::http::Status> {
    let dish = mongodb::oid::ObjectId::with_string(&id).map_err(|_| rocket::http::Status::BadRequest)?.into();
    crate::database::delete_dish(dish, (*con).to_owned()).map(|_| rocket::http::Status::Ok)
}

#[get("/user")]
fn get_users(_admin: Admin, con: Mongo) -> Result<Json<Vec<User>>, rocket::http::Status> {
    crate::database::get_all_users((*con).to_owned()).map(|users| Json(users))
}

#[post("/user", format = "application/json", data = "<user>")]
fn post_user(user: Json<Login>, _admin: Admin, con: Mongo) -> Result<rocket::http::Status,rocket::http::Status> {
    let user: User = user.into_inner().try_into()?;
    crate::database::insert_user(user, (*con).to_owned()).map(|_| rocket::http::Status::Created)
}

#[delete("/user/<id>")]
fn delete_user(id: String, con: Mongo) -> Result<rocket::http::Status, rocket::http::Status> {
    let id = serde_json::from_str::<mongodb::oid::ObjectId>(&id).map_err(|_| rocket::http::Status::BadRequest)?;
    crate::database::delete_user(id, (*con).to_owned())
}

#[get("/user/me")]
fn get_user_type_admin(_admin: Admin) -> Json<crate::users::UserType> {
    Json(crate::users::UserType::ADMIN)
}

#[get("/user/me", rank=1)]
fn get_user_type_user(_user: User) -> Json<crate::users::UserType> {
    Json(crate::users::UserType::USER)
}

#[get("/user/me", rank=2)]
fn get_user_type_unknown() -> Json<crate::users::UserType> {
    Json(crate::users::UserType::UNKNOWN)
}


#[post("/login", format = "application/json", data = "<user>")]
fn post_login(user: Json<Login>, mut cookies: Cookies, host: HostHeader, con: Mongo) -> Result<(),rocket::http::Status> {
    let user = user.into_inner();
    let token: Token = crate::database::get_user(user, (*con).to_owned())?.into();
    let token = serde_json::to_string(&token).map_err(|_| rocket::http::Status::InternalServerError)?;
    let cookie = Cookie::build("token", token)
                        .domain(host.0)
                        .http_only(true)
                        .same_site(rocket::http::SameSite::Strict)
                        .secure(true)
                        .max_age(Duration::hours(32))
                        .finish();
    cookies.add_private(cookie);
    Ok(())
}



pub struct HostHeader(pub String);
impl<'a, 'r> FromRequest<'a, 'r> for HostHeader {
    type Error = ();

    fn from_request(request: &'a Request) -> rocket::request::Outcome<Self, Self::Error> {
        match request.headers().get_one("Host") {
            Some(h) => rocket::request::Outcome::Success(HostHeader(h.into())),
            None => rocket::request::Outcome::Forward(()),
        }
    }
}
