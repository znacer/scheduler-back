use super::utilities;
use ::entity::user;
use actix_web::{delete, get, put, web, Error, HttpResponse};
use log::debug;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};

#[utoipa::path(
        responses(
            (status = 200, description = "list of the users", body = Vec::<user::Model> )
        ),
        tag = "user",
        description = "list all users"
)]
#[get("/list-users")]
pub async fn list_users() -> Result<HttpResponse, Error> {
    let db = utilities::sql_connect().await;
    let response = user::Entity::find()
        .all(&db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    Ok(HttpResponse::Ok().json(web::Json(response)))
}

#[utoipa::path(
        request_body(
            content = user::Model,
            example = json!(
                user::Model::default()
            )
        ),
        responses(
            (
                status = 201,
                description = "the newly created user",
                content_type = "text/json",
            )
        ),
        tag = "user",
        description = "create new user"
)]
#[put("/new-user")]
pub async fn new_user(user: web::Json<user::Model>) -> Result<HttpResponse, Error> {
    let db = utilities::sql_connect().await;
    let mut this_user = user::ActiveModel {
        id: ActiveValue::NotSet,
        ..Default::default()
    };
    let _ = this_user.set_from_json(serde_json::json!(user));

    let result = this_user
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
                description = "user deleted",
                content_type = "text/json",
            )
        ),
        description = "Delete a user based on its id",
        tag = "user"
)]
#[delete("/delete-user")]
pub async fn delete_user(user_id: web::Json<utilities::IdRequest>) -> Result<HttpResponse, Error> {
    let db = utilities::sql_connect().await;
    let this_user = user::Entity::find_by_id(user_id.id)
        .one(&db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    let this_user: user::ActiveModel = match this_user {
        Some(v) => v.into(),
        None => return Err(actix_web::error::ErrorNotFound("No ID found")),
    };

    let _ = this_user
        .delete(&db)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;

    Ok(HttpResponse::NoContent().into())
}
