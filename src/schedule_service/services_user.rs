use super::utilities;
use ::entity::user;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use actix_web::{get, put, web, HttpResponse};

#[utoipa::path(
        responses(
            (status = 200, description = "list users", body = Vec::<user::Model> )
        ),
        tag = "user"
)]
#[get("/list-users")]
pub async fn list_users() -> HttpResponse {
    let db = utilities::sql_connect().await;

    match user::Entity::find().all(&db).await {
        Ok(all_users) => {
            HttpResponse::Ok().json(
            web::Json(all_users)
            )
        },
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("{:?}",err));
        },
    }
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
        tag = "user"
)]
#[put("/new-user")]
pub async fn new_user(user: web::Json<user::Model>) -> HttpResponse {
    let db = utilities::sql_connect().await;
    let mut this_user = user::ActiveModel {
        id: ActiveValue::NotSet,
        ..Default::default()
    };
    let _ = this_user.set_from_json(serde_json::json!(user));

    let result = this_user.insert(&db).await.unwrap();
    HttpResponse::Created().json(web::Json(result))
}
