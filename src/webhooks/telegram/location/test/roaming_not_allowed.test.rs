use crate::webhooks::webhook_lib::telegram_mod::location_service::send_support_warning_if_roaming_not_allowed;
use super::{location_repo::MockLocationRepositoryI, telegram_service::MockTelegramServiceI};

#[tokio::test]
async fn test_send_support_warning_if_roaming_not_allowed1() {
    let mut location_repo_mock = MockLocationRepositoryI::new();
    let mut tg_service_mock = MockTelegramServiceI::new();

    let last_states1 = 
      [
        "plmnRoamingNotAllowed".to_owned(), 
        "plmnRoamingNotAllowed".to_owned(), 
        "ALLOW".to_owned()
      ].to_vec();

    let current_state = "plmnRoamingNotAllowed";
    // Setup mock 1
    location_repo_mock.expect_find_30min_states()
        .returning(move |_| {
            let res_clone = last_states1.clone();
            Box::pin(async move { Ok(res_clone) })
        });
    
    tg_service_mock.expect_send_to_channel()
        .times(0) // !!!
        .returning(|_| Box::pin(async move {}));

    let result = send_support_warning_if_roaming_not_allowed(
        &location_repo_mock, 
        &tg_service_mock, 
        "some_imsi", 
        &current_state
    ).await;
    assert!(result.is_ok());
}


//------------------------------------------------
#[tokio::test]
async fn test_send_support_warning_if_roaming_not_allowed2() {
    let mut location_repo_mock = MockLocationRepositoryI::new();
    let mut tg_service_mock = MockTelegramServiceI::new();

    let last_states2 = 
      [
        "plmnRoamingNotAllowed".to_owned(),
        "plmnRoamingNotAllowed".to_owned(), 
        "plmnRoamingNotAllowed".to_owned()
      ].to_vec();

    let current_state = "plmnRoamingNotAllowed";
    // Setup mock 2
    location_repo_mock.expect_find_30min_states()
        .returning(move |_| {
            let res_clone = last_states2.clone();
            Box::pin(async move { Ok(res_clone) })
        });
    
    tg_service_mock.expect_send_to_channel()
        .times(1) // !!!
        .returning(|_| Box::pin(async move {}));

    let result = send_support_warning_if_roaming_not_allowed(
        &location_repo_mock, 
        &tg_service_mock, 
        "some_imsi", 
        &current_state
    ).await;
    assert!(result.is_ok());
}

//------------------------------------------------
#[tokio::test]
async fn test_send_support_warning_if_roaming_not_allowed3() {
    let mut location_repo_mock = MockLocationRepositoryI::new();
    let mut tg_service_mock = MockTelegramServiceI::new();
    let last_states3 = ["ALLOW".to_owned()].to_vec();

    let current_state = "plmnRoamingNotAllowed";
    // Setup mock 3
    location_repo_mock.expect_find_30min_states()
        .returning(move |_| {
            let res_clone = last_states3.clone();
            Box::pin(async move { Ok(res_clone) })
        });
    
    tg_service_mock.expect_send_to_channel()
        .times(0) // !!!
        .returning(|_| Box::pin(async move {}));

    let result = send_support_warning_if_roaming_not_allowed(
        &location_repo_mock, 
        &tg_service_mock, 
        "some_imsi", 
        &current_state
    ).await;
    assert!(result.is_ok());
}


//------------------------------------------------
#[tokio::test]
async fn test_send_support_warning_if_roaming_not_allowed4() {
    let mut location_repo_mock = MockLocationRepositoryI::new();
    let mut tg_service_mock = MockTelegramServiceI::new();
    let last_states4 = ["plmnRoamingNotAllowed".to_owned()].to_vec();

    let current_state = "plmnRoamingNotAllowed";
    // Setup mock 3
    location_repo_mock.expect_find_30min_states()
        .returning(move |_| {
            let res_clone = last_states4.clone();
            Box::pin(async move { Ok(res_clone) })
        });
    
    tg_service_mock.expect_send_to_channel()
        .times(0) // !!!
        .returning(|_| Box::pin(async move {}));

    let result = send_support_warning_if_roaming_not_allowed(
        &location_repo_mock, 
        &tg_service_mock, 
        "some_imsi", 
        &current_state
    ).await;
    assert!(result.is_ok());
}