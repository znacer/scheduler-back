use super::models::{SchedulerDataResponse, SchedulerLabel, TaskData, TaskDataFront};
use super::palette::Palette;
use super::utilities;
use actix_web::http::header::ContentType;
use actix_web::{get, put, web, HttpResponse, Responder};
use sqlx::{Executor, PgConnection};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(update_task, test, create_tables),
    components(schemas(SchedulerDataResponse, TaskDataFront, SchedulerLabel))
)]
pub struct ApiDocScheduler;

#[utoipa::path(
        responses(
            (status = 201, description = "Creates tables if did not exists")
        )
)]
#[put("/create_tables")]
pub async fn create_tables() -> impl Responder {
    match utilities::create_tables().await {
        Ok(_) => {}
        Err(err) => {
            return HttpResponse::InternalServerError()
                .content_type(ContentType::plaintext())
                .body(format!("{:?}", err));
        }
    }
    HttpResponse::Created()
        .content_type(ContentType::plaintext())
        .body("Tables exists or have been created")
}

//TODO: produce a working example
#[utoipa::path(
        responses(
            (status = 200, 
             description = "Create or update a task",
             example=json!(TaskDataFront::new(TaskData::random_new(Color::Blue)))
             ),
        )
)]
#[put("/update-task")]
pub async fn update_task(task_data: web::Json<TaskDataFront>) -> impl Responder {
    let mut conn: PgConnection = utilities::sql_connect().await;
    let query = format!(
        "INSERT INTO task (task_id, start_date, end_date, occupancy, title, subtitle, description)
        VALUES ('{}', '{}', '{}', {}, '{}', '{}', '{}', '{}')",
        task_data.id,
        task_data.startDate,
        task_data.endDate,
        task_data.occupancy,
        task_data.title,
        task_data.subtitle,
        task_data.bgColor,
        task_data.description
    );
    println!("{}", query);
    conn.execute(query.as_str()).await.unwrap();

    HttpResponse::Created().body("Task data created successfully")
}

#[utoipa::path(
        responses(
            (status = 200, description = "Give some example tasks", body = [Vec::<SchedulerDataResponse>], example= json!(vec![TaskDataFront::new(TaskData::default())]))
        )
)]
#[get("/test")]
pub async fn test() -> impl Responder {
    let out = vec![
        SchedulerDataResponse::random_new("Ressource 1", Palette::Blue),
        SchedulerDataResponse::random_new("Ressource 2", Palette::Pink),
        SchedulerDataResponse::random_new("Ressource 3", Palette::Purple),
    ];

    web::Json(out)
}
