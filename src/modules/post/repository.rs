use sqlx::query_as;
use crate::DbAppState;

use super::models::{NewPost, Post, UpdatePost};

pub struct PostRepository;

impl PostRepository {
    pub async fn get_all_posts(
        db: &DbAppState
    )-> Result<Vec<Post>,sqlx::Error>{
        let query = query_as!(
            Post,
            r#"SELECT * FROM posts ORDER BY id"#
        ).fetch_all(db).await;
    
        match query{
            Ok(data)=> Ok(data),
            Err(error)=>Err(error)
        }
    }

    pub async fn get_post_by_id(
        db: &DbAppState,
        id: i32
    ) -> Result<Post,sqlx::Error>{
        let query = query_as!(
            Post,
            r#"SELECT * FROM posts where id = $1"#,
            id
        ).fetch_one(db).await;
    
        match query{
            Ok(data)=> Ok(data),
            Err(error)=>Err(error)
        }
    }

    pub async fn create_post(
        db: &DbAppState,
        data:NewPost
    )-> Result<Post,sqlx::Error>{
        let query= query_as!(
            Post,
            r#"
            INSERT INTO posts (title, content)
            VALUES ($1, $2) RETURNING *
            "#,
            data.title,
            data.content
        ).fetch_one(db).await;

        match query {
            Ok(new_post)=>Ok(new_post),
            Err(error)=>Err(error)
        }
    }

    pub async fn update_post(db: &DbAppState, id: i32, data: UpdatePost) -> Result<Post, sqlx::Error> {
        let query = query_as!(
            Post,
            r#"
            UPDATE posts
            SET 
                title = COALESCE($1, title),
                content = COALESCE($2, content)
            WHERE id = $3
            RETURNING *;
            "#,
            data.title,
            data.content,
            id
        )
        .fetch_one(db)
        .await;

        match query {
            Ok(updated_post) => Ok(updated_post),
            Err(error) => Err(error),
        }
    }

    pub async fn delete_post(db: &DbAppState, id: i32) -> Result<(), sqlx::Error> {
        let query = sqlx::query!(
            r#"DELETE FROM posts WHERE id = $1"#,
            id
        )
        .execute(db)
        .await;

        match query {
            Ok(result) => {
                if result.rows_affected() > 0 {
                    Ok(())
                } else {
                    Err(sqlx::Error::RowNotFound)
                }
            }
            Err(error) => Err(error),
        }
    }
}