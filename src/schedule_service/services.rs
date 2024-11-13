use super::models::{Task, Schedule};
use super::utilities;

use actix_web::http::header::ContentType;
use actix_web::{get, put, web, HttpResponse};
use sqlx::PgConnection;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        list_tasks,
        list_schedules,
        new_schedule,
        new_task,
        update_task,
        update_schedule,
        create_tables,
    ),
    components(schemas(
            Schedule,
            Task
    ))
)]
pub struct ApiDocScheduler;

#[utoipa::path(
        responses(
            (
                status = 201, 
                description = "Creates tables if did not exists", 
                content_type = "text/plain"
            )
        )
)]
#[put("/create_tables")]
pub async fn create_tables() -> HttpResponse {
    match utilities::create_tables().await {
        Ok(_) => {}
        Err(err) => {
            dbg!("hello");
            return HttpResponse::InternalServerError()
                .content_type(ContentType::plaintext())
                .body(format!("{:?}", err));
        }
    }
    HttpResponse::Created()
        .content_type(ContentType::plaintext())
        .body("Tables exists or have been created")
}

#[utoipa::path(
        request_body(
            content=Task,
            example=json!( Task::example())
        ),
        responses(
            (
                status = 201, 
                description = "Create or update a task",
                content_type = "text/plain",
            ),
        )
)]
#[put("/update-task")]
pub async fn update_task(task_data: web::Json<Task>) -> HttpResponse {
    let conn: PgConnection = utilities::sql_connect().await;
    match utilities::update_task(conn, &task_data.into_inner()).await {
        Ok(v) => {
            return HttpResponse::Created().json(v);
        }
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("{:?}",err));
        }
    };
}

#[utoipa::path(
        request_body(
            content = Schedule,
            example = json!( 
                Schedule::default()
            )
        ),
        responses(
            (
                status = 201, 
                description = "Create or update schedule with its label and tasks",
                content_type = "text/json",
            )
        )
)]
#[put("/update-schedule")]
pub async fn update_schedule(schedule: web::Json<Schedule>) -> HttpResponse {
    let conn: PgConnection = utilities::sql_connect().await;
    match utilities::update_schedule(conn, &schedule).await {
        Ok(v) => {
            return HttpResponse::Created().json(v);
        }
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("{:?}",err));
        }
    };
}

#[utoipa::path(
        request_body(
            content = Schedule,
            example = json!( 
                Schedule::default()
            )
        ),
        responses(
            (
                status = 201, 
                description = "id of the newly created schedule",
                content_type = "text/json",
            )
        )
)]
#[put("/new-schedule")]
pub async fn new_schedule(new_schedule: web::Json<Schedule>) -> HttpResponse {
    let conn: PgConnection = utilities::sql_connect().await;
    match utilities::new_schedule(conn, &new_schedule).await {
        Ok(v) => {
            return HttpResponse::Created().json(v);
        }
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("{:?}",err));
        }
    };
}

#[utoipa::path(
        request_body(
            content = Task,
            example = json!( 
                Task::default()
            )
        ),
        responses(
            (
                status = 201, 
                description = "Create a new task",
                content_type = "text/json",
            )
        )
)]
#[put("/new-task")]
pub async fn new_task(task: web::Json<Task>) -> HttpResponse {
    let conn: PgConnection = utilities::sql_connect().await;
    match utilities::new_task(conn, &task).await {
        Ok(v) => {
            return HttpResponse::Created().json(v);
        }
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("{:?}",err));
        }
   };
}


#[utoipa::path(
        responses(
            (status = 200, description = "Fetch all schedules", body = [Vec::<Task>], example= json!(vec![Task::default()]))
        )
)]
#[get("/list-tasks")]
pub async fn list_tasks() -> web::Json<Vec<Task>> {
    let query = "SELECT * FROM task";
    let mut conn = utilities::sql_connect().await;
    let schedule_db = sqlx::query_as(query).fetch_all(&mut conn).await.unwrap();

    web::Json(schedule_db)
}

#[utoipa::path(
        responses(
            (status = 200, description = "list schedules", body = [Vec::<Schedule>] )
        )
)]
#[get("/list-schedules")]
pub async fn list_schedules() -> web::Json<Vec<Schedule>>{
    let query = "SELECT * FROM schedule";
    let mut conn = utilities::sql_connect().await;
    let schedule_db = sqlx::query_as(query).fetch_all(&mut conn).await.unwrap();

    web::Json(schedule_db)
}
