
/*
    this file will hold the http parser and this is one of the important file going to be :)
*/


#[derive(Default)]
struct RequestData {
    request_type : String,
    route : String,
    http_type : String,
    host : String,
    connection : String,
    user_agent : String,

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

pub fn parse_request(request : &str){

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
    
}