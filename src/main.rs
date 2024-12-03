use actix_cors::Cors;
use actix_web::{middleware::Logger, services, web, App, HttpServer};
use env_logger::Env;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use scheduler_back::schedule_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = match env::var_os("BACKEND_PORT") {
        Some(val) => match val.into_string() {
            Ok(val) => val,
            _ => "8080".to_string(),
        },
        _ => "8080".to_string(),
    };
    println!("PORT = {port}");

    env_logger::init_from_env(Env::default());

    #[derive(OpenApi)]
    #[openapi(
        nest(
            (path = "/scheduler", api = schedule_service::ApiDocScheduler),
        ),
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .allow_any_origin()
            .send_wildcard();

        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors)
            .service(
                web::scope("/scheduler")
                    .service(schedule_service::create_tables)
                    .service(services![
                        schedule_service::list_tasks,
                        schedule_service::new_task,
                        schedule_service::update_task,
                        schedule_service::delete_task,
                    ])
                    .service(services![
                        schedule_service::list_schedules,
                        schedule_service::new_schedule,
                        schedule_service::update_schedule,
                        schedule_service::delete_schedule,
                    ])
                    .service(services![
                        schedule_service::list_categories,
                        schedule_service::new_category,
                        schedule_service::update_category,
                    ])
                    .service(services![
                        schedule_service::list_users,
                        schedule_service::new_user,
                        schedule_service::delete_user,
                        schedule_service::delete_user,
                    ])
                    .service(services![
                        schedule_service::list_groups,
                        schedule_service::new_group,
                        schedule_service::delete_group,
                    ]),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    // .bind_openssl("127.0.0.1:8080", builder)?
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
