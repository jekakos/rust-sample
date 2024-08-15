use axum::http::StatusCode;
use lightsim_vendor_interface::dto::extension_dto::*;

use crate::db::get_db_connection;
use crate::redis::get_redis_connection;

use super::dto::extension_dto::GetExtensionsRequestDTO;
use super::extension_service::add_extension as add_extension_service;
use super::extension_service::get_extension as get_extension_service;
use super::extension_service::delete_extension as delete_extension_service;

// Vendor method, doc in helper
pub async fn add_extension(
    request_data: AddExtensionRequestInterfaceDTO,
) -> Result<GetExtensionResponseInterfaceDTO, (StatusCode, String)> {

  let db = get_db_connection();
  
  let result = 
    add_extension_service(
      &db, 
      request_data.iccid.to_owned(), 
      request_data.package_id.to_owned(),
      None
    ).await;

    match result {
      Ok(data) => Ok(data),
      Err(error) => Err(
        (
          StatusCode::INTERNAL_SERVER_ERROR, 
          format!("Error while add extension: {}", error)
        )
      )
    }
}


pub async fn get_extension(
    request_params: GetExtensionRequestInterfaceDTO,
) -> Result<Vec<GetExtensionResponseInterfaceDTO>, (StatusCode, String)> {

  let db = get_db_connection();
  let mut redis = get_redis_connection();

  let params = GetExtensionsRequestDTO { 
    iccid: request_params.iccid.to_owned(), 
    actual_balance: request_params.actual_balance,
    user_uuid: request_params.user_uuid.to_owned(),
    ..Default::default()
  };

  let response = get_extension_service(&db, &mut redis, params).await;

  match response {
    Ok(result) => {
      let mut result_int: Vec<GetExtensionResponseInterfaceDTO> = vec![];

      for extetsion in result {
        let res_item = GetExtensionResponseInterfaceDTO { 
          id:                extetsion.id,
          package_id:        extetsion.package_id, 
          iccid:             extetsion.iccid,
          value_bytes_start: extetsion.n_value_move,
          value_bytes_current: extetsion.n_value,
          date_start:  chrono::DateTime::from_timestamp(extetsion.dt_start as i64, 0).unwrap(),
          date_stop:   
            match extetsion.dt_stop {
              Some(dt) => Some(chrono::DateTime::from_timestamp(dt as i64, 0).unwrap()),
              None => None,
            },
          status:      serde_json::from_str(&extetsion.v_status).unwrap_or(PackageStatus::None),
        };
        result_int.push(res_item);
      }
      
      Ok(result_int)
    }
    Err(error) => Err(
      (
        StatusCode::INTERNAL_SERVER_ERROR, 
        format!("Error: {}", error)
      )
    )
  }
}


pub async fn delete_extension(
    request: DeleteExtensionRequestInterfaceDTO
) -> Result<(StatusCode, String), (StatusCode, String)> {

    let db = get_db_connection();

    let iccid = request.iccid.to_owned();
    let extension_id = request.extension_id.to_owned();

    let response = delete_extension_service(&db, iccid, extension_id).await;

    match response {
      Ok(ok) => Ok((StatusCode::NO_CONTENT, ok)),
      Err(er) => Err((StatusCode::INTERNAL_SERVER_ERROR, er))
    }

}