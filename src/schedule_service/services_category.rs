use super::utilities;
use ::entity::category;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use actix_web::{get, put, web, HttpResponse};

#[utoipa::path(
        responses(
            (status = 200, description = "list categories", body = Vec::<category::Model> )
        ),
        tag = "category"
)]
#[get("/list-categories")]
pub async fn list_categories() -> HttpResponse {
    let db = utilities::sql_connect().await;

    match category::Entity::find().all(&db).await {
        Ok(all_categories) => {
            HttpResponse::Ok().json(
            web::Json(all_categories)
            )
        },
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("{:?}",err));
        },
    }
}
#[utoipa::path(
        request_body(
            content = category::Model,
            example = json!( 
                category::Model::default()
            )
        ),
        responses(
            (
                status = 201, 
                description = "the newly created category",
                content_type = "text/json",
            )
        ),
        tag = "category"
)]
#[put("/new-category")]
pub async fn new_category(category: web::Json<category::Model>) -> HttpResponse {
    let db = utilities::sql_connect().await;
    let mut this_category = category::ActiveModel {
        id: ActiveValue::NotSet,
        ..Default::default()
    };
    let _ = this_category.set_from_json(serde_json::json!(category));
    this_category.id = ActiveValue::NotSet;

    let result = this_category.insert(&db).await.unwrap();
    HttpResponse::Created().json(web::Json(result))
}

#[utoipa::path(
        request_body(
            content = category::Model,
            example = json!( 
                category::Model::default()
            )
        ),
        responses(
            (
                status = 200, 
                description = "the updated element",
                content_type = "text/json",
            )
        ),
        tag = "category"
)]
#[put("/update-category")]
pub async fn update_category(category: web::Json<category::Model>) -> HttpResponse {
    let db = utilities::sql_connect().await;
    let this_category: Option<category::Model> = category::Entity::find_by_id(category.id).one(&db).await.unwrap();
    let mut this_category: category::ActiveModel = this_category.unwrap().into();
    let _ = this_category.set_from_json(serde_json::json!(category));

    let result = this_category.update(&db).await.unwrap();
    HttpResponse::Ok().json(web::Json(result))
}
