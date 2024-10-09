use core::panic;

use super::models::{SchedulerDataResponse, SchedulerLabel, TaskData, TaskDataFront, NewLabelRequest};
use super::palette::Palette;
use super::utilities;
use actix_web::http::header::ContentType;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::PgConnection;
use utoipa::{OpenApi, ToSchema};
use uuid::{uuid, Uuid};

#[derive(OpenApi)]
#[openapi(
    paths(
        fetch_all, 
        fetch_schedule, 
        update_task, 
        update_schedule, 
        test, 
        create_tables, 
        list_schedules,
        new_schedule,
        ),
    components(schemas(
        SchedulerDataResponse,
        TaskDataFront,
        SchedulerLabel,
        FetchScheduleRequest,
        ScheduleListResponse,
        NewLabelRequest,
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

#[utoipa::path(
        request_body(
            content=TaskDataFront,
            example=json!( TaskDataFront::example())
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
pub async fn update_task(task_data: web::Json<TaskDataFront>) -> impl Responder {
    let mut conn: PgConnection = utilities::sql_connect().await;
    let _ = utilities::update_task(&mut conn, &task_data.into_inner()).await;

    HttpResponse::Created().body("Task data created successfully")
}

#[utoipa::path(
        request_body(
            content = SchedulerDataResponse,
            example = json!( 
                SchedulerDataResponse::random_new(Uuid::new_v4(), "Example", Palette::Blue)
            )
        ),
        responses(
            (
                status = 201, 
                description = "Create or update schedule with its label and tasks",
                content_type = "text/plain",
            )
        )
)]
#[put("/update-schedule")]
pub async fn update_schedule(schedule: web::Json<SchedulerDataResponse>) -> HttpResponse {
    let mut conn: PgConnection = utilities::sql_connect().await;
    match utilities::update_schedule(&mut conn, &schedule).await {
        Ok(_) => {}
        Err(err) => {
            panic!("{}", err);
        }
    };

    HttpResponse::Created().body("Task data created successfully")
}

#[utoipa::path(
        request_body(
            content = NewLabelRequest,
            example = json!( 
                NewLabelRequest::default()
            )
        ),
        responses(
            (
                status = 201, 
                description = "Create a new schedule",
                content_type = "text/plain",
            )
        )
)]
#[put("/new-schedule")]
pub async fn new_schedule(label: web::Json<NewLabelRequest>) -> HttpResponse {
    let conn: PgConnection = utilities::sql_connect().await;
    match utilities::new_schedule(conn, &label).await {
        Ok(_) => {}
        Err(err) => {
            panic!("{}", err);
        }
    };

    HttpResponse::Created().body("Task data created successfully")
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, ToSchema, FromRow)]
struct FetchScheduleRequest {
    id: Uuid,
}
#[utoipa::path(
        request_body(
            content = FetchScheduleRequest,
            example = json!(FetchScheduleRequest {id: uuid!("5bd8f32d-5e63-47e1-a6b7-59848b93a507")})
        ),
        responses(
            (
                status = 200, 
                description = "Fetch one schedule defined by its id", 
                body = SchedulerDataResponse, 
                example= json!(SchedulerDataResponse::random_new(uuid!("a0479339-beea-4444-a0b0-03d4376f659e"), "Example", Palette::Blue))
             )
        )
)]
#[post("/fetch-schedule")]
pub async fn fetch_schedule(
    project_id: web::Json<FetchScheduleRequest>,
) -> web::Json<SchedulerDataResponse> {
    let conn = utilities::sql_connect().await;
    let (schedule_db, tasks, label) = utilities::fetch_schedule(conn, &project_id.id)
        .await
        .unwrap();
    let schedule = SchedulerDataResponse::parse_from_db(schedule_db, tasks, label);

    web::Json(schedule)
}

#[utoipa::path(
        responses(
            (status = 200, description = "Fetch all schedules", body = [Vec::<SchedulerDataResponse>], example= json!(vec![TaskDataFront::new(TaskData::default())]))
        )
)]
#[get("/fetch-all")]
pub async fn fetch_all() -> web::Json<Vec<SchedulerDataResponse>> {
    let mut conn = utilities::sql_connect().await;
    let query = "SELECT id FROM schedule";
    let schedule_ids: Vec<FetchScheduleRequest> =
        sqlx::query_as(query).fetch_all(&mut conn).await.unwrap();
    let mut all_schedules: Vec<SchedulerDataResponse> = vec![];
    for id in schedule_ids.iter() {
        let conn = utilities::sql_connect().await;
        let (schedule_db, tasks, label) = utilities::fetch_schedule(conn, &id.id).await.unwrap();
        let schedule = SchedulerDataResponse::parse_from_db(schedule_db, tasks, label);
        all_schedules.push(schedule);
    }

    web::Json(all_schedules)
}

#[derive(Debug, Serialize, Deserialize, ToSchema, FromRow)]
struct ScheduleListResponse {
    id: Uuid,
    title: String,
    subtitle: String,
    icon: String,
}
#[utoipa::path(
        responses(
            (status = 200, description = "list schedules", body = [Vec::<ScheduleListResponse>] )
        )
)]
#[get("/list-schedules")]
pub async fn list_schedules() -> impl Responder {
    let mut conn = utilities::sql_connect().await;
    let query = "SELECT id, title, subtitle, icon FROM public.schedule
        INNER JOIN public.label
        ON public.label.label_id = public.schedule.label_id";
    let schedule_list: Vec<ScheduleListResponse> = sqlx::query_as(query).fetch_all(&mut conn).await.unwrap();


    web::Json(schedule_list)
}

#[utoipa::path(
        responses(
            (status = 200, description = "Give some example tasks", body = [Vec::<SchedulerDataResponse>], example= json!(vec![TaskDataFront::new(TaskData::default())]))
        )
)]
#[get("/test")]
pub async fn test() -> impl Responder {
    let out = vec![
        SchedulerDataResponse::random_new(
            uuid!("a0479339-beea-4444-a0b0-03d4376f659e"),
            "Ressource 1",
            Palette::Blue,
        ),
        SchedulerDataResponse::random_new(
            uuid!("11235c9d-05cf-46d7-8cb0-df498c05800d"),
            "Ressource 2",
            Palette::Pink,
        ),
        SchedulerDataResponse::random_new(
            uuid!("d79a3708-848b-4c0c-884e-4b547b130b5a"),
            "Ressource 3",
            Palette::RANDOM,
        ),
    ];

    web::Json(out)
}
