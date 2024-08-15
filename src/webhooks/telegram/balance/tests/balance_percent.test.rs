use serde_json::json;

use crate::webhooks::webhook_lib::telegram_mod::{balance_service::send_telegram_notification_balance, package_repo::MockPackageRepositoryI};
use super::{balance_repo::MockBalanceRepositoryI, telegram_service::MockTelegramServiceI};

// Here we're chacking if user's balacne become less then 10% of 1Gb
#[tokio::test]
async fn test_send_telegram_notification_balance_bytes_percent() {
    let mut balance_repo_mock = MockBalanceRepositoryI::new();
    let mut tg_service_mock = MockTelegramServiceI::new();
    let mut package_repo_mock = MockPackageRepositoryI::new();

    let current_balance: u128 = 107_374_182; // 107 mb = 10% of 1Gb
    //let current_balance_str = current_balance.to_string();
    // current balance is less then 10% of package
    let package_value_gb = 1; //1 Gb
    //let package_value_str = package_value.to_string();

    let current_balance_less = (current_balance - 10).to_string();
    let current_balance_more = (current_balance + 10).to_string();
    
    let uuid = "some_uuid".to_owned();

    // Setting up mocks
    balance_repo_mock.expect_find_penultimate_balance()
        .returning(move |_| {
            let border_clone = current_balance_more.clone();
            Box::pin(async move { Ok(Some(border_clone)) })
        });
    package_repo_mock.expect_get_package()
        .returning(move |_| {
            let json_obj = json!({"package_gb_value": package_value_gb});
            Box::pin(async move { json_obj })
        });
    
    let current_balance_clone = current_balance.to_string().clone();
    tg_service_mock.expect_send_balance()
        .withf(
            move |_imsi, balance_less, border_lable, balance_type| 
            * &balance_less == current_balance_clone.to_owned() && 
            * &border_lable == "10%" && 
            * &balance_type == "bytes")
        .times(1) // Run only one time
        .returning(|_, _, _, _| Box::pin(async move {}));

    // Run testing method
    let result = send_telegram_notification_balance(
        &balance_repo_mock, 
        &tg_service_mock, 
        &package_repo_mock,
        "some_imsi", 
        &uuid, 
        &current_balance_less // Balance is less then 200 mb
    ).await;

    assert!(result.is_ok());
}