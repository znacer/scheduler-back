use super::models::{SchedulerDataResponse, SchedulerLabel, TaskData, TaskDataFront};
use super::palette::Palette;
use actix_web::{get, web, Responder};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(test),
    components(schemas(SchedulerDataResponse, TaskDataFront, SchedulerLabel))
)]
pub struct ApiDocScheduler;

#[utoipa::path(
        responses(
            (status = 200, description = "Give some example tasks", body = [Vec::<SchedulerDataResponse>], example= json!(vec![TaskDataFront::new(TaskData::default())]))
        )
)]
#[get("/test")]
pub async fn test() -> impl Responder {
    let out = vec![
        SchedulerDataResponse::random_new("Ressource 1", Palette::Blue),
        SchedulerDataResponse::random_new("Ressource 2", Palette::Pink),
        SchedulerDataResponse::random_new("Ressource 3", Palette::Purple),
    ];

    web::Json(out)
}
