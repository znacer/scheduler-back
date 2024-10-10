use super::palette::Palette;
use chrono::{DateTime, Duration, Local, TimeZone, Utc};
use itertools::Itertools;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Decode};
use std::fmt;
use utoipa::ToSchema;
use uuid::{uuid, Uuid};

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
    id: Uuid,
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
        let id = Uuid::new_v4();
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
    pub id: Uuid,
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
    pub fn example() -> Self {
        // TaskDataFront::new(TaskData::random_new(Palette::Blue.rgb()))
        TaskDataFront {
            bgColor: "rgb(31,119,180)".to_string(),
            description: "".to_string(),
            endDate: chrono::DateTime::parse_from_rfc3339("2024-09-25T22:37:48.051524268Z")
                .unwrap()
                .into(),
            id: uuid!("5bd8f32d-5e63-47e1-a6b7-59848b93a507"),
            occupancy: 0,
            startDate: chrono::DateTime::parse_from_rfc3339("2024-09-25T18:37:48.051524268Z")
                .unwrap()
                .into(),
            subtitle: "subtitle".to_string(),
            title: "title 98".to_string(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SchedulerLabel {
    pub(super) id: Uuid,
    pub(super) icon: String,
    pub(super) title: String,
    pub(super) subtitle: String,
}
impl Default for SchedulerLabel {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            icon: String::from(""),
            title: String::from("Name"),
            subtitle: String::from("More details"),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone, Default, ToSchema)]
pub struct SchedulerDataResponse {
    pub(super) id: Uuid,
    pub(super) label: SchedulerLabel,
    pub(super) data: Vec<TaskDataFront>,
}
impl SchedulerDataResponse {
    pub fn random_new(id: Uuid, name: &str, color: Palette) -> Self {
        let mut tasks = vec![];
        for _ in 0..20 {
            let obj = TaskData::random_new(color.rgb());
            let obj = TaskDataFront::new(obj);
            tasks.push(obj.clone());
        }
        let mut out = SchedulerDataResponse::default();
        out.id = id;
        out.label.title = String::from(name);
        out.data = tasks.clone();
        out
    }

    pub fn parse_from_db(
        schedule_db: ScheduleDataModel,
        tasks_db: Vec<TaskDataModel>,
        label_db: LabelDataModel,
    ) -> Self {
        Self {
            id: schedule_db.id,
            label: SchedulerLabel {
                id: label_db.label_id,
                title: label_db.title,
                subtitle: label_db.subtitle,
                icon: label_db.icon,
            },
            data: tasks_db
                .iter()
                .map(|task_elt| TaskDataFront {
                    id: task_elt.task_id,
                    title: task_elt.title.clone(),
                    subtitle: task_elt.subtitle.clone(),
                    occupancy: task_elt.occupancy as u32,
                    description: task_elt.description.clone(),
                    startDate: chrono::Utc.timestamp_opt(task_elt.start_date, 0).unwrap(),
                    endDate: chrono::Utc.timestamp_opt(task_elt.end_date, 0).unwrap(),
                    bgColor: Palette::random().rgb().to_string(),
                    ..Default::default()
                })
                .collect_vec(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, ToSchema, FromRow, Decode)]
pub struct ScheduleDataModel {
    pub id: Uuid,
    pub tasks: Vec<Uuid>,
    pub label_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, ToSchema, FromRow, Decode)]
pub struct TaskDataModel {
    pub task_id: Uuid,
    pub start_date: i64,
    pub end_date: i64,
    pub occupancy: i32,
    pub title: String,
    pub subtitle: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, ToSchema, FromRow, Decode)]
pub struct LabelDataModel {
    pub label_id: Uuid,
    pub icon: String,
    pub title: String,
    pub subtitle: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, ToSchema, FromRow, Decode)]
pub struct NewLabelRequest {
    pub icon: String,
    pub title: String,
    pub subtitle: String,
}
