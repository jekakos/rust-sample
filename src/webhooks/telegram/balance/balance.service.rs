use axum::{
    http::StatusCode,
    response::IntoResponse,
    response::Response,
    Error,
};
use core::result::Result;
use super::{balance_repo::BalanceRepositoryI, package_repo::PackageRepositoryI, telegram_service::TelegramServiceI};
use std::cmp::PartialOrd;

pub async fn send_telegram_notification_balance(
  balance_repo: &dyn BalanceRepositoryI,
  tg_service: &dyn TelegramServiceI,
  package_repo: &dyn PackageRepositoryI,
  imsi: &str, 
  uuid: &str,
  balance: &str
) -> Result<Response, Error> {
  
  println!("telegram_notification_balance -> IMSI {} , balance: {}", imsi, balance);

  let balance_type: &str;
  let mut balance_euro = 0.0;
  let mut balance_bytes:u128 = 0;

  if balance.contains(".") {
    balance_euro = balance.to_owned().parse().unwrap();
    balance_type = "euro";
  } else {
    balance_bytes = balance.to_owned().parse().unwrap();
    balance_type = "bytes";
  }


  let last_balance_in_db: Option<String> = balance_repo.find_penultimate_balance(imsi).await.unwrap_or(None);
  // Below we assume that the balance in the database must be of the same type 
  // as the balance of the balance parameter (u128/f64)

  let mut borders_bytes: Vec<(u128,String)> =
    vec![
      ( 200*1024*1024, "fix".to_owned() ),
      ( 1024*1024*1024, "fix".to_owned() )
    ];

  let borders_euro: Vec<(f64,String)> = vec![
    (5.0, "fix".to_owned()), 
    (10.0, "fix".to_owned())
  ];
  let borders_bytes_percents: Vec<u8> = [10].to_vec(); // 10%

  let package = package_repo.get_package(uuid).await;

  // Adding Bytes percent borders
  if 
    package["package_gb_value"].is_null() ||
    package["package_gb_value"] == ""
  {
  } else {
    println!("Package value found = {}", package["package_gb_value"]);
    
    let package_gb_value_str = package["package_gb_value"].to_string().replace("\"", "");
    let package_gb_value = package_gb_value_str.parse::<f32>().unwrap();
    let package_bytes = (package_gb_value * 1024.0 * 1024.0 * 1024.0) as u128;

    for &border_percent in &borders_bytes_percents {
      let formatted_percent = format!("{}%", border_percent);
      //let clone_formatted_percent = formatted_percent.clone();

      let border = (((( package_bytes * border_percent as u128) ) / 100) as f64).round() as u128;
      borders_bytes.push(( border, formatted_percent ));
    }
  }
  println!("All borders = {:?}", borders_bytes);
  
  match last_balance_in_db {
    Some(last_balance) => {
      println!("Last balance ({}) = {}", imsi, last_balance);

      match balance_type {
          "euro" => {
            println!("Calculate Euro");
            let last_balance_euro: f64 = last_balance.to_owned().parse().unwrap();
            let current_balance_euro = balance_euro;
            
            println!("Last Balance {}, current balance {}", last_balance_euro, current_balance_euro);

            check_balance_and_send::<f64>(
              imsi, 
              &last_balance_euro, 
              &current_balance_euro, 
              &balance_type, 
              &borders_euro, 
              tg_service
            ).await;
          },

          "bytes" => {
            println!("Calculate Bytes");
            let last_balance_bytes: u128 = last_balance.to_owned().parse().unwrap();
            let current_balance_bytes = balance_bytes;
            
            println!("Last Balance {}, current balance {}", last_balance_bytes, current_balance_bytes);

            check_balance_and_send::<u128>(
              imsi, 
              &last_balance_bytes, 
              &current_balance_bytes, 
              &balance_type, 
              &borders_bytes, 
              tg_service
            ).await;
          }
          _ => {}
      }

    },
    None => {
      return Ok((StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response())
    }
  }

  return Ok((StatusCode::OK, "Ok").into_response());
   
}

async fn check_balance_and_send<T>(
  imsi: &str, 
  last_balance: &T, 
  current_balance: &T, 
  balance_type: &str,
  borders: &Vec<(T, String)>,
  tg_service: &dyn TelegramServiceI
) where
  T: PartialOrd + std::fmt::Display + Clone 
{
  for  (border_value, border_label) in borders {
    if last_balance >= border_value && current_balance < border_value {
      tg_service.send_balance(imsi, &border_value.to_string(), &border_label, balance_type).await;
      break;
    }
  }
}