use crate::DbAppState;

use super::{models::{Post, UpdatePost}, repository::PostRepository};

pub struct PostUsecase;

impl PostUsecase{
    pub async fn validate_post(post_id: i32,db: &DbAppState)->Result<Post,String>{
        let post = PostRepository::get_post_by_id(db, post_id).await;

        match post {
            Ok(post_value)=>{
                Ok(post_value)
            },
            Err(error)=>{
                let err = format!("{}",error);
                if err.contains("no rows returned"){
                    return Err(String::from("post not found"))
                }
                Err(err)
            }
        }
    }

    pub async fn update_post(post_id: i32, data: UpdatePost, db: &DbAppState)->Result<Post,String>{
        match Self::validate_post(post_id, &db).await{
            Ok(old_post)=>{
                let updated_post= PostRepository::update_post(db, old_post.id, data).await;

                match updated_post {
                    Ok(post)=>Ok(post),
                    Err(error)=>{
                        let err= format!("{}",error);

                        Err(err)
                    }
                }
            }
            Err(err)=>return Err(err)
        }
    }

    pub async fn delete_post(post_id: i32, db: &DbAppState)->Result<(),String>{
        match Self::validate_post(post_id, &db).await{
            Ok(old_post)=>{
                let updated_post= PostRepository::delete_post(db, old_post.id).await;

                match updated_post {
                    Ok(_)=>Ok(()),
                    Err(error)=>{
                        let err= format!("{}",error);

                        Err(err)
                    }
                }
            }
            Err(err)=>return Err(err)
        }
    }
}