use super::utilities;
use ::entity::group;
use actix_web::{delete, get, put, web, Error, HttpResponse};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};

#[utoipa::path(
        responses(
            (status = 200, description = "list groups", body = Vec::<group::Model> )
        ),
        tag = "group"
)]
#[get("/list-groups")]
pub async fn list_groups() -> Result<HttpResponse, Error> {
    let db = utilities::sql_connect().await;
    let response = group::Entity::find()
        .all(&db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    Ok(HttpResponse::Ok().json(web::Json(response)))
}

#[utoipa::path(
        request_body(
            content = group::Model,
            example = json!(
                group::Model::default()
            )
        ),
        responses(
            (
                status = 201,
                description = "the newly created group",
                content_type = "text/json",
            )
        ),
        tag = "group"
)]
#[put("/new-group")]
pub async fn new_group(group: web::Json<group::Model>) -> Result<HttpResponse, Error> {
    let db = utilities::sql_connect().await;
    let mut this_group = group::ActiveModel {
        id: ActiveValue::NotSet,
        ..Default::default()
    };
    let _ = this_group.set_from_json(serde_json::json!(group));

    let result = this_group
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
                description = "group deleted",
                content_type = "text/json",
            )
        ),
        description = "Delete a group based on its id",
        tag = "group"
)]
#[delete("/delete-group")]
pub async fn delete_group(
    group_id: web::Json<utilities::IdRequest>,
) -> Result<HttpResponse, Error> {
    let db = utilities::sql_connect().await;
    let this_group = group::Entity::find_by_id(group_id.id)
        .one(&db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    let this_group: group::ActiveModel = match this_group {
        Some(v) => v.into(),
        None => return Err(actix_web::error::ErrorNotFound("No ID found")),
    };

    let _ = this_group
        .delete(&db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;

    Ok(HttpResponse::NoContent().into())
}
