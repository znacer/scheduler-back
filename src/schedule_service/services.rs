use super::utilities;
use super::{
    services_category::*, services_group::*, services_schedule::*, services_schedule_group::*,
    services_task::*, services_user::*, services_user_group::*,
};
use ::entity::{schedule, task};
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
        delete_task,
        list_schedules,
        new_schedule,
        update_schedule,
        delete_schedule,
        list_categories,
        new_category,
        update_category,
        list_users,
        new_user,
        create_my_user,
        delete_user,
        list_groups,
        new_group,
        delete_group,
        list_user_groups,
        new_user_group,
        delete_user_group,
        list_my_groups,
        list_my_groups_admin,
        list_schedule_groups,
        new_schedule_group,
        delete_schedule_group
    ),
    components(schemas(schedule::Model, task::Model,))
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
        tag = "admin",
        security(
            ("bearerAuth" = [])
        )
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
