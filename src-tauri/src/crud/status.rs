use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

#[path = "../util/mod.rs"]
mod util;

#[derive(Serialize, Clone, FromRow, Debug)]
pub struct Status {
    id: i64,
    text: String,
    color: String,
}

#[tauri::command]
pub async fn status_insert(text: &str, color: &str) -> Result<i64, String> {
    let db_url = util::db::get_database();
    let db = SqlitePool::connect(&db_url).await.unwrap();
    let query_result = sqlx::query("INSERT INTO status (text, color) VALUES (?, ?)")
        .bind(text)
        .bind(color)
        .execute(&db)
        .await;
    if query_result.is_err() {
        db.close().await;
        return Err(format!("{:?}", query_result.err()));
    }

    let id = query_result.unwrap().last_insert_rowid();
    db.close().await;
    Ok(id)
}

#[tauri::command]
pub async fn status_update(id: i64, text: &str, color: &str) -> Result<i64, String> {
    let db_url = util::db::get_database();
    let db = SqlitePool::connect(&db_url).await.unwrap();
    let query_result = sqlx::query("UPDATE status SET text=?, color=? WHERE id=?")
        .bind(text)
        .bind(color)
        .bind(id)
        .execute(&db)
        .await;
    if query_result.is_err() {
        db.close().await;
        return Err(format!("{:?}", query_result.err()));
    }
    db.close().await;
    Ok(id)
}

#[tauri::command]
pub async fn status_delete(id: i64) -> Result<i64, String> {
    let db_url = util::db::get_database();
    let db = SqlitePool::connect(&db_url).await.unwrap();
    let query_result = sqlx::query("DELETE FROM status WHERE id=?")
        .bind(id)
        .execute(&db)
        .await;
    if query_result.is_err() {
        db.close().await;
        return Err(format!("{:?}", query_result.err()));
    }
    db.close().await;
    Ok(id)
}

#[tauri::command]
pub async fn status_select() -> Result<String, String> {
    let db_url = util::db::get_database();
    let db = SqlitePool::connect(&db_url).await.unwrap();
    let query_result = sqlx::query_as::<_, Status>("SELECT id, text, color FROM status")
        .fetch_all(&db)
        .await;
    if query_result.is_err() {
        db.close().await;
        return Err(format!("{:?}", query_result.err()));
    }
    let results = query_result.unwrap();
    let encoded_message = serde_json::to_string(&results).unwrap();
    db.close().await;
    Ok(format!("{:?}", encoded_message))
}
