//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "rss_feed")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub description: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub display_description: bool,
    pub abbreviation: String,
    pub color: String,
    pub next_update: Option<String>,
    pub update_interval_mins: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::article::Entity")]
    Article,
}

impl Related<super::article::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Article.def()
    }
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        super::rss_feed_category::Relation::Category.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::rss_feed_category::Relation::RssFeed.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
