fn main() {
    //our ip address we use is a sring slice so we need to use the to string method to convert to
    //type string 
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();

}

//similar to the class in java 
struct Server{
    addr: String,
}

impl Server{
    //Self and Server and interchangable 
    //this is the contructor funtion for the server
    fn new(addr: String) -> Self {
        Server {
            addr 
        }
    }

    fn run(self){
        print!("listening on {}", self.addr);
    }
        
}

struct Request{
    path: String,
    query_string: String,
    method: Method,
    
}

enum Method{
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
