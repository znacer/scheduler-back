use super::palette::Palette;
use chrono::{DateTime, Duration, Local, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;
use uuid::Uuid;

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
impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TaskData {
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
pub struct TaskDataFront {
    pub id: String,
    pub startDate: DateTime<Utc>,
    pub endDate: DateTime<Utc>,
    pub occupancy: u32,
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub bgColor: String,
}
impl TaskDataFront {
    pub fn new(input: TaskData) -> Self {
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
pub struct SchedulerLabel {
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
pub struct SchedulerDataResponse {
    id: String,
    label: SchedulerLabel,
    data: Vec<TaskDataFront>,
}
impl SchedulerDataResponse {
    pub fn random_new(name: &str, color: Palette) -> Self {
        let mut tasks = vec![];
        for _ in 0..15 {
            let obj = TaskData::random_new(color.rgb());
            let obj = TaskDataFront::new(obj);
            tasks.push(obj.clone());
        }
        let mut out = SchedulerDataResponse::default();
        out.id = String::from(name);
        out.label.title = String::from(name);
        out.data = tasks.clone();
        out
    }
}
