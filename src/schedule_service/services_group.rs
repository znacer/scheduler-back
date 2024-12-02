use super::utilities;
use ::entity::group;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use actix_web::{get, put, web, HttpResponse};

#[utoipa::path(
        responses(
            (status = 200, description = "list groups", body = Vec::<group::Model> )
        ),
        tag = "group"
)]
#[get("/list-groups")]
pub async fn list_groups() -> HttpResponse {
    let db = utilities::sql_connect().await;

    match group::Entity::find().all(&db).await {
        Ok(all_groups) => {
            HttpResponse::Ok().json(
            web::Json(all_groups)
            )
        },
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("{:?}",err));
        },
    }
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
pub async fn new_group(group: web::Json<group::Model>) -> HttpResponse {
    let db = utilities::sql_connect().await;
    let mut this_group = group::ActiveModel {
        id: ActiveValue::NotSet,
        ..Default::default()
    };
    let _ = this_group.set_from_json(serde_json::json!(group));

    let result = this_group.insert(&db).await.unwrap();
    HttpResponse::Created().json(web::Json(result))
}
