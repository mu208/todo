use clap::{Parser, Subcommand};
use crate::models::TaskStatus;

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