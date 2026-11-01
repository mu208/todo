use std::fmt;
use std::fmt::Formatter;
use clap::ValueEnum;

pub struct Task {
    pub title: String,
    pub content: String,
    pub status: TaskStatus,
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
#[derive(ValueEnum, Clone, Debug)]
pub enum TaskStatus {
    Pending,
    Progress,
    Done,
}