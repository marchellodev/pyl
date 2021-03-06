use actix_web::web;

use crate::admin::*;

pub fn router(cfg: &mut web::ServiceConfig) {
    /*
    Endpoints
    1. Tracking endpoints
        1. Ping
        2. Events
        3. Plugin calls
    2. Manager endpoints
        X. Website management
        2. User management
        3. Plugin management
     */

    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/admin")
                    .wrap(middleware::CheckLogin)
                    .service(
                        web::resource("/projects")
                            .route(web::put().to(projects::routes::create))
                            .route(web::get().to(projects::routes::list))
                            .route(web::post().to(projects::routes::edit))
                            .route(web::delete().to(projects::routes::delete)),
                    )
                    .service(
                        web::resource("/users")
                            .route(web::post().to(users::routes::login))
                            .route(web::get().to(users::routes::list))
                            .route(web::put().to(users::routes::create))
                            .route(web::delete().to(users::routes::delete)),
                    ),
            )
            .service(web::scope("/data"))
            .service(web::scope("/tracking")),
    );
}
