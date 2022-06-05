use crate::data::project::Project;
use crate::manager::worker::ProjectWorker;

#[derive(Debug, Clone)]
pub struct Assignment {
    pub id: i64,
    pub project: Project,
    pub input_data: String,
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