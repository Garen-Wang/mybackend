use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{auth::get_current_user, error::AppError, AppState, album::model::Album, artist::model::Artist};

use super::{
    model::{FavoriteAlbum, FavoriteArtist},
    response::{FavoriteAlbumResponse, FavoriteArtistResponse},
};

pub async fn search_favorite_albums(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let albums = FavoriteAlbum::find_by_user_id(&mut conn, current_user.id)?;
    let res = FavoriteAlbumResponse::from((albums, &mut conn));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn add_to_favorite_albums(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let album_id = params.into_inner();
    let _album = Album::find(&mut conn, album_id)?;
    let n = FavoriteAlbum::create(&mut conn, current_user.id, album_id)?;
    Ok(HttpResponse::Ok().json(json!({ "result": n })))
}

pub async fn remove_from_favorite_albums(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let album_id = params.into_inner();
    let _album = Album::find(&mut conn, album_id)?;
    let n = FavoriteAlbum::delete(&mut conn, current_user.id, album_id)?;
    Ok(HttpResponse::Ok().json(json!({ "result": n })))
}

pub async fn search_favorite_artists(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let artists = FavoriteArtist::find_by_user_id(&mut conn, current_user.id)?;
    let res = FavoriteArtistResponse::from((artists, &mut conn));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn add_to_favorite_artists(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let artist_id = params.into_inner();
    let _artist = Artist::find(&mut conn, artist_id)?;
    let n = FavoriteArtist::create(&mut conn, current_user.id, artist_id)?;
    Ok(HttpResponse::Ok().json(json!({ "result": n })))
}

pub async fn remove_from_favorite_artists(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let artist_id = params.into_inner();
    let _artist = Artist::find(&mut conn, artist_id)?;
    let n = FavoriteArtist::delete(&mut conn, current_user.id, artist_id)?;
    Ok(HttpResponse::Ok().json(json!({ "result": n })))
}
