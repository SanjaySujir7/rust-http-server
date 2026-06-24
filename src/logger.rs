use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;


fn get_current_time () -> String{
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S");
    
    current_time.to_string()
}


pub fn log(message : &str){
    println!("{}",message);
}


pub fn connection_log(){
    let current_time = get_current_time();

    println!("[{}] New connection !",current_time);
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
