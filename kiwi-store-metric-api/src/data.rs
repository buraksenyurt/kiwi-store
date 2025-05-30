use axum::Json;
use chrono::{DateTime, Utc};
use log::info;
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

/// Fetches the latest metrics from the database.
/// This function queries the `metrics` table and retrieves the most recent 50 entries,
/// ordered by the timestamp in descending order.
///
/// # Arguments
///
/// * `pool` - A connection pool to the PostgreSQL database.
///
/// # Returns
///
/// A JSON response containing a vector of `Metric` structs, each representing a row from the `metrics` table.
pub async fn get_metrics(pool: PgPool) -> Json<Vec<Metric>> {
    info!("Fetching metrics from the database");
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
