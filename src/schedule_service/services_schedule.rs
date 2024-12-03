use super::utilities;
use ::entity::schedule;
use actix_web::{delete, get, put, web, Error, HttpResponse};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};

#[utoipa::path(
        responses(
            (status = 200, description = "list schedules", body = Vec::<schedule::Model> )
        ),
        tag = "schedule"
)]
#[get("/list-schedules")]
pub async fn list_schedules() -> Result<HttpResponse, Error> {
    let db = utilities::sql_connect().await;
    let response = schedule::Entity::find()
        .all(&db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    Ok(HttpResponse::Ok().json(web::Json(response)))
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
pub async fn new_schedule(schedule: web::Json<schedule::Model>) -> Result<HttpResponse, Error> {
    let db = utilities::sql_connect().await;
    let mut this_schedule = schedule::ActiveModel {
        id: ActiveValue::NotSet,
        ..Default::default()
    };
    let _ = this_schedule.set_from_json(serde_json::json!(schedule));
    this_schedule.id = ActiveValue::NotSet;

    let result = this_schedule
        .insert(&db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    Ok(HttpResponse::Created().json(web::Json(result)))
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
pub async fn update_schedule(schedule: web::Json<schedule::Model>) -> Result<HttpResponse, Error> {
    let db = utilities::sql_connect().await;
    let this_schedule: Option<schedule::Model> = schedule::Entity::find_by_id(schedule.id)
        .one(&db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    let mut this_schedule: schedule::ActiveModel = this_schedule.unwrap().into();
    let _ = this_schedule.set_from_json(serde_json::json!(schedule));

    let result = this_schedule.update(&db).await.unwrap();
    Ok(HttpResponse::Ok().json(web::Json(result)))
}

#[utoipa::path(
        request_body(
            content = utilities::IdRequest,
            example = json!(
                utilities::IdRequest { id: 0}
            )
        ),
        responses(
            (
                status = 204,
                description = "schedule deleted",
                content_type = "text/json",
            )
        ),
        description = "Delete a schedule based on its id",
        tag = "schedule"
)]
#[delete("/delete-schedule")]
pub async fn delete_schedule(
    schedule_id: web::Json<utilities::IdRequest>,
) -> Result<HttpResponse, Error> {
    let db = utilities::sql_connect().await;
    let this_schedule = schedule::Entity::find_by_id(schedule_id.id)
        .one(&db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    let this_schedule: schedule::ActiveModel = match this_schedule {
        Some(v) => v.into(),
        None => return Err(actix_web::error::ErrorNotFound("No ID found")),
    };

    let _ = this_schedule
        .delete(&db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;

    Ok(HttpResponse::NoContent().into())
}
