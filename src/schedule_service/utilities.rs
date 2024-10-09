use std::fs;

use itertools::Itertools;
use sqlx::{Connection, Executor, PgConnection};
use uuid::Uuid;

use super::models::{
    LabelDataModel, NewLabelRequest, ScheduleDataModel, SchedulerDataResponse, SchedulerLabel,
    TaskDataFront, TaskDataModel,
};

pub(super) async fn sql_connect() -> PgConnection {
    PgConnection::connect("postgres://admin:secretpassword@localhost:5432/scheduler")
        .await
        .unwrap()
}

pub(super) async fn create_tables() -> Result<(), sqlx::Error> {
    let mut conn: PgConnection = sql_connect().await;

    let queries_path = vec![
        "sql_queries/create_table_task.sql",
        "sql_queries/create_table_label.sql",
        "sql_queries/create_table_schedule.sql",
    ];
    for query_path in queries_path.iter() {
        let query = fs::read_to_string(query_path)?;
        conn.execute(query.as_str()).await?;
    }
    Ok(())
}

pub(super) async fn update_task(
    conn: &mut PgConnection,
    task_data: &TaskDataFront,
) -> Result<(), sqlx::Error> {
    let query = format!(
        "INSERT INTO task (task_id, start_date, end_date, occupancy, title, subtitle, description)
        VALUES ('{}', '{}', {}, '{}', '{}', '{}', '{}')
            ON CONFLICT (task_id)
            DO UPDATE SET
                start_date = EXCLUDED.start_date,
                end_date = EXCLUDED.end_date,
                occupancy = EXCLUDED.occupancy,
                title = EXCLUDED.title,
                subtitle = EXCLUDED.subtitle,
                description = EXCLUDED.description
        ;",
        task_data.id,
        task_data.startDate.timestamp(),
        task_data.endDate.timestamp(),
        task_data.occupancy,
        task_data.title,
        task_data.subtitle,
        task_data.description
    );
    conn.execute(query.as_str()).await?;
    Ok(())
}

pub(super) async fn update_label(
    conn: &mut PgConnection,
    label_data: &SchedulerLabel,
) -> Result<(), sqlx::Error> {
    let query = format!(
        // "INSERT INTO label (label_id, icon, subtitle, title)
        // VALUES ('{}', '{}', {}, '{}')",
        "INSERT INTO label (label_id, icon, subtitle, title)
            VALUES ('{}', '{}', '{}', '{}')
            ON CONFLICT (label_id)
            DO UPDATE SET
                icon = EXCLUDED.icon,
                subtitle = EXCLUDED.subtitle,
                title = EXCLUDED.title;",
        label_data.id, label_data.icon, label_data.subtitle, label_data.title
    );
    println!("label_data: {:?}", query);
    conn.execute(query.as_str()).await?;
    Ok(())
}

pub(super) async fn update_schedule(
    conn: &mut PgConnection,
    schedule: &SchedulerDataResponse,
) -> Result<(), sqlx::Error> {
    for task in schedule.data.iter() {
        update_task(conn, &task).await?;
    }
    update_label(conn, &schedule.label).await?;

    let task_list: String = schedule
        .data
        .iter()
        .map(|task| format!("{}", &task.id.to_string()))
        .join(", ");
    let task_list = "{".to_string() + task_list.as_str() + "}";
    let query = format!(
        "INSERT INTO schedule (id, tasks, label_id)
        VALUES ('{}', '{}', '{}')
        ON CONFLICT (id)
        DO UPDATE SET
                tasks = EXCLUDED.tasks,
                label_id = EXCLUDED.label_id;",
        schedule.id, task_list, schedule.label.id
    );
    println!("query: {}", query);
    conn.execute(query.as_str()).await?;
    Ok(())
}

pub(super) async fn fetch_schedule(
    mut conn: PgConnection,
    schedule_id: &Uuid,
) -> Result<(ScheduleDataModel, Vec<TaskDataModel>, LabelDataModel), sqlx::Error> {
    let query = format!(
        "SELECT * FROM schedule WHERE id = '{}'",
        schedule_id.to_string()
    );
    let schedule_db: ScheduleDataModel =
        sqlx::query_as(query.as_str()).fetch_one(&mut conn).await?;

    let task_ids = schedule_db
        .tasks
        .iter()
        .map(|task| format!("'{}'", &task.to_string()))
        .join(", ");
    let mut tasks: Vec<TaskDataModel> = vec![];
    if task_ids.len() > 0 {
        let task_ids = "(".to_string() + task_ids.as_str() + ")";
        let query = format!("SELECT * FROM task WHERE task_id IN {}", task_ids);
        tasks = sqlx::query_as(query.as_str()).fetch_all(&mut conn).await?;
    }

    let query = format!(
        "SELECT * FROM label WHERE label_id = '{}'",
        schedule_db.label_id
    );
    let label: LabelDataModel = sqlx::query_as(query.as_str()).fetch_one(&mut conn).await?;

    Ok((schedule_db, tasks, label))
}

pub(super) async fn new_schedule(
    mut conn: PgConnection,
    new_label: &NewLabelRequest,
) -> Result<(), sqlx::Error> {
    let label_uuid = uuid::Uuid::new_v4();
    let query = format!(
        "INSERT INTO label (label_id, title, subtitle, icon)
        VALUES ('{}', '{}', '{}', '{}')
        ;",
        label_uuid, new_label.title, new_label.subtitle, new_label.icon
    );
    conn.execute(query.as_str()).await?;

    let query = format!(
        "INSERT INTO schedule (id, tasks, label_id)
        VALUES ('{}', '{}', '{}')
        ;",
        uuid::Uuid::new_v4(),
        "{}",
        label_uuid
    );
    conn.execute(query.as_str()).await?;
    Ok(())
}
