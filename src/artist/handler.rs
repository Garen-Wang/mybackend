use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{artist::response::ArtistResponse, auth::get_current_user, error::AppError, AppState, album::model::{Album, Track}};

use super::{model::Artist, request::AddArtistRequest};

pub async fn search_artists_by_name(
    app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let name = params.into_inner();
    let mut conn = app_state.conn()?;
    let artists = Artist::search(&mut conn, name)?;
    let res = ArtistResponse::from((artists, &mut conn));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn add_artist(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<AddArtistRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    if current_user.is_admin {
        let artist = Artist::create(&mut conn, &form.new_artist.name)?;
        let res = ArtistResponse::from((artist, &mut conn));
        Ok(HttpResponse::Ok().json(res))
    } else {
        Err(AppError::InternalServerError)
    }
}

pub async fn get_all_artists(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let _current_user = get_current_user(&req)?;
    let artists = Artist::get_all(&mut conn)?;
    let res = ArtistResponse::from((artists, &mut conn));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn delete_artist(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let artist_id = params.into_inner();
    if current_user.is_admin {
        let albums = Album::find_by_artist(&mut conn, artist_id)?;
        for album in albums {
            let tracks = Track::find_by_album(&mut conn, album.id)?;
            for track in tracks {
                Track::delete(&mut conn, track.id)?;
            }
            Album::delete(&mut conn, album.id)?;
        }
        let n = Artist::delete(&mut conn, artist_id)?;
        Ok(HttpResponse::Ok().json(json!({"result": n})))
    } else {
        Err(AppError::InternalServerError)
    }
}