use crate::models::{Task, TaskStatus};
use turso::Connection;

pub async fn create_db(conn: &Connection) -> Result<(), turso::Error> {
    // create db's tables
    conn.execute(
        r#"CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT,
            status TEXT DEFAULT 'pending',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"#,
        (),
    )
    .await?;
    Ok(())
}

pub async fn add_task(task: Task, conn: &Connection) -> Result<(), turso::Error> {
    conn.execute(
        "INSERT INTO tasks (title, content, status) VALUES (?, ?, ?)",
        (
            task.title.as_str(),
            task.content.as_str(),
            match task.status {
                TaskStatus::Pending => "pending",
                TaskStatus::Progress => "progress",
                TaskStatus::Done => "done",
            },
        ),
    )
    .await?;
    println!("Task added successfully.");
    println!(
        "Task Title :{}\nTask Content :{}\nTask Status :{}",
        task.title, task.content, task.status
    );
    Ok(())
}

pub async fn list_tasks(conn: &Connection) -> Result<(), turso::Error> {
    let mut rows = conn.query("SELECT * FROM tasks", ()).await?;
    while let Some(row) = rows.next().await? {
        let id: i64 = row.get(0)?;
        let title: String = row.get(1)?;
        let content: String = row.get(2)?;
        let status: String = row.get(3)?;
        println!("Task: {}, {}, {}, {}", id, title, content, status);
    }
    Ok(())
}

pub async fn delete_task(conn: &Connection, task_id: u32) -> Result<(), turso::Error> {
    let mut stmt = conn.prepare("DELETE FROM tasks WHERE id = ?1").await?;
    stmt.execute([task_id]).await?;
    println!("Task have been deleted Successfully");
    Ok(())
}

pub async fn update_task(conn: &Connection, task: Task, task_id: u32) -> Result<(), turso::Error> {
    let mut stmt = conn
        .prepare("UPDATE tasks SET title = ?1, content = ?2, status = ?3 WHERE id = ?4")
        .await?;
    stmt.execute((task.title, task.content, task.status.to_string(), task_id))
        .await?;
    Ok(())
}
