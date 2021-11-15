//! SeaORM Entity. Generated by sea-orm-codegen 0.3.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    #[sea_orm(unique)]
    pub nickname: String,
    pub avatar: Option<String>,
    pub reputation: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::blog::Entity")]
    Blog,
    #[sea_orm(has_one = "super::auths::Entity")]
    Auths,
}

impl Related<super::blog::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Blog.def()
    }
}

impl Related<super::auths::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Auths.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
