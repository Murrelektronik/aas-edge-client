use actix_web::web;
use crate::handlers; // Correctly import the handlers module

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/").route(web::get().to(handlers::home::index))
        )
        .service(
            web::resource("/submodels")
                .route(web::get().to(handlers::submodels::get_submodels))
        )
        .service(
            web::resource("/submodels/{submodel_id}")
                .route(web::get().to(handlers::submodels::get_submodel))
                .route(web::patch().to(handlers::submodels::patch_submodel))
        )
        .service(
            web::resource("/openapi")
                .route(web::get().to(handlers::openapi::openapi_endpoint)))
        .service(
            web::resource("/picture").route(web::get().to(handlers::picture::get_picture))
        );

}
