use crate::parser::RequestData;
use crate::header:: Response;



pub fn handle_route(request_header : &RequestData) -> Response{

    let mut response = Response::new();

    if request_header.route == "/" {
        response.html_response("Hello this is 200 response from the rust yea");
    }

    else if request_header.route == "/home" {
        response.html_response("Welcome to Home page !");
    }

    else if request_header.route == "/login" {
        response.render_html("home.html");
        
    }

    else if request_header.route == "/api/login" {
        println!("{:#?}",request_header.data);
        response.redirect("/home","");

    }

    else if request_header.route == "/register" {
        response.html_response("hello this is register page");
    }

    else{
        response.not_found("<h1>Not Found</h1>");
    }

    response
}