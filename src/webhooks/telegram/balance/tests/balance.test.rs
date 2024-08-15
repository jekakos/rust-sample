use serde_json::json;

use crate::webhooks::webhook_lib::telegram_mod::{balance_service::send_telegram_notification_balance, package_repo::MockPackageRepositoryI};
use super::{balance_repo::MockBalanceRepositoryI, telegram_service::MockTelegramServiceI};

#[tokio::test]
async fn test_send_telegram_notification_balance_euro() {
    let mut balance_repo_mock = MockBalanceRepositoryI::new();
    let mut tg_service_mock = MockTelegramServiceI::new();
    let mut package_repo_mock = MockPackageRepositoryI::new();

    let border_euro = 10.0; // 10 EUR
    let border = border_euro.to_string();
    let border_less = "9.0".to_owned();
    let border_more = "11.0".to_owned();
    let package_value = "".to_string();
    let uuid = "some_uuid".to_owned();

    // Setting up mocks
    balance_repo_mock.expect_find_penultimate_balance()
        .returning(move |_| {
            let border_clone = border_more.clone();
            Box::pin(async move { Ok(Some(border_clone)) })
        });
    
    package_repo_mock.expect_get_package()
        .returning(move |_| {
            let json_obj = json!({"package_gb_value": package_value});
            Box::pin(async move { json_obj })
        });

    tg_service_mock.expect_send_balance()
        .withf(
            move |_imsi, balance_less, border_lable, balance_type| 
            * &balance_less == border.to_owned() && 
            * &border_lable == "fix" && 
            * &balance_type == "euro"
        )
        .times(1) // Run only one time
        .returning(|_, _, _, _| Box::pin(async move {}));

    // Вызов тестируемой функции
    let result = send_telegram_notification_balance(
        &balance_repo_mock, 
        &tg_service_mock, 
        &package_repo_mock,
        "some_imsi", 
        &uuid,
        &border_less // Balance is less than 10 eur
    ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_send_telegram_notification_balance_bytes() {
    let mut balance_repo_mock = MockBalanceRepositoryI::new();
    let mut tg_service_mock = MockTelegramServiceI::new();
    let mut package_repo_mock = MockPackageRepositoryI::new();

    let border_bytes: u128 = 200*1024*1024; // 200 mb
    let border = border_bytes.to_string();
    let border_less = (border_bytes - 1).to_string();
    let border_more = (border_bytes + 1).to_string();
    let package_value_gb = 1; // 1 Gb
    let uuid = "some_uuid".to_owned();

    // Setting up mocks
    balance_repo_mock.expect_find_penultimate_balance()
        .returning(move |_| {
            let border_clone = border_more.clone();
            Box::pin(async move { Ok(Some(border_clone)) })
        });
    package_repo_mock.expect_get_package()
        .returning(move |_| {
            let json_obj = json!({"package_gb_value": package_value_gb});
            Box::pin(async move { json_obj })
        });

    tg_service_mock.expect_send_balance()
        .withf(
            move |_imsi, balance_less, border_lable, balance_type| 
            * &balance_less == border.to_owned() && 
            * &border_lable == "fix" && 
            * &balance_type == "bytes")
        .times(1) // Run only one time
        .returning(|_, _, _, _| Box::pin(async move {}));

    // Вызов тестируемой функции
    let result = send_telegram_notification_balance(
        &balance_repo_mock, 
        &tg_service_mock, 
        &package_repo_mock,
        "some_imsi", 
        &uuid, 
        &border_less // Balanse is less than 50 mb
    ).await;

    assert!(result.is_ok());
}