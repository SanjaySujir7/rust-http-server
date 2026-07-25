
/*
    this file will hold the http parser and this is one of the important file going to be :)
*/

use std::{collections::HashMap, i32};

#[derive(Default)]
pub struct RequestData {
    pub request_type : String,
    pub route : String,
    pub http_type : String,
    pub host : String,
    pub connection : String,
    pub user_agent : String,
    pub ip_address : String,
    content_length : i32,
    pub data : HashMap<String,String>,
    pub cookie : HashMap<String,String>,
}

// request = "GET /favicon.ico HTTP/1.1\r\nHost: 127.0.0.1:8080\r\nConnection: keep-alive"


fn get_method (request : &str, struct_request : &mut RequestData){
    let mut sub_parts = request.split_whitespace();

    if let Some(m) = sub_parts.next(){
        if m == "GET"{
            struct_request.request_type = String::from("GET");

        }
        else if m == "POST"{
            struct_request.request_type = String::from("POST");
        }

    }

    if let Some(r) = sub_parts.next(){
        struct_request.route = r.to_string();
    }

    if let Some(h)  = sub_parts.next(){
        struct_request.http_type = h.to_string();
    }

}

pub fn parse_request(request : &str) -> RequestData{

    let parts : Vec<&str> = request.split("\r\n").collect();
    let mut request_data = RequestData::default();

    get_method(parts[0], &mut request_data);


    for line in &parts[1..] {
        if let Some((key, value)) = line.split_once(':') {
            match key {
                "Host" => {
                    request_data.host = value.trim().to_string();
                }

                "Connection" => {
                    request_data.connection = value.trim().to_string();
                }

                "User-Agent" => {
                    request_data.user_agent = value.trim().to_string();
                }

                "Content-Length" => {
                    if let Ok(_length) = value[1..
                    ].parse::<i32>() {
                        request_data.content_length = _length;
                    }
                }
                "Cookie" => {
                    parse_cookie(&mut request_data, &value.to_string());
                }

                _ => {}
            }
        }
        
   }
   let _data : Vec<&str> = request.split("\r\n\r\n").collect();

   parse_form_data(&mut request_data,_data[1]);
   request_data
    
}



fn parse_form_data(struct_request : &mut RequestData,data : &str){

    if struct_request.request_type == "POST" {
        let _length = struct_request.content_length as usize;


        let _parts : Vec<&str> = data[.._length].split("&").collect();

        for part in _parts {
            if let Some((key , value)) = part.split_once("=") {
                struct_request.data.insert(key.to_string(), value.to_string());
            }
        }
    }
}

fn parse_cookie(struct_request : &mut RequestData, data : &str){

    let _parts = data.trim().split(";");

    for line in _parts {
        if let  Some((_cookie,_value)) = line.split_once("=") {
            struct_request.cookie.insert(_cookie.trim().into(), _value.into());
        }
    }
}


pub fn ip_parser(socket_address : &str) -> String{

    let _ip = socket_address.split_once(":");
    let ip_address : String;

    if let Some((key,_)) = socket_address.split_once(":"){
        ip_address = key.to_string();
    }
    else{
        ip_address = "127.0.0.1".to_string();
    }

    ip_address
}