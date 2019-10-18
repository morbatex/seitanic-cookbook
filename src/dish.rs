use mongodb::coll::options::IndexModel;

#[derive(Clone, Debug, Deserialize, Model, Serialize)]
pub struct Dish {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    id : Option<mongodb::oid::ObjectId>,
    name: String,
    chefs: Vec<Chef>,
    ingredients: Vec<Ingredient>,
    instruction: String,
}

impl Dish {
    fn uses_ingredient(&self, ingredient: &Ingredient) -> bool {
        self.ingredients.contains(ingredient)
    }

    pub fn uses_all_ingredients(&self, ingredients: &[Ingredient]) -> bool {
        ingredients.iter().all(|ingredient| self.uses_ingredient(ingredient))
    }

    pub fn uses_non_ingredients(&self, ingredients: &[Ingredient]) -> bool {
        !ingredients.iter().any(|ingredient| self.uses_ingredient(ingredient))
    }

    pub fn was_cooked_by(&self, chef: &Chef) -> bool{
        self.chefs.contains(chef)
    }

    pub fn name_contains(&self, name: &str) -> bool {
        self.name.contains(name)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Chef {
    name: String,
}

impl From<String> for Chef {

    fn from(name: String) -> Self {
        Self{name}
    }
}



#[derive(Clone, Debug, Deserialize, Eq, Serialize)]
pub struct Ingredient {
    name: String,
    amount: i64,
    unit: String
}

impl PartialEq for Ingredient {
    
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl From<String> for Ingredient {
    
    fn from(name: String) -> Self {
        Self{name, amount: 0, unit: "".into()}
    }
}
