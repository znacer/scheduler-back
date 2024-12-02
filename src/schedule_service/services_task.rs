use super::utilities;
use ::entity::task;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use actix_web::{get, put, web, HttpResponse};

#[utoipa::path(
        responses(
            (status = 200, description = "list tasks", body = Vec::<task::Model> )
        ),
        tag = "task"
)]
#[get("/list-tasks")]
pub async fn list_tasks() -> HttpResponse {
    let db = utilities::sql_connect().await;

    match task::Entity::find().all(&db).await {
        Ok(all_tasks) => {
            HttpResponse::Ok().json(
            web::Json(all_tasks)
            )
        },
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("{:?}",err));
        },
    }
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
pub async fn new_task(task: web::Json<task::Model>) -> HttpResponse {
    let db = utilities::sql_connect().await;
    let mut this_task = task::ActiveModel {
        id: ActiveValue::NotSet,
        ..Default::default()
    };
    let _ = this_task.set_from_json(serde_json::json!(task));

    let result = this_task.insert(&db).await.unwrap();
    HttpResponse::Created().json(web::Json(result))
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
pub async fn update_task(task: web::Json<task::Model>) -> HttpResponse {
    let db = utilities::sql_connect().await;
    let this_task: Option<task::Model> = task::Entity::find_by_id(task.id).one(&db).await.unwrap();
    let mut this_task: task::ActiveModel = this_task.unwrap().into();
    let _ = this_task.set_from_json(serde_json::json!(task));

    let result = this_task.update(&db).await.unwrap();
    HttpResponse::Ok().json(web::Json(result))
}
