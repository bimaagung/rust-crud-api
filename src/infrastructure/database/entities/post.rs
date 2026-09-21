use crate::domain::post::Post;
use sea_orm::entity::prelude::*;

// DB model — Go equivalent: type PostModel struct { gorm.Model; Title string; Body string }
// This is SeaORM's representation of the `posts` table. Separate from domain Post.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)] // Go: gorm:"primaryKey"
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {} // No relations defined yet

impl ActiveModelBehavior for ActiveModel {} // Required by SeaORM (lifecycle hooks)

// Mapper from DB model → domain entity — Go equivalent: func toDomainPost(m Model) domain.Post { ... }
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
