use prisma_client_rust::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    id: i32,
    created_at: DateTime<Utc>,
    email: String,
    password: String,
    firstname: String,
    lastname: String,
    age: i32,
    gender: Gender,
    biography: Option<String>,
    location_id: Option<i32>,
    location: Option<Location>,
    employer: Option<String>,
    school_grade: Option<String>,
    dietary_prefs: Option<String>,
    company_role: Option<String>,
    role: Role,
    likes: Vec<Like>,
    liked_by: Vec<Like>,
    dislikes: Vec<Dislike>,
    disliked_by: Vec<Dislike>,
    sent_matches: Vec<Match>,
    received_matches: Vec<Match>,
    gallery: Vec<Gallery>,
}

#[derive(Deserialize, Serialize)]
pub struct Like {
    id: i32,
    created_at: DateTime<Utc>,
    from_user_id: i32,
    to_user_id: i32,
}
#[derive(Deserialize, Serialize)]
pub struct Dislike {
    id: i32,
    created_at: DateTime<Utc>,
    from_user_id: i32,
    to_user_id: i32,
}
#[derive(Deserialize, Serialize)]
pub struct Match {
    id: i32,
    created_at: DateTime<Utc>,
    from_user_id: i32,
    to_user_id: i32,
}
#[derive(Deserialize, Serialize)]
pub struct Location {
    id: i32,
    latitude: Option<f64>,
    longitude: Option<f64>,
}
#[derive(Deserialize, Serialize)]
pub struct Gallery {
    id: i32,
    created_at: DateTime<Utc>,
    url: String,
    user_id: i32,
}
#[derive(Deserialize, Serialize)]
pub enum Role {
    USER,
    PLUS,
    LOVE,
    PRO,
    PREMIUM,
    GOLD,
    ADMIN,
}
#[derive(Deserialize, Serialize)]
pub enum Gender {
    Male,
    Female,
    Bisexual,
    Gay,
    Lesbian,
    Pansexual,
    Transgender,
    NonBinary,
    Intersex,
    Queer,
    Other,
}

impl User {
    pub fn find(id: i32) -> Option<User> {
        // ...
    }
    pub fn create() -> Option<User> {
        // ...
    }

    pub fn update() -> Option<User> {
        // ...
    }

    pub fn delete() -> Option<User> {
        // ...
    }
}
