use crate::data::project::Project;
use crate::manager::worker::ProjectWorker;

#[derive(Debug, Clone)]
pub struct Assignment {
    pub id: i64,
    pub project: Project,
    pub input_data: String,
}

#[derive(Debug, Clone)]
pub struct AssignmentResult {
    pub id: i64,
    pub output: String,
    pub error: String,
    pub status: i32,
    pub execution_time: u128,
}

impl Assignment {
    pub fn new(id: i64, project: Project, input_data: String) -> Assignment {
        Assignment {
            id,
            project,
            input_data,
        }
    }

    pub fn create_worker(&self) -> ProjectWorker {
        ProjectWorker {
            assignment: self.clone(),
        }
    }
}

impl AssignmentResult {
    pub fn new(id: i64, output: String, error: String, status: i32, execution_time: u128) -> AssignmentResult {
        AssignmentResult {
            id,
            output,
            error,
            status,
            execution_time,
        }
    }
}