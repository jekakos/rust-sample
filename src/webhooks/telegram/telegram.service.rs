use axum::async_trait;
use mockall::automock;
use reqwest::Client;
use serde_json::json;

#[async_trait]
#[automock]
pub trait TelegramServiceI: Sync {
  async fn send_balance(&self, imsi: &str, balance_less: &str, balance_lable: &str, balance_type: &str);
  async fn send_location(&self, imsi: &str, location: &str);
  async fn send_to_channel(&self, message: &str);
}

pub struct TelegramService {
  client: Client,
  service_url: String,
}

impl TelegramService {
  pub fn new(client: &Client) -> Self {
        TelegramService { 
          service_url: "http://telegram*************:3000".to_owned(),
          client: client.to_owned() 
        }
    }
}

#[async_trait]
impl TelegramServiceI for TelegramService {
  async fn send_balance(
    &self, 
    imsi: &str,
    balance_less: &str, 
    balance_lable: &str,
    balance_type: &str, 
  ) {
    println!("Telegram send balance ({}) = less {}", imsi, balance_less);
    let user_imsi_c = imsi.replace("\"", "");
    let imsi_u: u128 = user_imsi_c.parse().expect("Failed to parse imsi");

    let request_message =
        json!({
            "imsi": imsi_u, 
            "balance_less": balance_less.to_owned(),
            "balance_lable": balance_lable.to_owned(),
            "type": balance_type,
        });

    self.client
        .post(self.service_url.to_owned() + "/nc/balance")
        .json(&request_message)
        .send()
        .await
        .unwrap();

  }

  async fn send_location(
    &self, 
    imsi: &str,
    location: &str,
  ) {
    println!("Telegram send location ({}) = {}", imsi, location);
    let user_imsi_c = imsi.replace("\"", "");
    let imsi_u: u128 = user_imsi_c.parse().expect("Failed to parse imsi");

    let request_message =
        json!({
            "imsi": imsi_u, 
            "location": location.to_owned()
        });

    self.client
        .post(self.service_url.to_owned() + "/nc/location")
        .json(&request_message)
        .send()
        .await
        .unwrap();
  }

  async fn send_to_channel(&self, message: &str) {
    let _ = self.client
      .post("http://telegram.************/send")
      .body(message.to_owned())
      .send()
      .await
      .unwrap();
  }
}