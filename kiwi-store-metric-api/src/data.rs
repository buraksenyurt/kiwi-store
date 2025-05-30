use axum::Json;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Serialize, FromRow)]
pub struct Metric {
    time_stamp: DateTime<Utc>,
    test_type: String,
    total_commands: i32,
    successful_commands: i32,
    failed_commands: i32,
    average_latency_ms: f64,
}

pub async fn get_metrics(pool: PgPool) -> Json<Vec<Metric>> {
    let rows = sqlx::query_as::<_, Metric>(
        r#"
        SELECT 
            time_stamp, 
            test_type, 
            total_commands, 
            successful_commands, 
            failed_commands, 
            average_latency_ms
        FROM metrics
        ORDER BY time_stamp DESC
        LIMIT 50
        "#,
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch metrics");

    Json(rows)
}
