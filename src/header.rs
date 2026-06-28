use chrono::Utc;

const SERVER_NAME: &str = "rust_server[sanjay]/1.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpStatus {
    Ok,
    NotFound,
    Redirect,
}

impl HttpStatus {
    pub fn code(&self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::NotFound => 404,
            Self::Redirect => 302,
        }
    }

    pub fn reason(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::NotFound => "Not Found",
            Self::Redirect => "Found",
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
        }
    }

    pub fn set_content_type(&mut self, content_type: &str) {
        self.content_type = content_type.into();
    }

    pub fn set_status(&mut self, status: HttpStatus) {
        self.status = status;
    }

    pub fn set_body(&mut self, body: &str) {
        self.content = body.into();
    }

    pub fn redirect(&mut self, location: &str) {
        self.status = HttpStatus::Redirect;
        self.location = Some(location.into());
    }

    pub fn build(&mut self) -> String {
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
             \r\n{}",
            date,
            self.server,
            self.content_type,
            self.content_length,
            self.content
        ));

        response
    }
}