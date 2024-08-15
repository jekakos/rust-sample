use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};

use super::dto::extension_dto::{GetExtensionsRequestDTO, GetExtensionsResponseDTO};
use crate::extension::extension_entity;


pub async fn get_extension(
  db: &DatabaseConnection,
  params: GetExtensionsRequestDTO,
) -> Result<Vec<GetExtensionsResponseDTO>, DbErr> {

  let id = params.id.to_owned();
  let iccid = params.iccid.to_owned();
  let id_extension_inst = params.id_extension_inst.to_owned();

  let mut query = extension_entity::Entity::find();

  // Filters
  if let Some(id) = id {
      query = query.filter(extension_entity::Column::Id.eq(id));
  }

  if let Some(iccid) = iccid {
      query = query.filter(extension_entity::Column::Iccid.eq(iccid));
  }

  if let Some(id_extension_inst) = id_extension_inst {
      query = query.filter(extension_entity::Column::IdExtensionInst.eq(id_extension_inst));
  }


  let extensions = query
    .into_model::<GetExtensionsResponseDTO>()
    .all(db)
    .await
    .map_err(|error| {
        println!("Fall while searching for a extensions \n{error}");
        error
    })?;

  println!("packages from db = {:?}", extensions.clone());

  Ok(extensions)
}


//-------------------------------------------
pub async fn check_extension_and_set_in_use(
  db: &DatabaseConnection, id_extension_inst: i32
) -> Result<String, String> {
  
  println!("Chack ext and set in_use {}", id_extension_inst);

  let params = GetExtensionsRequestDTO {
    id_extension_inst: Some(id_extension_inst.to_owned()),
    ..Default::default()
  };

  let extensions = get_extension(db, params).await.unwrap_or_default();
  if extensions.len() == 1 && extensions[0].in_use == false {

    let extension_model = extension_entity::ActiveModel {
        id: Set(extensions[0].id),
        in_use: Set(true),
        ..Default::default()
    };

    extension_model.update(db).await
      .map_err(|error| {
          println!("failed to update extension \n{error}");
          "Error".to_string()
      })
      .unwrap();  
  } else {
    return Err("Error".to_owned());
  }
  Ok("Succes".to_owned())

}


//-------------------------------------------
pub async fn update_extension_balance(
  db: &DatabaseConnection, id_extension_inst: i32, balance: i64
) -> Result<String, String> {

  println!("Update ext balance {}, {}", id_extension_inst, balance);
  let current_time = Utc::now().naive_utc();

  let params = GetExtensionsRequestDTO {
    id_extension_inst: Some(id_extension_inst.to_owned()),
    ..Default::default()
  };

  let extensions = get_extension(db, params).await.unwrap_or_default();
  println!("Found ext for id_ext_inst: {:?}", extensions);

  if 
    extensions.len() == 1 &&
    balance < extensions[0].n_value // impossible to increase balance
  {

    let extension_model = extension_entity::ActiveModel {
        id: Set(extensions[0].id),
        n_value: Set(balance),
        balance_updated_timestamp: Set(Some(current_time)),
        ..Default::default()
    };

    extension_model.update(db).await
      .map_err(|error| {
          println!("failed to update extension \n{error}");
          "Error".to_string()
      })
      .unwrap();  
  }

  Ok("Succes".to_owned())

}

pub async fn delete_extension(
  db: &DatabaseConnection,
  extension_id: i32,
) -> Result<(), DbErr> {

  let _ = extension_entity::Entity::delete_by_id(extension_id)
    .exec(db)
    .await?;

  Ok(())
}