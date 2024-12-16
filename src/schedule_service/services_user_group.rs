use super::utilities;
use crate::errors::ProjectError as Error;
use ::entity::user_group;
use actix_web::{delete, get, put, web, HttpRequest, HttpResponse};
use entity::user;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DbErr, EntityTrait, ModelTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[utoipa::path(
        responses(
            (status = 200, description = "list of the user_groups", body = Vec::<user_group::Model> )
        ),
        tag = "user_group",
        description = "list all user_groups"
)]
#[get("/list-user-groups")]
pub async fn list_user_groups() -> Result<HttpResponse, Error> {
    let db = utilities::sql_connect().await;
    let response = user_group::Entity::find().all(&db).await?;
    Ok(HttpResponse::Ok().json(web::Json(response)))
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GroupResponse {
    pub id: i32,
    pub name: String,
    pub admin: bool,
}
#[utoipa::path(
        responses(
            (status = 200, description = "list of the user's groups", body = Vec::<GroupResponse> )
        ),
        tag = "user_group",
        description = "list all the groups of the user",
        security(
            ("bearer_token" = [])
        )
)]
#[get("/list-my-groups")]
pub async fn list_my_groups(req: HttpRequest) -> Result<HttpResponse, Error> {
    let username = utilities::token_username(&req)?;
    let db = utilities::sql_connect().await;
    let user_elt: user::Model = user::Entity::find()
        .filter(user::Column::Name.eq(&username))
        .one(&db)
        .await?
        .unwrap();

    let groups = user_elt
        .find_linked(::entity::links::UserToGroup)
        .all(&db)
        .await
        .unwrap();

    let groups_admin = user_elt
        .find_linked(::entity::links::UserToGroupAdmin)
        .all(&db)
        .await
        .unwrap();

    let response: Vec<GroupResponse> = groups
        .iter()
        .map(|group| {
            let admin: bool = groups_admin.contains(group);
            GroupResponse {
                id: group.id,
                name: group.name.clone(),
                admin,
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(web::Json(response)))
}

#[utoipa::path(
        responses(
            (status = 200, description = "list of the user's groups where user is admin", body = Vec::<user_group::Model> )
        ),
        tag = "user_group",
        description = "list all the groups of the user",
        security(
            ("bearer_token" = [])
        )
)]
#[get("/list-my-groups-admin")]
pub async fn list_my_groups_admin(req: HttpRequest) -> Result<HttpResponse, Error> {
    let username = utilities::token_username(&req)?;
    let db = utilities::sql_connect().await;
    let user_elt: user::Model = user::Entity::find()
        .filter(user::Column::Name.eq(&username))
        .one(&db)
        .await?
        .unwrap();

    let response = user_elt
        .find_linked(::entity::links::UserToGroupAdmin)
        .all(&db)
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(web::Json(response)))
}

#[utoipa::path(
        request_body(
            content = user_group::Model,
            example = json!(
                user_group::Model::default()
            )
        ),
        responses(
            (
                status = 201,
                description = "the newly created user_group",
                content_type = "text/json",
            )
        ),
        tag = "user_group",
        description = "create new user_group"
)]
#[put("/new-user-group")]
pub async fn new_user_group(
    user_group: web::Json<user_group::Model>,
) -> Result<HttpResponse, Error> {
    let result = fn_new_user_group(&user_group.into_inner()).await?;
    Ok(HttpResponse::Created().json(web::Json(result)))
}

pub async fn fn_new_user_group(user_group: &user_group::Model) -> Result<user_group::Model, Error> {
    let db = utilities::sql_connect().await;
    //check if not already exists
    let exists = user_group::Entity::find()
        .filter(
            Condition::all()
                .add(user_group::Column::UserId.eq(user_group.user_id))
                .add(user_group::Column::GroupId.eq(user_group.group_id)),
        )
        .one(&db)
        .await?;
    match exists {
        Some(_) => {
            return Err(Error::DbErr(DbErr::Custom(
                "user_group already exists".to_string(),
            )))
        }
        None => {
            let mut this_user_group: user_group::ActiveModel = user_group.clone().into();
            this_user_group.not_set(user_group::Column::Id);
            let result = this_user_group.insert(&db).await?;
            Ok(result)
        }
    }
}

#[utoipa::path(
        request_body(
            content = String,
        ),
        responses(
            (
                status = 204,
                description = "user_group deleted",
                content_type = "text/json",
            )
        ),
        description = "Delete a user_group based on its id",
        tag = "user_group",
        security(
            ("bearer_token" = [])
        )

)]
#[delete("/delete-user-group")]
pub async fn delete_user_group(req: HttpRequest, group_id: String) -> Result<HttpResponse, Error> {
    let username = utilities::token_username(&req)?;
    let db = utilities::sql_connect().await;
    let user_elt: user::Model = user::Entity::find()
        .filter(user::Column::Name.eq(&username))
        .one(&db)
        .await?
        .unwrap();
    let to_remove: Option<user_group::Model> = user_group::Entity::find()
        .filter(
            Condition::all()
                .add(user_group::Column::GroupId.eq(group_id))
                .add(user_group::Column::UserId.eq(user_elt.id)),
        )
        .one(&db)
        .await?;
    let _ = match to_remove {
        Some(v) => {
            // remove from user_group first
            v.delete(&db).await?
        }
        None => return Ok(actix_web::error::ErrorNotFound("No ID found").into()),
    };

    Ok(HttpResponse::NoContent().into())
}
