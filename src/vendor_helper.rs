use axum::{extract::{Path, Query}, http::StatusCode, response::Json, Extension};
use axum_macros::debug_handler;
use lightsim_vendor_interface::{
    dto::{
        balance_dto::*, esim_dto::*, extension_dto::*, get_quote_dto::*, package_dto::*
    }, interface::VendorInterface
};
use crate::vendor::ServiceVendor;


#[utoipa::path(
    post,
    path = "/get_quote",
    tag = "Esim",
    request_body(content = GetQuoteRequestInterfaceDTO, content_type = "application/json"),
    responses(
        (status = 200, body = GetQuoteRequestInterfaceDTO),
        (status = 500, description = "Error")
    ),
)]
pub async fn get_quote_helper(
    Extension(vendor): Extension<ServiceVendor>,
    Query(request): Query<GetQuoteRequestInterfaceDTO>
) -> Result<Json<GetQuoteResponseInterfaceDTO>, (StatusCode, String)> {
    let result = vendor.get_quote(request.clone()).await?;
    Ok(Json(result))
}


#[utoipa::path(
    post,
    path = "/esim",
    tag = "Esim",
    request_body(content = CreateEsimRequestInterfaceDTO, content_type = "application/json"),
    responses(
        (status = 200, body = CreateEsimResponseInterfaceDTO),
        (status = 500, description = "Error")
    ),
)]
pub async fn create_esim_helper(
    Extension(vendor): Extension<ServiceVendor>,
    Json(request): Json<CreateEsimRequestInterfaceDTO>
) -> Result<Json<CreateEsimResponseInterfaceDTO>, (StatusCode, String)> {
    let result = vendor.create_esim(request.clone()).await?;
    Ok(Json(result))
}




#[utoipa::path(
    get,
    path = "/esim",
    tag = "Esim",
    params(GetEsimRequestInterfaceDTO),
    responses(
        (status = 200, body = [GetEsimResponseInterfaceDTO]),
        (status = 500, description = "Error")
    ),
)]
pub async fn get_esim_helper(
    Extension(vendor): Extension<ServiceVendor>,
    Query(request): Query<GetEsimRequestInterfaceDTO>,
   //Json(request): Json<GetEsimRequestDTO>
) -> Result<Json<Vec<GetEsimResponseInterfaceDTO>>, (StatusCode, String)> {
    let result = vendor.get_esim(request.clone()).await;
    match result {
        Ok(response) => {
            Ok(Json(response))
        }
        Err((code,error)) => {
            Err((code, error))
        }
    }
}


#[debug_handler]
#[utoipa::path(
    post,
    path = "/extension",
    tag = "Extension",
    request_body(content = AddExtensionRequestInterfaceDTO, content_type = "application/json"),
    responses(
        (status = 201, body = GetExtensionResponseInterfaceDTO),
        (status = 500, description = "Error")
    ),
)]
pub async fn add_extension_helper(
    Extension(vendor): Extension<ServiceVendor>,
    Json(request): Json<AddExtensionRequestInterfaceDTO>,
) -> Result<Json<GetExtensionResponseInterfaceDTO>, (StatusCode, String)> {
    let data = vendor.add_extension(request.clone()).await?;
    Ok(Json(data))
}


#[utoipa::path(
    get,
    path = "/extension",
    tag = "Extension",
    request_body(content = AddExtensionRequestInterfaceDTO, content_type = "application/json"),
    responses(
        (status = 201, body = [GetExtensionResponseInterfaceDTO]),
        (status = 500, description = "Error")
    ),
)]
pub async fn get_extension_helper(
    Extension(vendor): Extension<ServiceVendor>,
    Query(request): Query<GetExtensionRequestInterfaceDTO>,
) -> Result<Json<Vec<GetExtensionResponseInterfaceDTO>>, (StatusCode, String)> {
    let result = vendor.get_extension(request.clone()).await?;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/extension/{iccid}/{extension_id}",
    tag = "Extension",
    params(DeleteExtensionRequestInterfaceDTO),
    params(
        ("iccid", description = "Esim iccid"),
        ("extension_id", description = "Extension local id"),
    ),
    responses(
        (status = 204, description = "Succes"),
        (status = 500, description = "Error")
    ),
)]
pub async fn delete_extension_helper(
    Extension(vendor): Extension<ServiceVendor>,
    Path((iccid, extension_id)): Path<(String, i32)>,
) -> Result<(StatusCode, String), (StatusCode, String)> {

    let request = DeleteExtensionRequestInterfaceDTO { 
        iccid, 
        extension_id, 
    };
    
    let result = vendor.delete_extension(request.clone()).await;
    match result {
        Ok((code,error)) => {
            Ok((code, error))
        }
        Err((code,error)) => {
            Err((code, error))
        }
    }
}


#[utoipa::path(
    get,
    path = "/package",
    tag = "Package",
    params(GetPackageRequestInterfaceDTO),
    responses(
        (status = 200, body = [GetPackageResponseInterfaceDTO]),
        (status = 500, description = "Error")
    ),
)]
pub async fn get_package_helper(
    Extension(vendor): Extension<ServiceVendor>,
    Query(request): Query<GetPackageRequestInterfaceDTO>,
) -> Result<Json<Vec<GetPackageResponseInterfaceDTO>>, (StatusCode, String)> {
    let result = vendor.get_package(request.clone()).await?;
    Ok(Json(result))
    //Err((StatusCode::INTERNAL_SERVER_ERROR, "Service has no packages".to_string()))
}


#[utoipa::path(
    patch,
    path = "/package/{id}",
    tag = "Package",
    params(
        ("id", description = "Package id"),
    ),
    request_body(content = UpdatePackageRequestInterfaceDTO, content_type = "application/json"),
    responses(
        (status = 200, body = GetPackageResponseInterfaceDTO),
        (status = 500, description = "Error")
    ),
)]
pub async fn update_package_helper(
    Extension(vendor): Extension<ServiceVendor>,
    Path(id): Path<String>,
    Json(request): Json<UpdatePackageRequestInterfaceDTO>,
) -> Result<Json<GetPackageResponseInterfaceDTO>, (StatusCode, String)> {
    let result = vendor.update_package(id, request.clone()).await?;
    Ok(Json(result))
    //Err((StatusCode::INTERNAL_SERVER_ERROR, "Service has no packages".to_string()))
}


#[utoipa::path(
    post,
    path = "/balance/{iccid}/topup",
    tag = "Balance",
    params(
        ("iccid", description = "Esim iccid"),
    ),
    request_body(content = AddBalanceRequestInterfaceDTO, content_type = "application/json"),
    responses(
        (status = 200, body = AddBalanceResponseInterfaceDTO),
        (status = 500, description = "Error")
    ),
)]
pub async fn add_balance_helper(
    Extension(vendor): Extension<ServiceVendor>,
    Path(iccid): Path<String>,
    Json(request_data): Json<AddBalanceRequestInterfaceDTO>,
) -> Result<Json<AddBalanceResponseInterfaceDTO>, (StatusCode, String)> {

    let result = vendor.add_balance(iccid, request_data).await?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/balance/{iccid}/refund",
    tag = "Balance",
    params(
        ("iccid", description = "Esim iccid"),
    ),
    request_body(content = RefundBalanceRequestInterfaceDTO, content_type = "application/json"),
    responses(
        (status = 200, body = RefundBalanceResponseInterfaceDTO),
        (status = 500, description = "Error")
    ),
)]
pub async fn refund_balance_helper(
    Extension(vendor): Extension<ServiceVendor>,
    Path(iccid): Path<String>,
    Json(request_data): Json<RefundBalanceRequestInterfaceDTO>,
) -> Result<Json<RefundBalanceResponseInterfaceDTO>, (StatusCode, String)> {

    let result = vendor.refund_balance(iccid, request_data).await?;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/balance/{iccid}",
    tag = "Balance",
    params(
        ("iccid", description = "Esim iccid"),
    ),
    responses(
        (status = 200, body = GetBalanceResponseInterfaceDTO),
        (status = 500, description = "Error")
    ),
)]
// Vendor method, doc in helper
pub async fn get_balance_helper(
    Extension(vendor): Extension<ServiceVendor>,
    Path(iccid): Path<String>,
) -> Result<Json<GetBalanceResponseInterfaceDTO>, (StatusCode, String)> {
    let result = vendor.get_balance(iccid).await?;
    Ok(Json(result))
}