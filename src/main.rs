#![feature(proc_macro_hygiene, decl_macro, option_flattening)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate mongodb;
#[macro_use] extern crate wither_derive;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod dish;
mod database;
mod routes;
mod users;

use mongodb::ThreadedClient;
use std::convert::TryFrom;

fn main() {
    let catchers = catchers![];

    let rock = rocket::ignite().mount("/",routes::get_routes()).register(catchers).attach(database::Mongo::fairing());
    if let (Some(name), Some(password)) = (std::env::var_os("SEI_USER"), std::env::var_os("SEI_PASS")) {
        if let(Ok(name), Ok(password)) = (name.into_string(), password.into_string()) {
            if let (Ok(mut user), Ok(dbtable)) = (users::User::try_from(users::Login::new(name, password)), rock.config().get_table("databases")) {
                if let Some(rocket::config::Value::Table(mongo)) = dbtable.get("mongodb") {
                    if let Some(rocket::config::Value::String(mongo_url)) = mongo.get("url") {
                        if let Ok(con) = mongodb::Client::with_uri(mongo_url) {
                            let db = con.db("test");
                            user.admin = true;
                            database::insert_user(user, db).ok();
                        }
                    }
                }
            }
        }
    } 
    rock.launch();
}
