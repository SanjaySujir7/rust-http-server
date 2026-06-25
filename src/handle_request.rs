use crate::parser::RequestData;
use crate::header::ResponseHeaders;



pub fn handle_route(request_header : &RequestData) -> ResponseHeaders{

    let mut response = ResponseHeaders::new();

    if request_header.route == "/" {
        response.set_content_type("text/html");
        response.create_200_response("Hello this is 200 response from the rust yea yea".to_string());
    }

    else if request_header.route == "/home" {
        response.set_content_type("text/html");
        response.create_200_response("hello this is Home page !".to_string());
    }

    else{
        response.set_content_type("text/html");
        response.create_404_response("Not found");
    }

    response
}