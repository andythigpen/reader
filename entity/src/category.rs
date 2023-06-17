//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "category")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::rss_feed::Entity> for Entity {
    fn to() -> RelationDef {
        super::rss_feed_category::Relation::RssFeed.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::rss_feed_category::Relation::Category.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
