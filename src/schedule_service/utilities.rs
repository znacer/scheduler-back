use std::fs;

use sqlx::{Connection, Executor, PgConnection};

pub(super) async fn sql_connect() -> PgConnection {
    PgConnection::connect("postgres://admin:secretpassword@localhost:5432/scheduler")
        .await
        .unwrap()
}

pub(super) async fn create_tables() -> Result<(), sqlx::Error> {
    let mut conn: PgConnection = sql_connect().await;

    let queries_path = vec![
        "sql_queries/create_table_schedule.sql",
        "sql_queries/create_table_task.sql",
        "sql_queries/create_table_label.sql",
    ];
    for query_path in queries_path.iter() {
        let query = fs::read_to_string(query_path)?;
        conn.execute(query.as_str()).await?;
    }
    Ok(())
}
