pub use super::_entities::contacts::{self, ActiveModel, Column, Entity, Model};
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}

impl Model {
    /// Check if contact user exists by email
    ///
    /// # Errors
    /// When could has query/connectivity error
    pub async fn exists(db: &DatabaseConnection, email: &str) -> ModelResult<bool> {
        let count = contacts::Entity::find()
            .filter(
                model::query::condition()
                    .eq(contacts::Column::Email, email)
                    .build(),
            )
            .count(db)
            .await?;
        Ok(count > 0)
    }
}
