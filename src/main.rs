mod cli;
mod db;
mod models;

use clap::Parser;
use cli::{Args, Commands};
use db::{add_task, create_db, delete_task, list_tasks, update_task};
use models::Task;
use turso::Builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

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
