use actix_web::{web, HttpRequest, HttpResponse};

use crate::{auth::get_current_user, error::AppError, AppState};

use super::{
    model::{UpdateUserTrackHistory, UserTrackHistory, UserAlbumHistory, UpdateUserAlbumHistory},
    request::{UpdateUserTrackHistoryRequest, UpdateUserAlbumHistoryRequest},
    response::{UserTrackHistoryResponse, UserAlbumHistoryResponse},
};

pub async fn search_last_playback_of_track(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let track_id = params.into_inner();
    let history = UserTrackHistory::find_by_user_track_id(&mut conn, current_user.id, track_id)?;
    let res = UserTrackHistoryResponse::from(history);
    Ok(HttpResponse::Ok().json(res))
}

// insert or update
pub async fn update_last_playback_of_track(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
    form: web::Json<UpdateUserTrackHistoryRequest>, // TODO: form
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let track_id = params.into_inner();
    // if not exist, insert
    if !UserTrackHistory::exists(&mut conn, current_user.id, track_id)? {
        let _item = UserTrackHistory::create(&mut conn, current_user.id, track_id)?;
    }
    let changeset = UpdateUserTrackHistory {
        last_time: form.latest_playback.last_time,
        last_date: form.latest_playback.last_date,
    };
    let history = UserTrackHistory::update(&mut conn, current_user.id, track_id, changeset)?;
    let res = UserTrackHistoryResponse::from(history);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn search_last_playback_of_album(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let album_id = params.into_inner();
    let history = UserAlbumHistory::find_by_user_album_id(&mut conn, current_user.id, album_id)?;
    let res = UserAlbumHistoryResponse::from(history);
    Ok(HttpResponse::Ok().json(res))
}

// insert or update
pub async fn update_last_playback_of_album(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
    form: web::Json<UpdateUserAlbumHistoryRequest>, // TODO: form
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let album_id = params.into_inner();
    // if not exist, insert
    if !UserAlbumHistory::exists(&mut conn, current_user.id, album_id)? {
        let _item = UserAlbumHistory::create(&mut conn, current_user.id, album_id)?;
    }
    let changeset = UpdateUserAlbumHistory {
        last_time: form.latest_playback.last_time,
        last_date: form.latest_playback.last_date,
    };
    let history = UserAlbumHistory::update(&mut conn, current_user.id, album_id, changeset)?;
    let res = UserAlbumHistoryResponse::from(history);
    Ok(HttpResponse::Ok().json(res))
}