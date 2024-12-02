use super::utilities;
use super::{services_schedule::*, services_task::*, services_category::*, services_user::*, services_group::*};
use ::entity::{task, schedule};
use actix_web::http::header::ContentType;
use actix_web::{put, HttpResponse};
use utoipa::OpenApi;


#[derive(OpenApi)]
#[openapi(
    paths(
        create_tables,
        list_tasks,
        new_task,
        update_task,
        list_schedules,
        new_schedule,
        update_schedule,
        list_categories,
        new_category,
        update_category,
        list_users,
        new_user,
        list_groups,
        new_group
    ),
    components(schemas(
            schedule::Model,
            task::Model,
    ))
)]
pub struct ApiDocScheduler;

#[utoipa::path(
        responses(
            (
                status = 201, 
                description = "Creates tables if did not exists", 
                content_type = "text/plain"
            )
        ),
        tag = "admin"
)]
#[put("/create_tables")]
pub async fn create_tables() -> HttpResponse {
    match utilities::create_tables().await {
        Ok(_) => {}
        Err(err) => {
            dbg!("hello");
            return HttpResponse::InternalServerError()
                .content_type(ContentType::plaintext())
                .body(format!("{:?}", err));
        }
    }
    HttpResponse::Created()
        .content_type(ContentType::plaintext())
        .body("Tables exists or have been created")
}
