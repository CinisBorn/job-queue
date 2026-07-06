use job_queue::api::Api;

fn main() {
    let connection = Api::connect().unwrap();
    
    loop {
        let buf = connection.deserializate_request();
        println!("{:?}", buf);
    }
}
