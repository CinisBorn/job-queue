#[derive(Debug)]
pub enum Jobs {
    RetrieveFromDatabase,
    RecordOnDatabase, 
    UpdateDatabase,
    SyncDatabase,
}

#[derive(Debug)]
#[allow(dead_code)] // lint issues because of "Debug" trait
pub struct Job {
    id: u32, 
    payload: String,
    job_type: Jobs,
}

impl Job {
    pub fn build(id: u32, payload: String, job_type: Jobs) -> Self {
        Self {
            id,
            payload,
            job_type,
        }
    }
}