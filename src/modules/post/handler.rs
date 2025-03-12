use crate::{modules::post::{models::UpdatePost, services::PostServices}, AppState};
use super::models::NewPost;
use actix_web::{
    delete, get, patch, post, web::{self, scope, Json, Path, ServiceConfig}, Responder
};
use serde_json::json;
use crate::utils::response_builder::PostResponse;

#[post("")]
pub async fn create_post(data: web::Data<AppState>, post_data: Json<NewPost>) -> impl Responder {
    let post = post_data.into_inner();

    let result = PostServices::create_post(&data.db, post).await;
    match result {
        Ok(new_post) => {
            let json_data = json!({"new post": new_post});
            PostResponse::created_response(json_data)
        },
        Err(error) => PostResponse::internal_error_response(error),
    }
}

#[get("")]
pub async fn get_all_post(data: web::Data<AppState>) -> impl Responder {
    let result = PostServices::get_all_post_query(&data.db).await;
    
    match result {
        Ok(posts) => {
            let json_data = json!({"data": posts});
            PostResponse::ok_response(json_data)
        },
        Err(error) => {
            if error.contains("not found") {
                PostResponse::not_found_response()
            } else {
                PostResponse::internal_error_response(error)
            }
        }
    }
}

#[get("/{post_id}")]
pub async fn get_one_post(data: web::Data<AppState>, path: Path<i32>) -> impl Responder {
    let post_id = path.into_inner();

    let result = PostServices::get_post_by_id(&data.db, post_id).await;
    match result {
        Ok(post) => {
            let json_data = json!({"data": post});
            PostResponse::ok_response(json_data)
        },
        Err(error) => PostResponse::internal_error_response(error),
    }
}

#[patch ("/{post_id}")]
pub async fn update_post(data: web::Data<AppState>, path: Path<i32>, post_data: Json<UpdatePost>) -> impl Responder {
    let post_id = path.into_inner();
    let updated_data = post_data.into_inner();

    let result = PostServices::update_post(&data.db, post_id, updated_data).await;
    match result {
        Ok(updated_post) => {
            let json_data = json!({"updated post": updated_post});
            PostResponse::ok_response(json_data)
        },
        Err(error) => {
            if error.contains("not found"){
                return PostResponse::not_found_response()
            }
            PostResponse::internal_error_response(error)
        },
    }
}

#[delete("/{post_id}")]
pub async fn delete_post(data: web::Data<AppState>, path: Path<i32>) -> impl Responder {
    let post_id = path.into_inner();

    let result = PostServices::delete_post(&data.db, post_id).await;
    match result {
        Ok(_) => PostResponse::ok_response(json!({"message": "Post deleted successfully"})),
        Err(error) => {
            if error.contains("not found"){
                return PostResponse::not_found_response()
            }
            PostResponse::internal_error_response(error)
        }
    }
}

pub fn post_config(config: &mut ServiceConfig) {
    config.service(
        scope("/post")
            .service(get_all_post)
            .service(get_one_post)
            .service(create_post)
            .service(update_post)
            .service(delete_post)
    );
}
