use axum::async_trait;
use core::result::Result;
use entity::service_web_hook_post::Column;
use entity::service_web_hook_post::Entity as BalanceLog;
use mockall::automock;
use sea_orm::QueryOrder;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};

#[async_trait]
#[automock]
pub trait BalanceRepositoryI: Sync {
    async fn find_penultimate_balance(&self, imsi: &str) -> Result<Option<String>, sea_orm::DbErr>;
    async fn find_last_balance(&self, imsi: &str) -> Result<Option<String>, sea_orm::DbErr>;
}

pub struct BalanceRepository {
    connection: DatabaseConnection,
}

impl BalanceRepository {
    pub fn new(db: &DatabaseConnection) -> Self {
        BalanceRepository {
            connection: db.to_owned(),
        }
    }
}

#[async_trait]
impl BalanceRepositoryI for BalanceRepository {
    async fn find_penultimate_balance(&self, imsi: &str) -> Result<Option<String>, sea_orm::DbErr> {
        BalanceLog::find()
            .filter(Column::Imsi.eq(imsi.to_owned()))
            .select_only()
            .order_by_desc(Column::Id)
            .column(Column::Balance)
            .limit(2)
            .into_tuple()
            .all(&self.connection)
            .await
            .map(|records| records.get(1).cloned()) // penultimate !!!
    }

    async fn find_last_balance(&self, imsi: &str) -> Result<Option<String>, sea_orm::DbErr> {
        BalanceLog::find()
            .filter(Column::Imsi.eq(imsi.to_owned()))
            .select_only()
            .order_by_desc(Column::Id)
            .column(Column::Balance)
            .into_tuple()
            .one(&self.connection)
            .await
    }
}
