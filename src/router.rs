use crate::{
    esim::esim_controller::upload_esims_file,
    openapi::ApiDoc,
    package::package_service::save_packages_from_csv,
    region::region_controller::{
        get_region, load_pyg_prices_from_csv, load_regions_from_csv, update_region_prices
    },
    vendor::ServiceVendor,
    vendor_helper::*,
    service::rsp::esim_info::get_esim_info_handler,
    webhooks::webhook_lib::handler_mod::handler::{get_last_balance, get_webhook, post_webhook},
};
use axum::{
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::{delete, get, patch, post, Router},
    Extension,
};
use lightsim_vendor_interface::types::{RouteMethodType, VendorCommonRoutes};
use reqwest::Client;
use sea_orm::DatabaseConnection;
use sim_auth::sim_lib::handler_mod::handler::{
    delete_user_link, esim_activate, esim_delete_extension, esim_link_user, esim_push, esim_qrcode,
    esim_reissue_extension, get_esim_user, get_tariff_packet,
};

use lightsim_logger::ext::axum_lyer::logger; //, package_name};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub async fn route_init(db: &DatabaseConnection, client: &Client, vendor: &ServiceVendor) -> Router {
    //tracing_sub().await;

    let mut router = Router::new();

    // Specific Service routes
    router = router
        .route("/esim/upload", post(upload_esims_file))
        .route("/delete-user-link", delete(delete_user_link))
        .route("/get-esim-user", get(get_esim_user))
        .route("/get-packet-dictionary", get(get_tariff_packet))
        .route("/esim-link-user", post(esim_link_user))
        .route("/esim-qrcode", post(esim_qrcode))
        .route("/esim-push-user", post(esim_push))
        .route("/esim-activate", post(esim_activate))
        .route("/esim-reissue-extension", post(esim_reissue_extension))
        .route("/esim-delete-extension", post(esim_delete_extension))
        .route("/absorber", get(get_webhook).post(post_webhook))
        .route("/get-last-balance", get(get_last_balance))
        // New routes
        .route("/package/save-from-csv", post(save_packages_from_csv))
        .route("/region/update-prices", get(update_region_prices))
        //.route("/extension", post(add_extension))
        //.route("/extension", get(get_extensions))
        .route("/region/load", post(load_regions_from_csv))
        .route("/region/update-pyg", post(load_pyg_prices_from_csv))
        .route("/region", get(get_region))
        .route("/esim/:iccid/info", get(get_esim_info_handler));
    //.route("/package", get(get_package))
    //.route("/package/:slug", patch(update_package));

    // Common routes
    let vendor_routes = VendorCommonRoutes::all();
    for route in vendor_routes {
        match (route.method_type, route.route) {
            // Esim
            (RouteMethodType::GET, "/get-quote") => {
                router = router.route(&route.route, get(get_quote_helper));
            }

            (RouteMethodType::POST, "/esim") => {
                router = router.route(&route.route, post(create_esim_helper));
            }

            (RouteMethodType::GET, "/esim") => {
                router = router.route(&route.route, get(get_esim_helper));
            }

            // Extension
            (RouteMethodType::POST, "/extension") => {
                router = router.route(&route.route, post(add_extension_helper));
            }

            (RouteMethodType::GET, "/extension") => {
                router = router.route(&route.route, get(get_extension_helper));
            }

            (RouteMethodType::DELETE, "/extension/:iccid/:extension_id") => {
                router = router.route(&route.route, delete(delete_extension_helper));
            }

            // Package
            (RouteMethodType::GET, "/package") => {
                router = router.route(&route.route, get(get_package_helper));
            }

            (RouteMethodType::PATCH, "/package/:id") => {
                router = router.route(&route.route, patch(update_package_helper));
            }

            // Balance
            (RouteMethodType::POST, "/balance/:iccid/topup") => {
                router = router.route(&route.route, post(add_balance_helper));
            }

            (RouteMethodType::POST, "/balance/:iccid/refund") => {
                router = router.route(&route.route, post(refund_balance_helper));
            }

            (RouteMethodType::GET, "/balance/:iccid") => {
                router = router.route(&route.route, get(get_balance_helper));
            }

            _ => {
                panic!("Vendor Interface error: Not all common routes are implemented or incorrect path names");
            }
        }
    }

    // Add Swagger UI
    router = router.merge(SwaggerUi::new("/doc").url("/doc/openapi.json", ApiDoc::openapi()));

    // Add Extensions
    router = router
        .layer(middleware::from_fn(show_info))
        .layer(Extension(client.clone()))
        .layer(Extension(db.clone()))
        .layer(Extension(vendor.clone()))
        .layer(middleware::from_fn({
            move |req, next| logger("service-service".to_owned(), req, next)
        }));

    router
}

async fn show_info(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().clone();
    let response = next.run(req).await;
    println!(
        "##########################################>>>>>>>>= {} {}",
        method, path
    );
    response
}
