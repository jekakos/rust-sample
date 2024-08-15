use axum::async_trait;
use mockall::automock;
use reqwest::Client;
use serde_json::Value;

#[async_trait]
#[automock]
pub trait PackageRepositoryI: Sync {
  async fn get_package(&self, uuid: &str) -> Value;
}

pub struct PackageRepository {
  client: Client,
  service_url: String,
}

impl PackageRepository {
  pub fn new(client: &Client) -> Self {
        PackageRepository { 
          service_url: "http://lightsim-billing-ls.default.svc.cluster.local".to_owned(),
          client: client.to_owned() 
        }
    }
}

#[async_trait]
impl PackageRepositoryI for PackageRepository {
  async fn get_package(&self, uuid: &str) -> Value {

    let body = self.client
      .get(self.service_url.to_owned() + "/get-packet?user=" + uuid)
      .send()
      .await
      .unwrap();
    let json_body: Value = body.json().await.unwrap();
    json_body[0].clone() // Assume that user have only one active package
  }
}