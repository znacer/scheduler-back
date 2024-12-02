use super::utilities;
use ::entity::schedule;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use actix_web::{get, put, web, HttpResponse};

#[utoipa::path(
        responses(
            (status = 200, description = "list schedules", body = Vec::<schedule::Model> )
        ),
        tag = "schedule"
)]
#[get("/list-schedules")]
pub async fn list_schedules() -> HttpResponse {
    let db = utilities::sql_connect().await;

    match schedule::Entity::find().all(&db).await {
        Ok(all_schedules) => {
            HttpResponse::Ok().json(
            web::Json(all_schedules)
            )
        },
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("{:?}",err));
        },
    }
}
#[utoipa::path(
        request_body(
            content = schedule::Model,
            example = json!( 
                schedule::Model::default()
            )
        ),
        responses(
            (
                status = 201, 
                description = "the newly created schedule",
                content_type = "text/json",
            )
        ),
        tag = "schedule"
)]
#[put("/new-schedule")]
pub async fn new_schedule(schedule: web::Json<schedule::Model>) -> HttpResponse {
    let db = utilities::sql_connect().await;
    let mut this_schedule = schedule::ActiveModel {
        id: ActiveValue::NotSet,
        ..Default::default()
    };
    let _ = this_schedule.set_from_json(serde_json::json!(schedule));
    this_schedule.id = ActiveValue::NotSet;

    let result = this_schedule.insert(&db).await.unwrap();
    HttpResponse::Created().json(web::Json(result))
}

#[utoipa::path(
        request_body(
            content = schedule::Model,
            example = json!( 
                schedule::Model::default()
            )
        ),
        responses(
            (
                status = 200, 
                description = "the updated element",
                content_type = "text/json",
            )
        ),
        tag = "schedule"
)]
#[put("/update-schedule")]
pub async fn update_schedule(schedule: web::Json<schedule::Model>) -> HttpResponse {
    let db = utilities::sql_connect().await;
    let this_schedule: Option<schedule::Model> = schedule::Entity::find_by_id(schedule.id).one(&db).await.unwrap();
    let mut this_schedule: schedule::ActiveModel = this_schedule.unwrap().into();
    let _ = this_schedule.set_from_json(serde_json::json!(schedule));

    let result = this_schedule.update(&db).await.unwrap();
    HttpResponse::Ok().json(web::Json(result))
}
