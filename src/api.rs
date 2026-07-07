use std::io::Read;
use std::net::TcpListener;
use std::sync::Arc;

#[derive(Debug)]
pub struct Api {
    listener: Arc<TcpListener> 
}

#[derive(Debug)]
pub struct ClientRequest {
    pub job_type: String,
    pub payload: String,
}

impl Api {
    pub fn connect() -> Result<Self, std::io::Error> {
        let socket = Arc::new(TcpListener::bind("127.0.0.1:0")?);
        let addr = socket.local_addr()?;

        println!("Connected at: {}", addr);
        
        Ok(Self {
            listener: socket
        })
    }

    fn get_listener(&self) -> Arc<TcpListener>{
        self.listener.clone()
    }

    fn get_client_data(&self) -> Result<Vec<u8>, std::io::Error> {
        let (mut stream, _) = self.get_listener().accept()?;
        let mut buf = [0u8; 524];
    
        let bytes_read = stream.read(&mut buf)?;
        let mut buf = buf.to_vec();
    
        buf.truncate(bytes_read);
        Ok(buf)
    }

    pub fn deserializate_request(&self) -> Result<ClientRequest, std::io::Error> {
        let content = self.get_client_data()?;
        let request = String::from_utf8(content);
        let request = request
            .unwrap()
            .split(";")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        Ok(ClientRequest { 
            job_type: request[0].clone(), 
            payload: request[1].clone(), 
        })
    }
}