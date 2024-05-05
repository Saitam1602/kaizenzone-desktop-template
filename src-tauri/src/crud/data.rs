use sqlx::SqlitePool;

#[path = "../util/db.rs"]
mod db;

#[tauri::command]
pub async fn create_tables() -> Result<String, String> {
    let db_url = db::get_database();
    db::create(&db_url).await;
    let db = SqlitePool::connect(&db_url).await.unwrap();
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS status (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text VARCHAR(250) NOT NULL,
            color VARCHAR(50)
        );",
    )
    .execute(&db)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tag (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text VARCHAR(250) NOT NULL,
            color VARCHAR(50)
        );",
    )
    .execute(&db)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS note (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title VARCHAR(250),
            text TEXT NULL,
            status_id INTEGER REFERENCES status(id) NULL
        );",
    )
    .execute(&db)
    .await
    .unwrap();

    let result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS note_tags (
                note_id INTEGER REFERENCES note(id),
                tag_id INTEGER REFERENCES tag(id),
                PRIMARY KEY (note_id, tag_id)
            );",
    )
    .execute(&db)
    .await;

    if result.is_err() {
        db.close().await;
        return Err(format!("{:?}", result.err()));
    }

    db.close().await;
    Ok("Data structure is ready.".to_string())
}

#[tauri::command]
pub async fn fill_tables() -> Result<String, String> {
    let db_url = db::get_database();
    let db = SqlitePool::connect(&db_url).await.unwrap();

    let sql = vec![
        "INSERT INTO status (text, color) VALUES('Pending', 'yellow')",
        "INSERT INTO status (text, color) VALUES('In Progress', 'blue')",
        "INSERT INTO status (text, color) VALUES('Completed', 'green')",
    ];
    for query in sql {
        let result = sqlx::query(query).execute(&db).await;
        if result.is_err() {
            db.close().await;
            return Err(format!("{:?}", result.err()));
        }
    }
    db.close().await;
    Ok("Test content is ready.".to_string())
}
