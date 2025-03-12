use actix_web::HttpResponse;
use serde_json::{json, Value};

/// Enum untuk status kode HTTP agar lebih terstruktur
#[derive(Debug, Clone, Copy)]
enum HttpStatus {
    Ok = 200,
    Created = 201,
    BadRequest = 400,
    Unauthorized = 401,
    InternalServerError = 500,
}

/// Builder response yang lebih fleksibel
fn response_builder<T: Into<Option<Value>>>(
    status: HttpStatus,
    response_code: i128,
    message: &str,
    additional_info: T,
) -> HttpResponse {
    let mut response = json!({
        "responseCode": response_code,
        "responseMessage": message
    });

    if let Some(Value::Object(map)) = additional_info.into() {
        if let Some(obj) = response.as_object_mut() {
            obj.extend(map);
        }
    }

    match status {
        HttpStatus::Ok => HttpResponse::Ok().json(response),
        HttpStatus::Created => HttpResponse::Created().json(response),
        HttpStatus::BadRequest => HttpResponse::BadRequest().json(response),
        HttpStatus::Unauthorized => HttpResponse::Unauthorized().json(response),
        HttpStatus::InternalServerError => HttpResponse::InternalServerError().json(response),
    }
}

pub struct PostResponse;

impl PostResponse {
    pub fn ok_response(data: Value) -> HttpResponse {
        response_builder(HttpStatus::Ok, 200100, "Get all posts success", data)
    }

    pub fn not_found_response() -> HttpResponse {
        response_builder(HttpStatus::BadRequest, 400212, "Data not found", None)
    }

    pub fn internal_error_response(message: String) -> HttpResponse {
        response_builder(HttpStatus::InternalServerError, 59999, message.as_str(), None)
    }

    pub fn created_response(new_post: Value) -> HttpResponse {
        response_builder(HttpStatus::Created, 201111111, "New post created", new_post)
    }
}

// pub struct SignatureAccessToken;


// impl SignatureAccessToken {
//     pub fn signature_success_response_builder(signature: String) -> HttpResponse {
//         let data = json!({
//             "signature": signature
//         });
//         response_builder(200, 200000, "Success".to_string(), Some(data))
//     }

//     pub fn signature_unauthorize_response_builder()->HttpResponse{
//         let response_code = 4017300;
//         let message:String = String::from("Unauthorized stringToSign");
//         response_builder(401,response_code , message, None)
//     }
// }
