use std::io::Read;
use std::net::TcpListener;
use std::sync::Arc;

pub struct Api {
    listener: Arc<TcpListener> 
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

    pub fn get_client_payload(api: &Self) -> Result<Vec<u8>, std::io::Error> {
        let (mut stream, _) = api.get_listener().accept()?;
        let mut buf = [0u8; 524];
    
        let bytes_read = stream.read(&mut buf)?;
        let mut buf = buf.to_vec();
    
        buf.truncate(bytes_read);
        Ok(buf)
    }
}