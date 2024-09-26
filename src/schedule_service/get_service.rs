use std::fmt;

use actix_web::{get, web, Responder};
use chrono::{DateTime, Duration, Local, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

#[derive(OpenApi)]
#[openapi(
    paths(test),
    components(schemas(SchedulerDataResponse, TaskDataResponse, SchedulerLabel))
)]
pub struct ApiDocGetExample;

#[utoipa::path(
        responses(
            (status = 200, description = "Give some example tasks", body = [Vec::<SchedulerDataResponse>], example= json!(vec![TaskDataResponse::new(TaskData::default())]))
        )
)]
#[get("/test")]
pub async fn test() -> impl Responder {
    let out = vec![
        SchedulerDataResponse::random_new("1", Palette::Blue),
        SchedulerDataResponse::random_new("2", Palette::Pink),
        SchedulerDataResponse::random_new("3", Palette::Purple),
    ];

    web::Json(out)
}

// Structures

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({},{},{})", self.red, self.green, self.blue)
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct TaskData {
    id: String,
    startDate: DateTime<Utc>,
    endDate: DateTime<Utc>,
    occupancy: u32,
    title: String,
    subtitle: String,
    description: String,
    bgColor: Color,
}

impl TaskData {
    fn random_new(bg_color: Color) -> Self {
        let id = Uuid::new_v4().to_string();
        let mut rng = rand::thread_rng();
        let random_hours = rng.gen_range(0..720);

        let start_date: DateTime<Utc> = Local::now().into();
        let start_date = start_date - Duration::hours(random_hours);

        let random_hours = rng.gen_range(1..12);
        let end_date: DateTime<Utc> = start_date + Duration::hours(random_hours);

        Self {
            id,
            startDate: start_date,
            endDate: end_date,
            title: format!("title {}", rng.gen_range(1..100)),
            subtitle: format!("subtitle"),
            bgColor: bg_color,
            ..Self::default()
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone, Default, ToSchema)]
struct TaskDataResponse {
    id: String,
    startDate: DateTime<Utc>,
    endDate: DateTime<Utc>,
    occupancy: u32,
    title: String,
    subtitle: String,
    description: String,
    bgColor: String,
}

impl TaskDataResponse {
    fn new(input: TaskData) -> Self {
        Self {
            id: input.id,
            startDate: input.startDate,
            endDate: input.endDate,
            occupancy: input.occupancy,
            title: input.title,
            subtitle: input.subtitle,
            description: input.description,
            bgColor: format!("{}", input.bgColor),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
struct SchedulerLabel {
    icon: String,
    title: String,
    subtitle: String,
}
impl Default for SchedulerLabel {
    fn default() -> Self {
        Self {
            icon: String::from(""),
            title: String::from("Name"),
            subtitle: String::from("More details"),
        }
    }
}
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone, Default, ToSchema)]
struct SchedulerDataResponse {
    id: String,
    label: SchedulerLabel,
    data: Vec<TaskDataResponse>,
}

impl SchedulerDataResponse {
    fn random_new(name: &str, color: Palette) -> Self {
        let mut tasks = vec![];
        for _ in 0..15 {
            let obj = TaskData::random_new(color.rgb());
            let obj = TaskDataResponse::new(obj);
            tasks.push(obj.clone());
        }
        let mut out = SchedulerDataResponse::default();
        out.id = String::from(name);
        out.label.title = String::from(name);
        out.data = tasks.clone();
        out
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Palette {
    Blue,
    Orange,
    Green,
    Red,
    Purple,
    Brown,
    Pink,
    Grey,
    Turquoise,
    Yellow,
}

impl Palette {
    pub fn rgb(&self) -> Color {
        let c = match self {
            Palette::Blue => (31, 119, 180),
            Palette::Orange => (255, 127, 14),
            Palette::Green => (44, 160, 44),
            Palette::Red => (214, 39, 40),
            Palette::Purple => (148, 103, 189),
            Palette::Brown => (140, 86, 75),
            Palette::Pink => (227, 119, 194),
            Palette::Grey => (127, 127, 127),
            Palette::Turquoise => (23, 190, 207),
            Palette::Yellow => (255, 187, 33),
        };
        Color {
            red: c.0,
            green: c.1,
            blue: c.2,
        }
    }
}
