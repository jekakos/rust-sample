use axum::async_trait;
use core::result::Result;
use chrono::{Utc, TimeDelta};
use entity::service_web_hook_get::Column;
use entity::service_web_hook_get::Entity as LocationLog;
use mockall::automock;
use sea_orm::QueryOrder;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};

#[async_trait]
#[automock]
pub trait LocationRepositoryI: Sync {
    async fn find_penultimate_location(&self, imsi: &str) -> Result<Option<String>, sea_orm::DbErr>;
    async fn find_last_location(&self, imsi: &str) -> Result<Option<String>, sea_orm::DbErr>;
    async fn find_30min_states(&self, imsi: &str) -> Result<Vec<String>, sea_orm::DbErr>;
}

pub struct LocationRepository {
    connection: DatabaseConnection,
}

impl LocationRepository {
    pub fn new(db: &DatabaseConnection) -> Self {
        LocationRepository {
            connection: db.to_owned(),
        }
    }
}

#[async_trait]
impl LocationRepositoryI for LocationRepository {
    async fn find_penultimate_location(&self, imsi: &str) -> Result<Option<String>, sea_orm::DbErr> {
        LocationLog::find()
            .filter(Column::Imsi.eq(imsi.to_owned()))
            .select_only()
            .order_by_desc(Column::Id)
            .column(Column::Country)
            .limit(2)
            .into_tuple()
            .all(&self.connection)
            .await
            .map(|records| records.get(1).cloned()) // penultimate !!!
    }

    async fn find_last_location(&self, imsi: &str) -> Result<Option<String>, sea_orm::DbErr> {
        LocationLog::find()
            .filter(Column::Imsi.eq(imsi.to_owned()))
            .select_only()
            .order_by_desc(Column::Id)
            .column(Column::Country)
            .into_tuple()
            .one(&self.connection)
            .await
    }

    async fn find_30min_states(&self, imsi: &str) -> Result<Vec<String>, sea_orm::DbErr> {

        let half_hour_ago = Utc::now() - TimeDelta::try_minutes(30).unwrap();
        LocationLog::find()
            .filter(Column::Imsi.eq(imsi.to_owned()))
            .filter(Column::TimeStamp.gt(half_hour_ago))
            .select_only()
            .order_by_desc(Column::Id)
            .column(Column::State)
            .into_tuple()
            .all(&self.connection)
            .await
    }
}
