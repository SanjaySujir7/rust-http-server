
/*
    this file will hold the http parser and this is one of the important file going to be :)
*/

#[derive(Default)]
pub struct RequestData {
    pub request_type : String,
    pub route : String,
    pub http_type : String,
    pub host : String,
    pub connection : String,
    pub user_agent : String,
    pub ip_address : String
}

// request = "GET /favicon.ico HTTP/1.1\r\nHost: 127.0.0.1:8080\r\nConnection: keep-alive"


fn get_method (request : &str, struct_request : &mut RequestData){
    let mut sub_parts = request.split_whitespace();

    if let Some(m) = sub_parts.next(){
        if m == "GET"{
            struct_request.request_type = String::from("GET");

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

                _ => {}
            }
        }
   }

   request_data
    
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