use sqlx::{Connection, Executor, PgConnection};
use std::{env, fs};

use super::models::{Schedule, Task};

pub(super) async fn sql_connect() -> PgConnection {
    let username = match env::var_os("POSTGRES_USER") {
        Some(val) => match val.into_string() {
            Ok(val) => val,
            _ => "postgres".to_string(),
        },
        _ => "postgres".to_string(),
    };

    let password = match env::var_os("POSTGRES_PASSWORD") {
        Some(val) => match val.into_string() {
            Ok(val) => val,
            _ => "mysecretpassword".to_string(),
        },
        _ => "mysecretpassword".to_string(),
    };

    let address = match env::var_os("POSTGRES_HOST") {
        Some(val) => match val.into_string() {
            Ok(val) => val,
            _ => "localhost".to_string(),
        },
        _ => "localhost".to_string(),
    };

    let port = match env::var_os("POSTGRES_PORT") {
        Some(val) => match val.into_string() {
            Ok(val) => val,
            _ => "5432".to_string(),
        },
        _ => "5432".to_string(),
    };

    let schema = match env::var_os("POSTGRES_DATABASE") {
        Some(val) => match val.into_string() {
            Ok(val) => val,
            _ => "public".to_string(),
        },
        _ => "public".to_string(),
    };

    let connection_string = format!("postgress://{username}:{password}@{address}:{port}/{schema}");
    println!("{}", connection_string);
    PgConnection::connect(connection_string.as_str())
        .await
        .unwrap()
}

pub(super) async fn create_tables() -> Result<(), sqlx::Error> {
    let mut conn: PgConnection = sql_connect().await;

    println!("create_tables");
    let queries_path = vec![
        "sql_queries/create_table_schedule.sql",
        "sql_queries/create_table_task.sql",
        "sql_queries/create_table_users.sql",
    ];
    for query_path in queries_path.iter() {
        let query = fs::read_to_string(query_path)?;
        conn.execute(query.as_str()).await?;
    }
    Ok(())
}

pub(super) async fn update_task(
    mut conn: PgConnection,
    task_data: &Task,
) -> Result<Task, sqlx::Error> {
    let query = format!(
        "INSERT INTO task (id, name, start, duration, description, category, schedule_id)
        VALUES ('{}', '{}', {}, '{}', '{}', '{}', '{}')
            ON CONFLICT (id)
            DO UPDATE SET
                name = EXCLUDED.name,
                start = EXCLUDED.start,
                duration = EXCLUDED.duration,
                description = EXCLUDED.description,
                category = EXCLUDED.category,
                schedule_id = EXCLUDED.schedule_id
        RETURNING *
        ;",
        task_data.id,
        task_data.name,
        task_data.start,
        task_data.duration,
        task_data.description,
        task_data.category,
        task_data.schedule_id
    );
    let out: Task = sqlx::query_as(query.as_str())
        .fetch_one(&mut conn)
        .await
        .unwrap();
    Ok(out)
}

pub(super) async fn update_schedule(
    mut conn: PgConnection,
    schedule: &Schedule,
) -> Result<Schedule, sqlx::Error> {
    let query = format!(
        "INSERT INTO schedule (id, name, description)
        VALUES ('{}', '{}', '{}')
        ON CONFLICT (id)
        DO UPDATE SET
                name = EXCLUDED.name,
                description = EXCLUDED.description
        RETURNING *
        ;",
        schedule.id, schedule.name, schedule.description
    );
    let out: Schedule = sqlx::query_as(query.as_str())
        .fetch_one(&mut conn)
        .await
        .unwrap();
    Ok(out)
}

pub(super) async fn new_schedule(
    mut conn: PgConnection,
    new_schedule: &Schedule,
) -> Result<Schedule, sqlx::Error> {
    let query = format!(
        "INSERT INTO schedule (name, description)
        VALUES ('{}', '{}')
        RETURNING *
        ;",
        new_schedule.name, new_schedule.description
    );

    let out: Schedule = sqlx::query_as(query.as_str())
        .fetch_one(&mut conn)
        .await
        .unwrap();
    Ok(out)
}

pub(super) async fn new_task(mut conn: PgConnection, new_task: &Task) -> Result<Task, sqlx::Error> {
    let query = format!(
        "INSERT INTO task (name, start, duration, description, category, schedule_id)
        VALUES ('{}', {}, '{}', '{}', '{}', '{}')
        RETURNING *
        ;",
        new_task.name,
        new_task.start,
        new_task.duration,
        new_task.description,
        new_task.category,
        new_task.schedule_id
    );

    let out: Task = sqlx::query_as(query.as_str())
        .fetch_one(&mut conn)
        .await
        .unwrap();
    Ok(out)
}
