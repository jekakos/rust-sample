use crate::webhooks::webhook_lib::telegram_mod::location_service::send_telegram_notification_location;
use super::{location_repo::MockLocationRepositoryI, telegram_service::MockTelegramServiceI};

#[tokio::test]
async fn test_send_telegram_notification_location() {
    let mut location_repo_mock = MockLocationRepositoryI::new();
    let mut tg_service_mock = MockTelegramServiceI::new();

    let last_location = "Russia".to_owned();
    let current_location = "Portugal".to_owned();

    // Setup mock
    location_repo_mock.expect_find_penultimate_location()
        .returning(move |_| {
            let last_location_clone = last_location.clone();
            Box::pin(async move { Ok(Some(last_location_clone)) })
        });
    
    // Location was changed
    let current_location_clone = current_location.clone();
    tg_service_mock.expect_send_location()
        .withf(
            move |_imsi, location| 
            * &location ==  current_location_clone.to_owned()
        )
        .times(1) // Убедитесь, что метод был вызван один раз
        .returning(|_, _| Box::pin(async move {}));
    
    // Location wasn't changed
    let the_same_location = current_location.clone();
    tg_service_mock.expect_send_location()
        .withf(
            move |_imsi, location| 
            * &location ==  the_same_location.to_owned()
        )
        .times(0) // Haven't called
        .returning(|_, _| Box::pin(async move {}));

    // Вызов тестируемой функции
    let result = send_telegram_notification_location(
        &location_repo_mock, 
        &tg_service_mock, 
        "some_imsi", 
        &current_location
    ).await;

    assert!(result.is_ok());
}