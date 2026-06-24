use std::{ io::{Read, Write}, net::{TcpListener, TcpStream}};
mod logger;
mod header;
mod parser;

use header::HttpStatus;


/* 
    This is the core file which handles request. iam still expermenting so some of the functions will stay here :)
*/



fn write_request(stream : &mut TcpStream){

    let content : String = String::from("<h1> this is from rust and this is to test how much data this can send </h1>");
    let content_length = content.len();

    let _buffer = format!("HTTP/1.1 {} OK\r\nContent-Length: {} \r\n\r\n{}",HttpStatus::Ok as u16,content_length.to_string(),content);

    if let Err(e) = stream.write_all(_buffer.as_bytes()) {
        let error = format!("Failed to send data {}",e);

        logger::log_error(&error);
    }

}




// to read the bytes form the request 
fn read_incoming_request(stream : &mut TcpStream){
    let mut buffer = [0;1024];

    let bytes_read = stream.read(&mut buffer).unwrap();

    let log = format!("Bytes readed from incoming requst is {}",bytes_read);
    logger::log(&log);

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    parser::parse_request(request.as_ref());

    

    println!("{}", request);


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

                read_incoming_request(&mut stream);

                write_request(&mut stream);

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