use super::utilities;
use ::entity::schedule_group;
use actix_web::{delete, get, put, web, Error, HttpRequest, HttpResponse};
use core::str;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};

#[utoipa::path(
        responses(
            (status = 200, description = "list of the schedule_groups", body = Vec::<schedule_group::Model> )
        ),
        tag = "schedule_group",
        description = "list all schedule_groups"
)]
#[get("/list-schedule_groups")]
pub async fn list_schedule_groups() -> Result<HttpResponse, Error> {
    let db = utilities::sql_connect().await;
    let response = schedule_group::Entity::find()
        .all(&db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    Ok(HttpResponse::Ok().json(web::Json(response)))
}

#[utoipa::path(
        request_body(
            content = schedule_group::Model,
            example = json!(
                schedule_group::Model::default()
            )
        ),
        responses(
            (
                status = 201,
                description = "the newly created schedule_group",
                content_type = "text/json",
            )
        ),
        tag = "schedule_group",
        description = "create new schedule_group"
)]
#[put("/new-schedule_group")]
pub async fn new_schedule_group(
    schedule_group: web::Json<schedule_group::Model>,
) -> Result<HttpResponse, Error> {
    let db = utilities::sql_connect().await;
    let mut this_schedule_group = schedule_group::ActiveModel {
        id: ActiveValue::NotSet,
        ..Default::default()
    };
    let _ = this_schedule_group.set_from_json(serde_json::json!(schedule_group));

    let result = this_schedule_group
        .insert(&db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    Ok(HttpResponse::Created().json(web::Json(result)))
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
                description = "schedule_group deleted",
                content_type = "text/json",
            )
        ),
        description = "Delete a schedule_group based on its id",
        tag = "schedule_group",
        security(
            ("bearer_token" = [])
        )
)]
#[delete("/delete-schedule_group")]
pub async fn delete_schedule_group(
    req: HttpRequest,
    _user_id: web::Json<utilities::IdRequest>,
) -> Result<HttpResponse, Error> {
    let _ = utilities::token_username(&req);
    //     let db = utilities::sql_connect().await;
    //     let this_schedule_group = schedule_group::Entity::find_by_id(user_id.id)
    //         .one(&db)
    //         .await
    //         .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    //     let this_schedule_group: schedule_group::ActiveModel = match this_schedule_group {
    //         Some(v) => v.into(),
    //         None => return Err(actix_web::error::ErrorNotFound("No ID found")),
    //     };
    //
    //     let _ = this_schedule_group
    //         .delete(&db)
    //         .await
    //         .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;

    Ok(HttpResponse::NotImplemented().finish())
}
