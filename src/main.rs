use std::{ io::{Read, Write}, net::{TcpListener, TcpStream}};
mod logger;
mod header;
mod parser;
mod handle_request;


use crate::{header::ResponseHeaders, parser::RequestData};


/* 
    This is the core file which handles request. iam still expermenting so some of the functions will stay here :)
*/



fn write_request(stream : &mut TcpStream,request_header : &RequestData){
 

    let mut handle : ResponseHeaders = handle_request::handle_route(request_header);

    let _buffer = handle.create_response();

    if let Err(e) = stream.write_all(_buffer.as_bytes()) {
        let error = format!("Failed to send data {}",e);

        logger::log_error(&error);
    }

}




// to read the bytes form the request 
fn read_incoming_request(stream : &mut TcpStream) -> RequestData{
    let mut buffer = [0;1024];

    let bytes_read = stream.read(&mut buffer).unwrap();

    let log = format!("Bytes readed from incoming requst is {}",bytes_read);
    logger::log(&log);

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    let request_data : RequestData = parser::parse_request(request.as_ref());
    

    println!("{}", request);

    request_data

}


// this will listne to the port and adress given
fn tcp_listener(address : &str, port : u16){

    let adress_and_port :String = format!("{}:{}",address,port);
    println!("starting listening in http://{}",adress_and_port);

    let listener : TcpListener = TcpListener::bind(adress_and_port).unwrap();
   

    for _stream  in listener.incoming(){
         match _stream {

            Ok(_stream) => {

                logger::connection_log();
                
                let mut stream = _stream;

                let request_headers : RequestData = read_incoming_request(&mut stream);

                write_request(&mut stream,&request_headers);

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