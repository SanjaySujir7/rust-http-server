use chrono::Utc;
static  SERVER_NAME : &str = "rust_server[sanjay]/1.0";

pub enum HttpStatus {
    Ok = 200,
    NotFound = 404
}

pub struct ResponseHeaders {
    pub status_code : u16,
    pub reason_pharse : String,
    pub content_length : usize,
    pub content : String,
    pub server : String,
    pub date : String,
    pub content_type : String,
}



impl  ResponseHeaders {

    pub fn new() -> Self{

        let gmt = Utc::now()
        .format("%a, %d %b %Y %H:%M:%S GMT");

        let response = Self{

            status_code : 0,
            reason_pharse : String::from(""),
            content_length : 0,
            content :String::from(""),
            server : SERVER_NAME.to_string(),
            content_type : String::from(""),
            date : gmt.to_string(),
        };

        response
    }

    pub fn set_content_type(&mut self , content_type : &str){
        self.content_type = content_type.to_string();
    }

    pub fn create_200_response(&mut self,content : String){

        self.status_code = HttpStatus::Ok as u16;
        self.reason_pharse = String::from("Ok");
        self.content = content;
    }

    pub fn create_404_response(&mut self , content : &str){
        self.status_code = HttpStatus::NotFound as u16;
        self.reason_pharse = "Not Found".to_string();
        self.content = content.to_string();
    }

    pub fn create_response(&mut self) -> String{

        self.content_length = self.content.len();

        let buffer = format!(
            "HTTP/1.1 {} {}\r\nDate: {}\r\nServer: {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}"
            ,self.status_code, self.reason_pharse,self.date,self.server,self.content_type,self.content_length,self.content
        );

        buffer
    }

}



