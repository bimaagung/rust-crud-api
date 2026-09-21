use crate::domain::entities::Post;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for Post {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            body: model.body,
            published: model.published,
        }
    }
}
