use axum::http::StatusCode;
use service_interface::{ 
    dto::{
        esim_dto::*, 
        extension_dto::*, 
        package_dto::*,
        balance_dto::*,
        get_quote_dto::*
    }, 
    interface::VendorInterface
};
use async_trait::async_trait;

use crate::{
    balance::balance_controller::{add_balance, get_balance, refund_balance}, 
    esim::esim_controller::{create_esim, get_esim}, 
    extension::extension_controller::{add_extension, delete_extension, get_extension}, 
    package::package_controller::{get_package, update_package}, 
    region::region_controller::get_quote
};


#[derive(Clone)]
pub struct ServiceVendor {}

impl ServiceVendor {
    pub fn new() -> Self {
        ServiceVendor {}
    }
}

#[async_trait]
impl VendorInterface for ServiceVendor {

    // Get a quote
    async fn get_quote(&self,
        request: GetQuoteRequestInterfaceDTO
    ) -> Result<GetQuoteResponseInterfaceDTO, (StatusCode, String)> {
        get_quote(request).await
    }

    // Esim group
    async fn create_esim(&self,
        request: CreateEsimRequestInterfaceDTO
    ) -> Result<CreateEsimResponseInterfaceDTO, (StatusCode, String)> {
        create_esim(request).await
    }

    async fn get_esim(&self,
        request: GetEsimRequestInterfaceDTO
    ) -> Result<Vec<GetEsimResponseInterfaceDTO>, (StatusCode, String)> {
        get_esim(request).await
    }

    // Extension group
    async fn add_extension(&self,
        request: AddExtensionRequestInterfaceDTO
    ) -> Result<GetExtensionResponseInterfaceDTO, (StatusCode, String)> {
        add_extension(request).await
    }

    async fn get_extension(&self,
        request: GetExtensionRequestInterfaceDTO
    ) -> Result<Vec<GetExtensionResponseInterfaceDTO>, (StatusCode, String)> {
        get_extension(request).await
    }

    async fn delete_extension(&self,
        request: DeleteExtensionRequestInterfaceDTO
    ) -> Result<(StatusCode, String), (StatusCode, String)> {
        delete_extension(request).await
    }

    // Balance group
    async fn add_balance(&self,
        iccid: String,
        request: AddBalanceRequestInterfaceDTO
    ) -> Result<AddBalanceResponseInterfaceDTO, (StatusCode, String)> {
        add_balance(iccid, request).await
    }

    async fn refund_balance(&self,
        iccid: String,
        request: RefundBalanceRequestInterfaceDTO
    ) -> Result<RefundBalanceResponseInterfaceDTO, (StatusCode, String)> {
        refund_balance(iccid, request).await
    }

    async fn get_balance(&self, 
        iccid: String,
    ) -> Result<GetBalanceResponseInterfaceDTO, (StatusCode, String)> {
        get_balance(iccid).await
    }

    // Package group
    async fn get_package(&self, 
        request: GetPackageRequestInterfaceDTO
    ) -> Result<Vec<GetPackageResponseInterfaceDTO>, (StatusCode, String)> {
        get_package(request).await
    }

    async fn update_package(&self,
        id: String,
        request: UpdatePackageRequestInterfaceDTO
    ) -> Result<GetPackageResponseInterfaceDTO, (StatusCode, String)> {
        update_package(id, request).await
    }

}