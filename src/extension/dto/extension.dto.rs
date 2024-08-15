use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(ToSchema, IntoParams, Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetExtensionsRequestDTO {
    pub id: Option<i32>,
    pub iccid: Option<String>,
    pub user_uuid: Option<String>,
    pub id_extension_inst: Option<i32>,
    pub actual_balance: Option<bool>,
}

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone, FromQueryResult)]
pub struct GetExtensionsResponseDTO {
    pub id: i32,
    pub id_extension_inst: i32, //  Unique ID in Service scope
    pub iccid: String,
    pub package_id: String, // link to package.slug
    pub id_extension: i32,  // hz chto eto - eto ID region x.x
    pub n_value_move: i64,  // Bytes on start
    pub n_value: i64,       // Current value (should be updates by webhook)
    pub dt_start: i32,
    pub dt_stop: Option<i32>,
    pub v_status: String,
    pub in_use: bool,
    pub add_timestamp: Option<NaiveDateTime>,
}
