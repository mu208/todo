use clap::{Parser, Subcommand, ValueEnum};
use std::fmt;
use std::fmt::Formatter;
use turso::{Builder, Connection};

#[derive(Parser, Debug)]
#[command(
    name = "todo",
    version = "1.0",
    author = "icssa",
    about = "A simple and fast CLI todo manager powered by Turso",
    long_about = None
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

pub struct Task {
    title: String,
    content: String,
    status: TaskStatus,
}
impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::Progress => write!(f, "In Progress"),
            TaskStatus::Done => write!(f, "Done"),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        content: String,
        #[arg(short, long, value_enum)]
        status: TaskStatus,
    },
    Update {
        #[arg(short = 'i', long)]
        task_id: u32,
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        content: String,
        #[arg(short, long, value_enum)]
        status: TaskStatus,
    },
    Delete {
        #[arg(long, short)]
        task_id: u32,
    },
    List,
}
#[derive(ValueEnum, Clone, Debug)]
pub enum TaskStatus {
    Pending,
    Progress,
    Done,
}

async fn create_db(conn: &Connection) -> Result<(), turso::Error> {
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
async fn add_task(task: Task, conn: &Connection) -> Result<(), turso::Error> {
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
async fn list_tasks(conn: &Connection) -> Result<(), turso::Error> {
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
async fn delete_task(conn: &Connection, task_id: u32) -> Result<(), turso::Error> {
    let mut stmt = conn.prepare("DELETE FROM tasks WHERE id = ?1").await?;
    stmt.execute([task_id]).await?;
    println!("Task have been deleted Successfully");
    Ok(())
}

async fn update_task(conn: &Connection, task: Task, task_id: u32) -> Result<(), turso::Error> {
    let mut stmt = conn
        .prepare("UPDATE tasks SET title = ?1, content = ?2, status = ?3 WHERE id = ?4")
        .await?;
    stmt.execute((task.title, task.content, task.status.to_string(), task_id))
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    //println!("{:#?}", args);

    // define the database and give the task's table a name
    let db = Builder::new_local("todo.db").build().await?;

    // define a connection for the db
    let conn = db.connect()?;

    // call a database that create the db in the first run
    create_db(&conn)
        .await
        .expect("Failed to create the Database");

    // Dispatch commands based on user input
    match args.command {
        Commands::Add {
            title,
            content,
            status,
        } => {
            add_task(
                Task {
                    title,
                    content,
                    status,
                },
                &conn,
            )
            .await?
        }
        Commands::Update {
            task_id,
            title,
            content,
            status,
        } => {
            update_task(
                &conn,
                Task {
                    title,
                    content,
                    status,
                },
                task_id,
            )
            .await?
        }
        Commands::Delete { task_id } => delete_task(&conn, task_id).await?,
        Commands::List => list_tasks(&conn).await?,
    }

    Ok(())
}
