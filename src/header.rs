use chrono::Utc;
use std::fs;
use crate::logger::log_error;

const SERVER_NAME: &str = "rust_server[sanjay]/1.0";
const TEMPLATES_PATH : &str = "templates/";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpStatus {
    Ok,
    NotFound,
    Redirect,
    ServerError,
}

impl HttpStatus {
    pub fn code(&self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::NotFound => 404,
            Self::Redirect => 302,
            Self::ServerError => 500,
        }
    }

    pub fn reason(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::NotFound => "Not Found",
            Self::Redirect => "Found",
            Self::ServerError => "Internal Server Error",
        }
    }
}

pub struct Response {
    pub status: HttpStatus,
    pub content: String,
    pub server: &'static str,
    pub content_type: String,
    pub content_length : usize,
    pub location: Option<String>,
    pub cookie : String,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status: HttpStatus::Ok,
            content: String::new(),
            server: SERVER_NAME,
            content_type: String::new(),
            content_length : 0,
            location: None,
            cookie : String::new(),
        }
    }

    fn set_html_content(&mut self,content : &str){
        self.content_type = "text/html".into();
        self.content = content.into();
    }


    pub fn html_response(&mut self, body : &str){
        self.set_html_content(body);
        self.status = HttpStatus::Ok;
    }

    pub fn not_found(&mut self,body : &str){
        self.status = HttpStatus::NotFound;
        self.set_html_content(body);
        
    }

    pub fn redirect(&mut self, location: &str,body : &str) {
        self.status = HttpStatus::Redirect;
        self.location = Some(location.into());
        self.set_html_content(body);

    }

    fn internal_server_error(&mut self) {

        self.status = HttpStatus::ServerError;
        self.set_html_content("<h1>Server Error : try again later ! </h1>");
    }

    pub fn render_html(&mut self , filename : &str){
        let mut file = filename.to_string();
        file.insert_str(0, TEMPLATES_PATH);

        let contents = fs::read_to_string(file);

        match contents {
            Ok(content) => {
                self.set_html_content(&content);
            },

            Err(e) => {
                self.internal_server_error();
                log_error(&e.to_string());
            }
        }
    }

    pub fn set_cookie(&mut self , key : &str,value : &str,http : bool){

        let _cookie_formated :String = format!("{key}={value};");

        self.cookie.push_str(&_cookie_formated);
        self.cookie.push_str(" path=/;");

        if http {
            self.cookie.push_str(" HttpOnly");
        }
    }

    pub fn build(&mut self) -> String{
        let date = Utc::now()
            .format("%a, %d %b %Y %H:%M:%S GMT");

        let mut response = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status.code(),
            self.status.reason()
        );

        if let Some(location) = &self.location {
            response.push_str(&format!("Location: {}\r\n", location));
        }

        self.content_length = self.content.len();

        response.push_str(&format!(
            "Date: {}\r\n\
             Server: {}\r\n\
             Content-Type: {}\r\n\
             Content-Length: {}\r\n\
             ",
            date,
            self.server,
            self.content_type,
            self.content_length,
        ));

        if !self.cookie.is_empty() {
            response.push_str(&format!("Set-Cookie: {}\r\n",self.cookie));
        }

        response.push_str(&format!("\r\n{}",self.content));

        println!("the response is : {}",&response);
        response

    }
}