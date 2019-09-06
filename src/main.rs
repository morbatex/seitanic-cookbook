#[macro_use]
extern crate diesel;

mod schema;

use diesel::{ExpressionMethods, mysql::MysqlConnection, prelude::*, QueryDsl};

fn main() {
    dotenv::dotenv().ok();
    let con = MysqlConnection::establish(&std::env::var("DATABASE_URL").unwrap()).unwrap();
    get_dishes(QueryDish{title: None, ingredients: vec!["asd".into(),"second".into()], exgredients: Vec::new() ,chefs: Vec::new(), languages: vec!["ENG".into(), "GER".into()], categories: Vec::new(), events: Vec::new()}, con);
}


#[derive(Debug)]
pub struct QueryDish {
    pub title: Option<String>,
    pub ingredients: Vec<String>,
    pub exgredients: Vec<String>,
    pub chefs: Vec<String>,
    pub categories: Vec<String>,
    pub languages: Vec<String>,
    pub events: Vec<u64>,    
}


pub fn get_dishes(dish: QueryDish, con: MysqlConnection) {
    use schema::*;
    let a = ingredient::table.inner_join(ingredient_translation::table)
                      .inner_join(dish_ingredient::table)
                      .filter(ingredient_translation::language_name.eq_any(dish.languages))
                      .filter(ingredient_translation::ingredient_name.eq_any(dish.ingredients))
                      .order_by(dish_ingredient::dish_id.asc())
                      .select((dish_ingredient::dish_id,dish_ingredient::ingredient_id))
                      .distinct()
                      //.select(dish_ingredient::dish_id)
                      .load::<(u64,u64)>(&con)
                      ;
    dbg!(a.unwrap());
}
