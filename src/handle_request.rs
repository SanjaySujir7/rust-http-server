use crate::parser::RequestData;
use crate::header::{HttpStatus, Response};



pub fn handle_route(request_header : &RequestData) -> Response{

    let mut response = Response::new();

    if request_header.route == "/" {
        response.set_content_type("text/html");
        response.set_status(HttpStatus::Ok);
        response.set_body("Hello this is 200 response from the rust yea yea");
    }

    else if request_header.route == "/home" {
        response.set_content_type("text/html");
        response.set_status(HttpStatus::Ok);
        response.set_body("hello this is Home page !");
    }

    else if request_header.route == "/login" {
        response.redirect("/register");
    }

    else if request_header.route == "/register" {
        response.set_content_type("text/html");
        response.set_status(HttpStatus::Ok);
        response.set_body("hello this is register page");
    }

    else{
        response.set_content_type("text/html");
        response.set_status(HttpStatus::NotFound);
        response.set_body("Not found");
    }

    response
}