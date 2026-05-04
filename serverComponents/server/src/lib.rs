//! # Server
//! Crate which hold the server components of the webserver program


pub use core::Config;

/// # Core
/// This mod include the web server features which we implemented 
pub mod core {
    pub mod threads;
    use threads::ThreadPool;

    const HTML_ROOT : &str = "./serverComponents/server/html/basics";

    use std::{fs, io::{Read, Write},thread::sleep , net::{TcpListener, TcpStream}, time::Duration};

    /// keeps ip and port of the address in [`str`] 
    /// mainly exist to work with [`Server`] components 
    pub struct Config<'a> {
        ip : & 'a str ,
        port : & 'a str
    }

    impl <'a> Config <'a> {
        /// Create new config settings from [`str`] slice 
        /// # Example 
        /// ```
        /// use server::Config;
        /// 
        /// let config = Config::new("192.168.1.1:8080").unwrap();
        /// 
        /// assert_eq!(config.address() , "192.168.1.1:8080")
        /// ```
        /// If ip given is without the port None will comeback and struct wont save any ip address
        pub fn new(ip_port : & 'a str) -> Option<Self> {
            let item = ip_port.split(':').collect::<Vec<&str>>();
            if item.len() > 2 || item.len() < 2 {
               None
            } else {
                Some(Self { ip : item[0] , port: item[1] })
            }
        }

        /// Returns the address that is currently saved inside of config by the user 
        /// by creating new [`String`]
        pub fn address(&self) -> String {
            format!("{}:{}" , self.ip , self.port)
        }
     
    }

    #[derive(PartialEq)]
    enum ServerStatus {
        Online ,
        Offline
    }

    /// Simple struct to hold server components and functions in one place
    /// still use standard [`TcpListener`] as its main component
    pub struct Server {
        status : ServerStatus ,
        listener : TcpListener,
        pool : ThreadPool,
    }

    impl Server {
        /// Create the server from [`Config`] if the ip is invalid it will return an error
        pub fn new(cfg : Config) -> Result<Self , Box<dyn std::error::Error>> {
            let listener = TcpListener::bind(cfg.address())?;
            Ok(Self { listener : listener , status : ServerStatus::Offline , pool : ThreadPool::new(4) })            
        }


        /// Start the server as an single thread server 
        pub fn start(&mut self) -> () {
            self.status = ServerStatus::Online;
            let _ = self.listener.set_nonblocking(true).expect("Cannot be non-blockade");
            while self.status == ServerStatus::Online {
                match self.listener.accept() {
                    Ok((stream , addr)) => {
                        println!("connection from {} recived !" , addr);
                        self.pool.execution(move || {
                            let _ = Server::handel_connections(stream);
                        });
                    }

                    Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                        sleep(Duration::from_millis(300));
                    }

                    Err(err) => {
                        panic!("Error listening to connections : {}" , err)
                    }
                }
            } 
        }
        
        /// Useless function for now has no use
        pub fn stop(&mut self) -> () {
            self.status = ServerStatus::Offline;
        }

        /// Reads the stream respond and and read the users 
        fn handel_connections(mut stream : TcpStream) -> Result<() , Box<dyn std::error::Error>> {
            let mut buffer = [0 ; 1024];
            let _ = stream.read(&mut buffer)?;
            println!("{}" , String::from_utf8_lossy(&buffer));

            if buffer.starts_with(b"GET") {
                stream.write(Server::make_response("hello").as_bytes())?;
            } else {
                stream.write(Server::make_response("notfound").as_bytes())?;
            }

            let _ = stream.flush();
            Ok(())
        }


        // TODO 1 : change this function asap after you finish that struct so you can render 
        pub fn make_response(page_name : &str) -> String {
            // Hint : you have to change this function here otherwise its hardcoded 
            let body = Server::render_html(page_name);
            let response = format!(
                    "HTTP/1.1 200 OK\r\n\
                    Content-Type: text/html; charset=utf-8\r\n\
                    Content-Length: {}\r\n\
                    Connection: close\r\n\
                    \r\n\
                    {}",
                    body.len(),
                    body
                ); 
            
            response
        } 


        /// Read and send the HTML into other functions so they can use that
        /// returns the data in owned [`String`]
        fn render_html(page : &str) -> String {
            let mut html_data = String::new();

            println!("loading {} ..." , format!("{}/{}" , HTML_ROOT , page));

            let mut html_file = fs::OpenOptions::new()
            .read(true)
            .write(false)
            .open(format!("{}/{}.html" , HTML_ROOT , page))
            .expect("Error loading HTML file");
        
            let read_buffer = html_file.read_to_string(&mut html_data);
            println!("Read {} bytes from the buffer !" , read_buffer.unwrap());

            html_data
        }


    }
}