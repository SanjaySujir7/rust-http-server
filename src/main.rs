use std::{io::{Read, Write}, net::{TcpListener, TcpStream},time::Duration};
mod logger;
mod header;
mod parser;
mod handle_request;


use crate::{header::Response, parser::RequestData};


/* 
    This is the core file which handles request. iam still expermenting so some of the functions will stay here :)
*/



fn write_request(stream : &mut TcpStream,request_header : &RequestData){
 

    let mut handle : Response = handle_request::handle_route(request_header);

    let _buffer = handle.build();

    if let Err(e) = stream.write_all(_buffer.as_bytes()) {
        let error = format!("Failed to send data {}",e);

        logger::log_error(&error);
    }
    else {
        logger::response_log(&handle);
    }

}




// to read the bytes form the request 
fn read_incoming_request(stream : &mut TcpStream, ip_address : &str) -> Result<RequestData,bool> {
    let mut buffer = [0;2024];

    if let Err(_) = stream.set_read_timeout(Some(Duration::from_secs(15))) {
        return  Err(false);
    }
    
    if let Ok(bytes_read ) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            return Err(false);
        }
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);

        let mut request_data : RequestData = parser::parse_request(request.as_ref());
        request_data.ip_address = ip_address.into();

        logger::connection_log(&request_data,&bytes_read);

        // println!("{}", request);

        Ok(request_data)
    }

    else{
        return  Err(false);
    }

}


fn get_client_address(stream : &TcpStream) -> String{

    let mut ip_address : String = String::from("127.0.0.1");

    if let Ok(address) = stream.peer_addr() {
        ip_address = parser::ip_parser(&address.to_string());
    }
    

    ip_address
}

// this will listne to the port and adress given
fn tcp_listener(address : &str, port : u16){

    let adress_and_port :String = format!("{}:{}",address,port);
    println!("starting listening in http://{}",adress_and_port);

    let listener : TcpListener = TcpListener::bind(adress_and_port).unwrap();
   

    for _stream  in listener.incoming(){
         match _stream {

            Ok(_stream) => {
                
                let mut stream = _stream;

                
                let ip_address = get_client_address(&stream);
                // let request_headers : = read_incoming_request(&mut stream,ip_address);
                loop{

                    if let Ok(request_headers) = read_incoming_request(&mut stream, &ip_address) {
                        write_request(&mut stream,&request_headers);
                        
                        if request_headers.connection != "keep-alive" {
                            break;
                        }
                    }
                    else {
                        println!("connection terminated !");
                        break;
                    }
                }
                

            }

            Err(e) => {
                logger::log_error(&e.to_string());
            }
             
         }
    }

}

fn main() {
    let adress = "127.0.0.1";
    let port: u16 = 8080;
    
    tcp_listener(adress,port);
}