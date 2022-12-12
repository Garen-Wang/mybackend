use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{auth::get_current_user, error::AppError, AppState};

use super::{model::Comment, request::CommentRequest, response::CommentResponse};

// POST
pub async fn add_comment(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
    form: web::Json<CommentRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let album_id = params.into_inner();
    let a = Comment::create(&mut conn, album_id, current_user.id, &form.comment.body)?;
    Ok(HttpResponse::Ok().json(json!({ "result": a })))
}

// DELETE
pub async fn delete_comment(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let comment_id = params.into_inner();
    let a = Comment::delete(
        &mut conn,
        comment_id,
        if current_user.is_admin {
            None
        } else {
            Some(current_user.id)
        },
    )?;
    Ok(HttpResponse::Ok().json(json!({ "result": a })))
}

// GET
pub async fn get_comments_of_album(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let album_id = params.into_inner();
    let comments = Comment::find_by_album_id(&mut conn, album_id)?;
    let res = CommentResponse::from(comments);
    Ok(HttpResponse::Ok().json(res))
}
