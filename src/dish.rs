use mongodb::coll::options::IndexModel;

#[derive(Clone, Debug, Default, Deserialize, Model, Serialize)]
pub struct Dish {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    id : Option<mongodb::oid::ObjectId>,
    name: String,
    chefs: Vec<Chef>,
    #[serde(rename="ingredients", default="Vec::new")]
    unnamed_ingredients: Vec<Ingredient>,
    #[serde(rename="namedIngredients", default="Vec::new")]
    named_ingredients: Vec<NamedIngredient>,
    instruction: String,
    #[serde(default="Vec::new")]
    tags: Vec<String>,
    #[serde(default="Vec::new")]
    images: Vec<String>,
}

impl Dish {
    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    pub fn set_id(&mut self, oid: mongodb::oid::ObjectId) {
        self.id = Some(oid);
    }

    fn uses_ingredient(&self, ingredient: &Ingredient) -> bool {
        self.unnamed_ingredients.iter().any(|ing| ing.name_contains(ingredient)) || self.named_ingredients.iter().any(|unnamed_ingredient| unnamed_ingredient.ingredients.iter().any(|ing| ing.name_contains(ingredient)))
    }

    pub fn uses_all_ingredients(&self, ingredients: &[Ingredient]) -> bool {
        ingredients.iter().all(|ingredient| self.uses_ingredient(ingredient))
    }

    pub fn uses_non_ingredients(&self, ingredients: &[Ingredient]) -> bool {
        !ingredients.iter().any(|ingredient| self.uses_ingredient(ingredient))
    }

    pub fn was_cooked_by(&self, chef: &Chef) -> bool {
        self.chefs.iter().any(|ch| ch.name_contains(chef))
    }

    pub fn name_contains(&self, name: &str) -> bool {
        caseless::default_case_fold_str(&self.name).contains(&caseless::default_case_fold_str(name))
    }
    
    pub fn contains_all_tags(&self, tags: &[String]) -> bool {
        tags.iter().all(|tag| self.tags.contains(tag))
    }
}

impl From<mongodb::oid::ObjectId> for Dish {
    

    fn from(oid: mongodb::oid::ObjectId) -> Self {
        Self{id: Some(oid), ..Default::default()}
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chef {
    name: String,
}

impl Chef {

    fn name_contains(&self, other: &Self) -> bool {
        caseless::default_case_fold_str(&self.name).contains(&caseless::default_case_fold_str(&other.name))
    }
}

impl From<String> for Chef {

    fn from(name: String) -> Self {
        Self{name}
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NamedIngredient {
    name: String,
    ingredients: Vec<Ingredient>,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ingredient {
    name: String,
    amount: String,
    unit: String
}

impl Ingredient {
    
    fn name_contains(&self, other: &Self) -> bool {
        caseless::default_case_fold_str(&self.name).contains(&caseless::default_case_fold_str(&other.name))
    }

}

impl From<String> for Ingredient {
    
    fn from(name: String) -> Self {
        Self{name, amount: "".into(), unit: "".into()}
    }
}
