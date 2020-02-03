use rocket::data::{Data, FromDataSimple, Outcome};
use multipart::server::{Multipart, save::{Entries ,SaveResult, SavedData}};

use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

pub struct MulitpartData {
    data: Data,
    boundary: String,
}


impl FromDataSimple for MulitpartData {
    type Error = String;

    fn from_data(req: &rocket::Request, data: Data) -> Outcome<Self, Self::Error> {
        match req.content_type() {
            Some(content_type) if content_type.is_form_data() =>{
                match content_type.params().find(|&(k, _) | k == "boundary") {
                    Some((_, boundary)) => {
                        Outcome::Success(MulitpartData{ data, boundary: boundary.into() })
                    },
                    None => Outcome::Forward(data),
                }
            },
            _ => Outcome::Forward(data),
        }
    }
}

impl MulitpartData {
    
    pub fn process(self) -> Result<Vec<String>, rocket::http::Status> {
        match Multipart::with_body(self.data.open(), self.boundary).save().size_limit(10_000_000).temp() {
            SaveResult::Full(entries) => process_entries(entries),
            SaveResult::Partial(partial, _) => {
                if let Some(field) = partial.partial {
                    if let Some(SavedData::File(path,_)) = field.dest {
                        std::fs::remove_file(path).ok();
                    }
                }
                Err(rocket::http::Status::BadRequest)
            },
            SaveResult::Error(_) => Err(rocket::http::Status::BadRequest),
        }
    }
}

fn process_entries(entries: Entries) -> Result<Vec<String>, rocket::http::Status>{
    let mut images = Vec::new();
    let entries = entries.fields.values().flatten().collect::<Vec<_>>();
    for entry in entries {
        if let SavedData::File(data, size) = &entry.data {
            let mime_type = tree_magic::from_filepath(data);
            if mime_type.starts_with("image/") {
                let ending = mime_type.trim_start_matches("image/"); 
                let mut hasher = DefaultHasher::new();
                (data, size).hash(&mut hasher);
                let new_path = format!("images/{}.{}", hasher.finish(), ending);
                std::fs::rename(data, &new_path).map_err(|_| rocket::http::Status::BadRequest)?;
                images.push(new_path);
            }
        }
    }
    Ok(images)
}
