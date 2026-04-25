pub mod post;
pub mod user;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Id {
    pub id: mongodb::bson::Uuid,
}
