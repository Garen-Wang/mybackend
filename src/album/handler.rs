use std::f32::consts::E;

use actix_web::{web, HttpResponse, HttpRequest};
use serde_json::json;

use crate::{error::AppError, AppState, album::response::AlbumResponse, auth::get_current_user};

use super::model::Album;

pub async fn search_albums_by_name(
    app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let name = params.into_inner();
    let mut conn = app_state.conn()?;
    let albums = Album::search(&mut conn, name)?;
    let res = AlbumResponse::from(albums);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn remove_album(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let album_id = params.into_inner();
    if current_user.is_admin {
        let n = Album::delete(&mut conn, album_id)?;
        Ok(HttpResponse::Ok().json(json!({"result": n})))
    } else {
        // no permission
        Err(AppError::InternalServerError)
    }
}

pub async fn issue_album(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let album_id = params.into_inner();
    if current_user.is_admin {
        let album = Album::issue(&mut conn, album_id)?;
        let res = AlbumResponse::from(album);
        Ok(HttpResponse::Ok().json(res))
    } else {
        Err(AppError::InternalServerError)
    }
}

pub async fn upload_album(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    // TODO: form design
) -> Result<HttpResponse, AppError> {
    unimplemented!()
}