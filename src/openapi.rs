use lightsim_vendor_interface::dto::{
  esim_dto::*, 
  extension_dto::*, 
  package_dto::*, 
  balance_dto::*,
};
use crate::extension::dto::extension_dto::{
  GetExtensionsRequestDTO, GetExtensionsResponseDTO
};

use crate::vendor_helper::*;
//use crate::extension::extension_controller::*;

use utoipa::OpenApi;


#[derive(OpenApi)]
#[openapi(
    info(title = "Service Vendor", description = "Service Vendor methods"),
    components(schemas(
      CreateEsimRequestInterfaceDTO, 
      CreateEsimResponseInterfaceDTO,

      GetEsimRequestInterfaceDTO, 
      GetEsimResponseInterfaceDTO,

      GetPackageRequestInterfaceDTO,
      GetPackageResponseInterfaceDTO,

      UpdatePackageRequestInterfaceDTO,

      AddExtensionRequestInterfaceDTO,
      GetExtensionsRequestDTO,
      GetExtensionsResponseDTO,
      DeleteExtensionRequestInterfaceDTO,

      AddBalanceRequestInterfaceDTO,
      RefundBalanceRequestInterfaceDTO,
      GetBalanceResponseInterfaceDTO,

    )),
    paths(
      create_esim_helper, get_esim_helper,
      get_package_helper, update_package_helper,
      add_extension_helper, get_extension_helper, delete_extension_helper,
    ),
    tags(
        (name = "Esim", description = "Group of endpoints for working with esim"),
        (name = "Package", description = "Group of endpoints for working with package"),
        (name = "Extension", description = "Group of endpoints for working with extenstion"),
        (name = "Balance", description = "Group of endpoints for working with balance"),
    ),
)]
pub struct ApiDoc;
