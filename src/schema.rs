table! {
    category (category_id) {
        category_id -> Unsigned<Bigint>,
    }
}

table! {
    category_translation (category_id, language_name) {
        category_id -> Unsigned<Bigint>,
        category_name -> Varchar,
        language_name -> Varchar,
    }
}

table! {
    chef (chef_name) {
        chef_id -> Unsigned<Bigint>,
        chef_name -> Varchar,
    }
}

table! {
    details (dish_id, language_name) {
        dish_id -> Unsigned<Bigint>,
        language_name -> Varchar,
        dish_name -> Varchar,
        dish_instruction -> Nullable<Text>,
    }
}

table! {
    dish (dish_id) {
        dish_id -> Unsigned<Bigint>,
        dish_link -> Nullable<Varchar>,
    }
}

table! {
    dish_category (dish_id, category_id) {
        dish_id -> Unsigned<Bigint>,
        category_id -> Unsigned<Bigint>,
    }
}

table! {
    dish_chef (dish_id, chef_id) {
        dish_id -> Unsigned<Bigint>,
        chef_id -> Unsigned<Bigint>,
    }
}

table! {
    dish_ingredient (dish_id, ingredient_id) {
        dish_id -> Unsigned<Bigint>,
        ingredient_id -> Unsigned<Bigint>,
        ingredient_amount -> Unsigned<Bigint>,
        ingredient_unit -> Varchar,
    }
}

table! {
    event (event_id) {
        event_id -> Unsigned<Bigint>,
        event_name -> Varchar,
        event_date -> Datetime,
        location_id -> Unsigned<Bigint>,
    }
}

table! {
    ingredient (ingredient_id) {
        ingredient_id -> Unsigned<Bigint>,
    }
}

table! {
    ingredient_translation (ingredient_id, language_name) {
        ingredient_id -> Unsigned<Bigint>,
        ingredient_name -> Varchar,
        language_name -> Varchar,
    }
}

table! {
    language (language_name) {
        language_name -> Varchar,
    }
}

table! {
    location (location_id) {
        location_id -> Unsigned<Bigint>,
        location_name -> Varchar,
        location_address -> Varchar,
    }
}

joinable!(category_translation -> category (category_id));
joinable!(category_translation -> language (language_name));
joinable!(details -> dish (dish_id));
joinable!(details -> language (language_name));
joinable!(dish_category -> category (category_id));
joinable!(dish_category -> dish (dish_id));
joinable!(dish_chef -> dish (dish_id));
joinable!(dish_ingredient -> dish (dish_id));
joinable!(dish_ingredient -> ingredient (ingredient_id));
joinable!(event -> location (location_id));
joinable!(ingredient_translation -> ingredient (ingredient_id));
joinable!(ingredient_translation -> language (language_name));

allow_tables_to_appear_in_same_query!(
    category,
    category_translation,
    chef,
    details,
    dish,
    dish_category,
    dish_chef,
    dish_ingredient,
    event,
    ingredient,
    ingredient_translation,
    language,
    location,
);
