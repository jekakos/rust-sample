use axum::{
    http::StatusCode,
    response::IntoResponse,
    response::Response,
    Error,
};
use core::result::Result;
use super::{location_repo::LocationRepositoryI, telegram_service::TelegramServiceI};

pub async fn send_telegram_notification_location(
  location_repo: &dyn LocationRepositoryI,
  tg_service: &dyn TelegramServiceI,
  imsi: &str, 
  location: &str
) -> Result<Response, Error> {
  
  println!("telegram_notification_location -> IMSI {} , country: {}", imsi, location);

  let last_location_in_db: Option<String> = location_repo.find_penultimate_location(imsi).await.unwrap_or(None);

  match last_location_in_db {
    Some(last_location) => {
      println!("Last location ({}) = {}", imsi, last_location);

      if last_location != location {
        tg_service.send_location(imsi, &location).await;
      }
    },
    None => {
      return Ok((StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response())
    }
  }
  return Ok((StatusCode::OK, "Ok").into_response());  
}


pub async fn send_support_warning_if_roaming_not_allowed(
  location_repo: &dyn LocationRepositoryI,
  tg_service: &dyn TelegramServiceI,
  imsi: &str, 
  state: &str
) -> Result<Response, Error> {
  
  println!("telegram_alert_roaming_not_allowed -> IMSI {} , state: {}", imsi, state);
  
  if state != "plmnRoamingNotAllowed" {
    return Ok((StatusCode::OK, "Ok").into_response());
  }

  let last_states: Vec<String> = location_repo.find_30min_states(imsi).await.unwrap_or([].to_vec());
  
  println!("Last 30 min states: {:?}", last_states);
  let mut is_send_alert = false;
  let mut count_roaming_errors = 0;

  if last_states.len() > 0 {
    for item_state in last_states {
      // If appear ALLOW then not spend 30 min in RoamingNotAllowed
      if item_state == "ALLOW" {
        is_send_alert = false;
        break;
      }
      // Keep true while item_state == "plmnRoamingNotAllowed"
      if item_state == "plmnRoamingNotAllowed" {
        is_send_alert = true;
        count_roaming_errors += 1;
      }
    }
  }
  let message = format!("WARNING: IMSI {} could not connect to roaming during 30 min", imsi);
  if is_send_alert && count_roaming_errors > 1 {
    tg_service.send_to_channel(&message).await;
  }

  return Ok((StatusCode::OK, "Ok").into_response());  
}
