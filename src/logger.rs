use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;
use crate::header::Response;
use crate::parser::RequestData;


fn get_current_time () -> String{
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S");
    
    current_time.to_string()
}




pub fn connection_log(request_header : &RequestData, bytes : &usize){
    let current_time = get_current_time();
    println!("[incoming request][{}] {} {} {} {}",current_time,request_header.request_type,request_header.route,request_header.ip_address,bytes);
}

pub fn response_log(request_header : &Response){
    let current_time = get_current_time();
    println!("[response][{}] {} {} {}",current_time,request_header.status.code(),request_header.status.reason(),request_header.content_length);
}


pub fn log_error(error_message : &str){
    let current_time = get_current_time();

    let error = format!("[{}] Error : {}",current_time,error_message);

    let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open("error.log")
    .unwrap();

    writeln!(file,"{}",error).unwrap();

    println!("{}",error_message);
}
