use crate::DbAppState;
use super::{models::{NewPost, Post, UpdatePost}, repository::PostRepository,usecase::PostUsecase};

pub struct PostServices;

impl PostServices {
    pub async fn create_post(db: &DbAppState, data: NewPost) -> Result<Post, String> {
        PostRepository::create_post(db, data)
            .await
            .map_err(|error| format!("{}", error))
    }

    pub async fn get_all_post_query(db: &DbAppState) -> Result<Vec<Post>, String> {
        let posts = PostRepository::get_all_posts(db).await.map_err(|error| format!("{}", error))?;
        if posts.is_empty() {
            Err("data not found".to_string())
        } else {
            Ok(posts)
        }
    }

    pub async fn get_post_by_id(db: &DbAppState, post_id: i32) -> Result<Post, String> {
        PostUsecase::validate_post(post_id, db)
            .await
            .map_err(|error| error)
    }

    pub async fn update_post(db: &DbAppState, post_id: i32, data: UpdatePost) -> Result<Post, String> {
        PostUsecase::update_post(post_id, data,db)
            .await
            .map_err(|error| format!("{}", error))
    }

    pub async fn delete_post(db: &DbAppState, post_id: i32) -> Result<(), String> {
        PostUsecase::delete_post(post_id, db)
            .await
            .map_err(|error| format!("{}", error))
    }
}
