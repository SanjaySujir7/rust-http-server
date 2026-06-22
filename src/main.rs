use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};


fn logger(message : &str){
    println!("{}",message);
}


fn write_request(stream : &mut TcpStream){

    let _buffer = "HTTP/1.1 200 OK\r\nContent-Length: 15\r\n\r\nHello_from_rust";

    if let Err(e) = stream.write_all(_buffer.as_bytes()) {
        println!("Failed to send data: {}", e);
    }

}




// to read the bytes form the request 
fn read_incoming_request(stream : &mut TcpStream){
    let mut buffer = [0;1024];

    let bytes_read = stream.read(&mut buffer).unwrap();

    let log = format!("Bytes readed from incoming requst is {}",bytes_read);
    logger(&log);

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
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
                logger("New connection !");
                
                let mut stream = _stream;

                read_incoming_request(&mut stream);

                write_request(&mut stream);

            }

            Err(e) => {
                println!("Error happend {}",e);
            }
             
         }
    }

}

fn main() {
    let adress = "127.0.0.1";
    let port: u16 = 8080;
    
    tcp_listener(adress,port);
}