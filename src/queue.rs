use std::collections::VecDeque;
use crate::api::ClientRequest;
use crate::jobs::{Job, Jobs};

#[derive(Debug)]
pub struct Queue {
    queue: VecDeque<Job>,
}

impl Queue {
    pub fn new() -> Self {
         Self { queue: VecDeque::new() }
    }
    pub fn create_job(&mut self, request: ClientRequest) {    
        match request.job_type.as_str() {
            "retrieve" => self.queue.push_back(Job::build (
                rand::random(), 
                request.payload, 
                Jobs::RetrieveFromDatabase
            )),
            "sync" => self.queue.push_back(Job::build (
                rand::random(), 
                request.payload, 
                Jobs::SyncDatabase
            )),
            "update" => self.queue.push_back(Job::build (
                rand::random(), 
                request.payload, 
                Jobs::UpdateDatabase
            )),
            "record" => self.queue.push_back(Job::build (
                rand::random(), 
                request.payload, 
                Jobs::RecordOnDatabase
            )),
            _ => {
                eprintln!("no supported job") 
            }
        }
    }
    pub fn queue(&self) -> &VecDeque<Job>{
        &self.queue
    }
    
    #[cfg(debug_assertions)]
    pub fn show_queue(&self) {
        dbg!(&self.queue);
    }
}
