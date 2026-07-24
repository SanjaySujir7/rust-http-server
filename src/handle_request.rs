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
        response.html_response("<h1>Welcome to Home page !<h2>");
    }

    else if request_header.route == "/login" {
        response.render_html("home.html");
        
    }

    else if request_header.route == "/api/login" {
        let _result: bool = handle_login(&request_header);

        if _result {
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