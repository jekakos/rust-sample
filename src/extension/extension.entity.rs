use sea_orm::entity::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "extension")]
pub struct Model {
    #[sea_orm(primary_key)]
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
    pub add_timestamp: Option<DateTime>,
    pub balance_updated_timestamp: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /*
     #[sea_orm(
         belongs_to = "crate::package::package_entity::Entity",
         from = "Column::PackageId",
         to = "crate::package::package_entity::Column::Slug",
         on_update = "Cascade",
         on_delete = "Cascade"
     )]
     Package,
    */
    #[sea_orm(
        belongs_to = "entity::user_iccid::Entity",
        from = "Column::Iccid",
        to = "entity::user_iccid::Column::VIccid",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    UserIccid,
}

/*
impl Related<crate::package::package_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Package.def()
    }
}
*/

impl Related<entity::user_iccid::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserIccid.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
