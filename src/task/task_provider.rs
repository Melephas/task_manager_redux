use crate::task::{
    task,
    task_provider_error::TaskProviderError,
};

pub trait TaskProvider {
    fn get_all_tasks(&self) -> Result<Vec<task::Task>, TaskProviderError>;
    fn get_task_with_name(&self, name: &str) -> Result<task::Task, TaskProviderError>;
}