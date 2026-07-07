use job_queue::api::Api;
use job_queue::queue::Queue;

fn main() {
    let connection = Api::connect().unwrap();
    let mut  queue = Queue::new();
    
    loop {
        let buf = connection.deserializate_request();
        
        queue.create_job(buf.unwrap());
        queue.show_queue();
    }
}
