use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use futures::TryStreamExt;
use serde_json::json;
use uuid::Uuid;

use crate::{album::response::AlbumResponse, auth::get_current_user, error::AppError, AppState};

use super::{model::{Album, Track}, request::{CreateAlbumRequest, AddTrackRequest}, response::TrackResponse};

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
        Ok(HttpResponse::Ok().json(json!({ "result": n })))
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
        // no permission
        Err(AppError::InternalServerError)
    }
}

// POST
pub async fn create_album(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<CreateAlbumRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let album = Album::create(
        &mut conn,
        form.new_album.artist_id,
        &form.new_album.name,
        current_user.is_admin,
    )?;
    let res = AlbumResponse::from(album);
    Ok(HttpResponse::Ok().json(res))
}

// POST
pub async fn add_track_to_album(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
    form: web::Json<AddTrackRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let _current_user = get_current_user(&req)?;
    let album_id = params.into_inner();
    let track = Track::create(&mut conn, &form.new_track.name, form.new_track.artist_id, album_id)?;
    let res = TrackResponse::from(track);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn upload_audio(
    params: web::Path<i32>,
    mut payload: Multipart
) -> Result<HttpResponse, AppError> {
    let track_id = params.into_inner();
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();

        let original_filename = content_disposition.get_filename().unwrap();
        let new_filename = if let Some((_, ext)) = original_filename.rsplit_once('.') {
            format!("{}{}", track_id, ext)
        } else {
            sanitize_filename::sanitize(Uuid::new_v4().to_string())
        };
        
        let filepath = format!("./static/audio/{new_filename}");

        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        while let Some(chunk) = field.try_next().await? {
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }
    Ok(HttpResponse::Ok().json(json!({"result": 1})))
}
