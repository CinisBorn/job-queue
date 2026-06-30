use job_queue::api::Api;

fn main() {
    let connection = Api::connect().unwrap();
    
    loop {
        let buf = Api::get_client_payload(&connection).unwrap();
        println!("{:?}", String::from_utf8(buf));
    }
}
