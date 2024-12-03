use super::utilities;
use ::entity::task;
use actix_web::{delete, get, put, web, Error, HttpResponse};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};

#[utoipa::path(
        responses(
            (status = 200, description = "list tasks", body = Vec::<task::Model> )
        ),
        tag = "task"
)]
#[get("/list-tasks")]
pub async fn list_tasks() -> Result<HttpResponse, Error> {
    let db = utilities::sql_connect().await;

    let response = task::Entity::find()
        .all(&db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    Ok(HttpResponse::Ok().json(web::Json(response)))
}

#[utoipa::path(
        request_body(
            content = task::Model,
            example = json!(
                task::Model::default()
            )
        ),
        responses(
            (
                status = 201,
                description = "the newly created task",
                content_type = "text/json",
            )
        ),
        tag = "task"
)]
#[put("/new-task")]
pub async fn new_task(task: web::Json<task::Model>) -> Result<HttpResponse, Error> {
    let db = utilities::sql_connect().await;
    let mut this_task = task::ActiveModel {
        id: ActiveValue::NotSet,
        ..Default::default()
    };
    let _ = this_task.set_from_json(serde_json::json!(task));

    let result = this_task
        .insert(&db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    Ok(HttpResponse::Created().json(web::Json(result)))
}

#[utoipa::path(
        request_body(
            content = task::Model,
            example = json!(
                task::Model::default()
            )
        ),
        responses(
            (
                status = 200,
                description = "the updated element",
                content_type = "text/json",
            )
        ),
        tag = "task"
)]
#[put("/update-task")]
pub async fn update_task(task: web::Json<task::Model>) -> Result<HttpResponse, Error> {
    let db = utilities::sql_connect().await;
    let this_task: Option<task::Model> = task::Entity::find_by_id(task.id)
        .one(&db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    let mut this_task: task::ActiveModel = this_task.unwrap().into();
    let _ = this_task.set_from_json(serde_json::json!(task));

    let result = this_task.update(&db).await.unwrap();
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
                description = "task deleted",
                content_type = "text/json",
            )
        ),
        description = "Delete a task based on its id",
        tag = "task"
)]
#[delete("/delete-task")]
pub async fn delete_task(task_id: web::Json<utilities::IdRequest>) -> Result<HttpResponse, Error> {
    let db = utilities::sql_connect().await;
    let this_task = task::Entity::find_by_id(task_id.id)
        .one(&db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    let this_task: task::ActiveModel = match this_task {
        Some(v) => v.into(),
        None => return Err(actix_web::error::ErrorNotFound("No ID found")),
    };

    let _ = this_task
        .delete(&db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;

    Ok(HttpResponse::NoContent().into())
}
