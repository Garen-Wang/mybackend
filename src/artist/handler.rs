use actix_web::{web, HttpRequest, HttpResponse};

use crate::{artist::response::ArtistResponse, auth::get_current_user, error::AppError, AppState};

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
