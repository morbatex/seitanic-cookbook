use mongodb::coll::options::IndexModel;
use ring::rand::SecureRandom;
use rocket::request::{FromRequest, Outcome, Request};
use wither::model::Model;
use std::convert::TryFrom;

#[derive(Clone, Deserialize, Model, Serialize)]
pub struct User {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<mongodb::oid::ObjectId>,
    #[model(index(index="asc", unique="true"))]
    pub name: String,
    pub admin: bool,
    pub salt: String,
    pub hash: String,
    pub algorithm: HashAlgo,
}

impl User {

    pub fn has_id(&self, id: &mongodb::oid::ObjectId) -> bool {
        self.id.as_ref().map_or(false, |sid| sid == id)
    }
}

impl TryFrom<Login> for User {
    type Error = rocket::http::Status;
    
    fn try_from(login: Login) -> Result<Self, Self::Error> {
        let entropy = zxcvbn::zxcvbn(&login.password,&[&login.name]).map_err(|_| rocket::http::Status::BadRequest)?;
        if entropy.crack_times_seconds.offline_slow_hashing_1e4_per_second <  chrono::Duration::weeks(5200).num_seconds() as f64 {
            return Err(rocket::http::Status::BadRequest);
        }
        HashAlgo::PBKDF2.hash(&login.password).map(|(salt, hash)| User{ id: None, name: login.name, admin: false, salt, hash, algorithm: HashAlgo::PBKDF2})
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let mut cookies = request.cookies();
        let token = cookies.get_private("token").map(|token| serde_json::from_str::<crate::routes::Token>(token.value()).ok()).flatten();
        let token = match token {
            Some(token) => token,
            None => return Outcome::Forward(()),
        };
        if token.is_expired() {
            return Outcome::Forward(());
        }
        let con = request.guard::<crate::database::Mongo>()?;
        match Self::find((*con).to_owned(), None, None) {
            Ok(users) => {
                let users: Vec<_> = users.iter().filter(|user| user.has_id(&token.id)).collect();
                match users.first() {
                    Some(user) => Outcome::Success((*user).clone()),
                    None => Outcome::Forward(()),
                }
            },
            Err(_) => Outcome::Forward(()),
        }
    }

}

pub struct Admin;

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let user = request.guard::<User>()?;
        if user.admin {
            Outcome::Success(Self)
        } else {
            Outcome::Forward(())
        }
    }
}

#[derive(Serialize)]
pub enum UserType {
    ADMIN,
    USER,
    UNKNOWN,
}


#[derive(Deserialize, Serialize)]
pub struct Login {
    pub name: String,
    pub password: String,
}

impl Login {
    pub fn new(name: String, password: String) -> Self {
        Self{name, password}
    }
}


#[derive(Clone, Deserialize, Serialize)]
pub enum HashAlgo {
    PBKDF2,
}

impl HashAlgo {
    
    pub fn hash<T: Into<String>>(&self, password: T) -> Result<(String, String), rocket::http::Status> {
        let password = password.into();
        match self {
            Self::PBKDF2 => {
                let mut salt: [u8; 120] = [0; 120];
                let mut hash: [u8; 120] = [0; 120];
                ring::rand::SystemRandom::new().fill(&mut salt).map_err(|_| rocket::http::Status::InternalServerError)?;
                ring::pbkdf2::derive(self.get_algo(), self.get_rounds(), &salt, password.as_bytes(), &mut hash);
                Ok((base64::encode(&salt.to_vec()),base64::encode(&hash.to_vec())))
            },
        }
    }

    pub fn check<T: Into<String>>(&self, password: T, salt: T, hash: T) -> bool {
        let password = password.into();
        if let (Ok(salt), Ok(hash)) = (base64::decode(&salt.into()), base64::decode(&hash.into())) {
            match self {
                Self::PBKDF2 => {
                    ring::pbkdf2::verify(self.get_algo(), self.get_rounds(), &salt, password.as_bytes(), &hash).is_ok()
                }
            }
        } else {
            false
        }

    }

    fn get_rounds(&self) -> u32 {
        match self {
            Self::PBKDF2 => 100_000,
        }
    }

    fn get_algo(&self) -> &'static ring::digest::Algorithm {
        match self {
           Self::PBKDF2 => &ring::digest::SHA512,
        }
    }
}
