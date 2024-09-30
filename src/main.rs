use actix_cors::Cors;
use actix_web::{middleware::Logger, services, web, App, HttpServer};
use env_logger::Env;
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use scheduler_back::schedule_service;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("key.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();
    //
    env_logger::init_from_env(Env::default());

    #[derive(OpenApi)]
    #[openapi(
        nest(
            (path = "/scheduler-service", api = schedule_service::ApiDocScheduler, tags = ["Scheduler service"]),
        ),
        // paths(get_service::test),
        tags(
            (name = "Scheduler service", description = "Backend of the scheduler service, handles data and offer solutions through OR")
        )
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();
    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().send_wildcard();

        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors)
            .service(web::scope("/scheduler-service").service(services![
                schedule_service::update_task,
                schedule_service::test,
                schedule_service::create_tables
            ]))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    // .bind_openssl("127.0.0.1:8080", builder)?
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
