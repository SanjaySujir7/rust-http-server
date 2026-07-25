use crate::parser::RequestData;
use crate::header:: Response;


fn handle_login(request : &RequestData) -> bool{
    match request.data.get("username"){

        Some(name) => {
            if let Some(password) = request.data.get("password") {
                if password == "test123" && name == "sanjay" {
                    return true;
                }
                else {
                    return  false;
                }
            }
            else {
                println!("No Password is specified !");
                return false;
            }
        },

        None => {
            println!("No username found !");
            return false;
        
        }
    }
}


pub fn handle_route(request_header : &RequestData) -> Response{

    let mut response = Response::new();

    if request_header.route == "/" {
        response.html_response("Hello this is 200 response from the rust yea");
    }

    else if request_header.route == "/home" {

        if let Some(name) = request_header.cookie.get("user") {
            let _response_text = format!("<h1> Hello {} Welcome to Home page !",name);
            response.html_response(&_response_text);
        }
        else {
            response.redirect("/login", "login in ");
        }
        
    }

    else if request_header.route == "/login" {

        response.render_html("login.html");
        
    }

    else if request_header.route == "/api/login" {
        let _result: bool = handle_login(&request_header);

        if _result {
            response.set_cookie("user",request_header.data.get("username").unwrap_or(&"none".to_string()),true);
            response.redirect("/home", "");
        }
        else {
            response.html_response("<h2>Wrong username or password try again !</h2>");
        }

    }

    else if request_header.route == "/register" {
        response.html_response("hello this is register page");
    }

    else{
        response.not_found("<h1>Not Found</h1>");
    }

    response
}