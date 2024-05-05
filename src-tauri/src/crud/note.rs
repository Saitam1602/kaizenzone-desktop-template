use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

#[path = "../util/mod.rs"]
mod util;

#[derive(Serialize, Clone, FromRow, Debug)]
pub struct Note {
    id: i64,
    title: String,
    text: String,
    status_id: Option<i64>,
}

#[tauri::command]
pub async fn note_insert(
    title: &str,
    text: Option<&str>,
    status_id: Option<i64>,
) -> Result<i64, String> {
    let db_url = util::db::get_database();
    let db = SqlitePool::connect(&db_url).await.unwrap();
    let query_result = sqlx::query("INSERT INTO note (title, text, status_id) VALUES (?, ?, ?)")
        .bind(title)
        .bind(text)
        .bind(status_id)
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
pub async fn note_update(
    id: i64,
    title: &str,
    text: &str,
    status_id: Option<i64>,
) -> Result<i64, String> {
    let db_url = util::db::get_database();
    let db = SqlitePool::connect(&db_url).await.unwrap();
    let query_result = sqlx::query("UPDATE note SET title=?, text=?, status_id=? WHERE id=?")
        .bind(title)
        .bind(text)
        .bind(status_id)
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
pub async fn note_delete(id: i64) -> Result<i64, String> {
    let db_url = util::db::get_database();
    let db = SqlitePool::connect(&db_url).await.unwrap();
    let query_result = sqlx::query("DELETE FROM note WHERE id=?")
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
pub async fn note_select() -> Result<String, String> {
    let db_url = util::db::get_database();
    let db = SqlitePool::connect(&db_url).await.unwrap();
    let query_result = sqlx::query_as::<_, Note>("SELECT id, title, text, status_id FROM note")
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
