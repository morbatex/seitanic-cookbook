use mongodb::coll::options::IndexModel;

#[derive(Clone, Debug, Deserialize, Model, Serialize)]
pub struct Dish {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    id : Option<mongodb::oid::ObjectId>,
    name: String,
    chefs: Vec<Chef>,
    #[serde(rename="ingredients")]
    unnamed_ingredients: Option<Vec<Ingredient>>,
    named_ingredients: Option<Vec<NamedIngredient>>,
    instruction: String,
}

impl Dish {
    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    pub fn set_id(&mut self, oid: mongodb::oid::ObjectId) {
        self.id = Some(oid);
    }

    fn uses_ingredient(&self, ingredient: &Ingredient) -> bool {
        self.unnamed_ingredients.as_ref().map_or(false, |ingredients| ingredients.contains(ingredient)) ||
            self.named_ingredients.as_ref().map_or(false, |named_ingredients| named_ingredients.iter().any(|named_ingredient| named_ingredient.ingredients.contains(ingredient)))
    }

    pub fn uses_all_ingredients(&self, ingredients: &[Ingredient]) -> bool {
        ingredients.iter().all(|ingredient| self.uses_ingredient(ingredient))
    }

    pub fn uses_non_ingredients(&self, ingredients: &[Ingredient]) -> bool {
        !ingredients.iter().any(|ingredient| self.uses_ingredient(ingredient))
    }

    pub fn was_cooked_by(&self, chef: &Chef) -> bool {
        self.chefs.contains(chef)
    }

    pub fn name_contains(&self, name: &str) -> bool {
        caseless::default_case_fold_str(&self.name).contains(&caseless::default_case_fold_str(name))
    }
}

impl From<mongodb::oid::ObjectId> for Dish {
    

    fn from(oid: mongodb::oid::ObjectId) -> Self {
        Self{id: Some(oid), name: String::from(""), chefs: Vec::new(), unnamed_ingredients: None, named_ingredients: None, instruction: String::from("")}
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Serialize)]
pub struct Chef {
    name: String,
}

impl From<String> for Chef {

    fn from(name: String) -> Self {
        Self{name}
    }
}

impl PartialEq for Chef {

    fn eq(&self, other: &Self) -> bool {
        caseless::default_caseless_match_str(&self.name, &other.name)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NamedIngredient {
    name: String,
    ingredients: Vec<Ingredient>,
}


#[derive(Clone, Debug, Deserialize, Eq, Serialize)]
pub struct Ingredient {
    name: String,
    amount: String,
    unit: String
}

impl PartialEq for Ingredient {
    
    fn eq(&self, other: &Self) -> bool {
        caseless::default_caseless_match_str(&self.name, &other.name)
    }
}

impl From<String> for Ingredient {
    
    fn from(name: String) -> Self {
        Self{name, amount: "".into(), unit: "".into()}
    }
}
