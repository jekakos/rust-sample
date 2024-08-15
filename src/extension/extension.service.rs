use chrono::{TimeZone, Utc};
use lightsim_vendor_interface::dto::{esim_dto::*, extension_dto::{GetExtensionResponseInterfaceDTO, PackageStatus}};
use redis::{Commands, Connection as RedisConnection};
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, DatabaseConnection, Set};

use crate::{
  esim::esim_repo::get_esim, extension::{extension_entity::ActiveModel as ExtensionActiveModel, extension_repo::update_extension_balance}, 
  package::package_repo::slug_to_params, region::region_repo::get_region_by_code_name, 
  service::{
    dto::extension_dto::{ActiveExtensionServiceResponseDTO, AddExtensionServiceRequestDTO, DeleteExtensionServiceRequestDTO}, 
    extension::{add_extension as add_extension_service, delete_extension as delete_extension_service, get_active_extensions}
  }
};

use super::{
  dto::extension_dto::{GetExtensionsRequestDTO, GetExtensionsResponseDTO}, 
  extension_repo::{delete_extension as delete_extension_repo, get_extension as get_extension_repo}
};


//---------------------------------------------------------------
pub async fn add_extension(
  db: &DatabaseConnection, 
  
  iccid: String, 
  slug: String,
  id_serv_inst: Option<i32>,

) -> Result<GetExtensionResponseInterfaceDTO, String>{

  // Get id_service_inst (for Service request)
  let id_service_inst: i32;
  if let Some(id) = id_serv_inst {
    id_service_inst = id;
  } else  {
    let params = GetEsimRequestInterfaceDTO {
        iccid: Some(iccid.to_owned()),
        user_uuid: None,
    };
    let data = get_esim(db, params).await;
    match data {
      Ok(esims) => {
        if esims.len() == 1 {
          id_service_inst = esims[0].id_service_inst.to_owned();
        } else {
          return Err(format!("Cannot find esim by iccid {}", iccid));
        }
      }
      Err(_) => return Err("Error serching esim".to_owned())
    }
  }

  // Get region code
  let (slug_region_code, slug_days, slug_gb) = slug_to_params(slug.to_owned())
    .map_err(|_| return "Error slug".to_owned())?;


  // Get region bu region_code to get extension_id
  let region = get_region_by_code_name(slug_region_code.to_owned(), &db).await.unwrap();

  // Prepare DTO for Service
  let gb = 1024 * 1024 * 1024;
  let bytes_from_data_gb: i64 = (slug_gb * Decimal::from(gb)).try_into().unwrap();

  let add_request_data = AddExtensionServiceRequestDTO {
      id_service_inst:    id_service_inst.to_owned(),
      value:              bytes_from_data_gb.to_owned(),
      period_count:       slug_days,//package.days.to_owned(),
      id_extension:       region.service_id.to_owned(), // = extension_id
      period_length_type: 0,
  };

  // Send data to Service
  let response = add_extension_service(add_request_data).await;
  match response {
    Ok(service_response) => {

      // Find recently added extension to get all ext data (dt_start, status, etc)
      let service_extensions = 
        get_active_extensions(
          id_service_inst.to_owned(), 
          Some(service_response.id_extension_inst.to_owned())
        ).await.unwrap();
      
      if service_extensions.len() != 1 {
        return Err("Error serching new added  extention".to_owned());
      }

      let added_extension = service_extensions[0].clone();

      
      // Add extension to db
      // Here should be request to repo but more short to make it here
      let db_insert_data = ExtensionActiveModel {
        id:                NotSet,
        id_extension_inst: Set(added_extension.id_extension_inst), // unique
        iccid:             Set(iccid.to_owned()),
        package_id:        Set(slug.to_owned()),

        id_extension:      Set(added_extension.id_extension),
        n_value_move:      Set(added_extension.n_value.to_owned()),
        n_value:           Set(added_extension.n_value.to_owned()),
        dt_start:          Set(added_extension.dt_start.to_owned()),
        dt_stop:           Set(added_extension.dt_stop.to_owned()),
        v_status:          Set(added_extension.v_status.to_owned()),
        in_use:            Set(false),
        add_timestamp:     NotSet,
        balance_updated_timestamp: NotSet,
      };

      println!("SAVING EXTENSION TO DB {:?}", db_insert_data);
      let saved = db_insert_data.save(db).await;
      match saved {
        Ok(result) => {

          let package_status: PackageStatus = serde_json::from_str(&format!("\"{}\"", added_extension.v_status.to_owned()))
                .unwrap_or(PackageStatus::None);
              
          Ok(GetExtensionResponseInterfaceDTO {
            id: result.id.unwrap(),
            package_id: slug.to_owned(),
            iccid: iccid.to_owned(),
            value_bytes_start: added_extension.n_value.to_owned(),
            value_bytes_current: added_extension.n_value.to_owned(),
            date_start: Utc.timestamp_opt(added_extension.dt_start.to_owned().into(), 0).unwrap(),
            date_stop: Some(Utc.timestamp_opt(added_extension.dt_stop.to_owned().unwrap_or(0).into(), 0).unwrap()),
            status: package_status,
          })
        }
        Err(e) => {
          Err(format!("Error while saving data to extension table: {:?}", e))
        }
      }
    }
    Err(_) => {
      Err("Error on Service side".to_owned())
    }
  }
}

//---------------------------------------------------------------
pub async fn get_extension(
  db: &DatabaseConnection, 
  redis: &mut RedisConnection,
  params: GetExtensionsRequestDTO,
) -> Result<Vec<GetExtensionsResponseDTO>, String> {
  
  let result = get_extension_repo(&db, params.clone()).await;
  let mut exts_result: Vec<GetExtensionsResponseDTO>;
  match result {
      Ok(data) => {
        exts_result = data.clone();
      },
      Err(e) => { 
        return Err(e.to_string());
      }
  }
  
  let actual_balance = params.actual_balance.unwrap_or(false);
  let iccid = params.iccid.unwrap_or("".to_owned());

  // if iccid was not set we cannot get actual balance
  if actual_balance && iccid != "" {

    let get_esim_params = GetEsimRequestInterfaceDTO {
        iccid: Some(iccid.to_owned()),
        user_uuid: None,
    };

    let esims = get_esim(&db, get_esim_params).await.unwrap_or(vec!());
    if esims.len() == 1 {
      
      let exts_service_or_redis: Vec<ActiveExtensionServiceResponseDTO>;
      

      // Try get extensions from redis by key "extension-<ICCID>"
      let key = format!("extension-{}", esims[0].v_iccid );
      let exts_redis_json_str: String = redis.get(key.to_owned()).unwrap_or("".to_string());
      println!("Redis search for {}: {}", key, exts_redis_json_str);

      let exts_redis: Vec<ActiveExtensionServiceResponseDTO> = serde_json::from_str(&exts_redis_json_str).unwrap_or_default();


      // Define exts_service_or_redis
      if exts_redis.len() > 0 {
        println!("Found exts in Redis: {:?}", exts_redis);
        exts_service_or_redis = exts_redis;

      } else {
        println!("Go to Service");
        let exts_service = get_active_extensions(
          esims[0].id_service_inst, 
          None
        ).await.unwrap_or_default();
        println!("Found exts on Service: {:?}", exts_service);

        // Set redis value
        let exts_service_str = serde_json::to_string(&exts_service).unwrap_or_default();
        redis.set::<String, String, String>(key.to_owned(), exts_service_str.to_owned()).unwrap();

        let balance_life = dotenv::var("REDIS_BALANCE_TIME")
          .unwrap_or("60".to_owned())
          .parse::<i64>()
          .unwrap();

        redis.expire::<String, i64>(key.to_owned(), balance_life).unwrap();
        println!("Redis was writen for {}: {}", key, exts_service_str);

        exts_service_or_redis = exts_service;
      }

      for ext_service_or_redis in exts_service_or_redis {
        for ext_result in &mut exts_result {
          if 
            ext_result.id_extension_inst == ext_service_or_redis.id_extension_inst &&
            ext_service_or_redis.n_value < ext_result.n_value //impossible to increase balance
          {
            ext_result.n_value = ext_service_or_redis.n_value.to_owned();

            let _= update_extension_balance(
              &db, 
              ext_result.id_extension_inst.to_owned(), 
              ext_service_or_redis.n_value.to_owned()
            ).await;
          }
        }
      }


      return Ok(exts_result);
    }
  }

  Ok(exts_result)
}

//---------------------------------------------------------------
pub async fn delete_extension(
  db: &DatabaseConnection, 
  iccid: String,
  extension_id: i32,
) -> Result<String, String> {

  // Get user_iccid data by iccid ( id_contract_inst, id_service_inst, id_extension_inst)
  let esim_params = GetEsimRequestInterfaceDTO {
    iccid: Some(iccid.to_owned()),
    user_uuid: None,
  };

  let esims = get_esim(&db, esim_params)
    .await
    .unwrap_or_else(|_| vec!());
  
  if esims.len() != 1 { 
    return Err("Not found esim data for iccid".to_string());
  }

  // Get extension's service_id

  let ext_params = GetExtensionsRequestDTO {
    id: Some(extension_id.to_owned()),
    ..Default::default()
  };
  
  let exts = get_extension_repo(&db, ext_params)
    .await
    .unwrap_or_else(|_| vec!());

  if exts.len() != 1 { 
    return Err("Not found extension".to_string());
  }
  
  // Delete on Service side
  let delete_service_data = DeleteExtensionServiceRequestDTO {
    id_contract_inst:  esims[0].id_contract_inst,
    id_service_inst:   esims[0].id_service_inst,
    id_extension_inst: exts[0].id_extension_inst,
  };

  let service_response = delete_extension_service(delete_service_data).await;

  match service_response {
      Ok(_) => {},
      Err(_) => {
        return Err("Error while delete extension on Service side".to_string());
      }
  }

  // Delete from db
  let db_response = delete_extension_repo(&db, extension_id.to_owned()).await;

  match db_response {
      Ok(_) => {},
      Err(_) => {
        return Err("Error while delete extension on Service side".to_string());
      }
  }

  Ok("Success".to_owned())
}