use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{
    album::response::AlbumResponse, artist::model::Artist, auth::get_current_user, error::AppError,
    AppState,
};

use super::{
    model::{Album, Track},
    request::CreateAlbumRequest,
    response::TrackResponse,
};

pub async fn search_albums_by_name(
    app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let name = params.into_inner();
    let mut conn = app_state.conn()?;
    let albums = Album::search(&mut conn, name)?;
    let res = AlbumResponse::from((albums, &mut conn));
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
        let tracks = Track::find_by_album(&mut conn, album_id)?;
        for track in tracks {
            Track::delete(&mut conn, track.id)?;
        }
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
        let res = AlbumResponse::from((album, &mut conn));
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
    let artist = if let Ok(artist) = Artist::find_by_name(&mut conn, &form.new_album.artist_name) {
        artist
    } else {
        Artist::create(&mut conn, &form.new_album.artist_name)?
    };
    let album = Album::create(
        &mut conn,
        artist.id,
        &form.new_album.album_name,
        current_user.is_admin,
    )?;
    for track in form.new_album.tracks.iter() {
        Track::create(&mut conn, &track.name, &track.url, artist.id, album.id)?;
    }
    let res = AlbumResponse::from((album, &mut conn));
    Ok(HttpResponse::Ok().json(res))
}

// GET
pub async fn get_album_by_id(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let _current_user = get_current_user(&req)?;
    let album_id = params.into_inner();
    let album = Album::find(&mut conn, album_id)?;
    let res = AlbumResponse::from((album, &mut conn));
    Ok(HttpResponse::Ok().json(res))
}

// pub async fn add_track_to_album(
//     app_state: web::Data<AppState>,
//     req: HttpRequest,
//     params: web::Path<i32>,
//     form: web::Json<AddTrackRequest>,
// ) -> Result<HttpResponse, AppError> {
//     let mut conn = app_state.conn()?;
//     let _current_user = get_current_user(&req)?;
//     let album_id = params.into_inner();
//     let track = Track::create(&mut conn, &form.new_track.name, form.new_track.artist_id, album_id)?;
//     let res = TrackResponse::from(track);
//     Ok(HttpResponse::Ok().json(res))
// }

// pub async fn upload_audio(
//     params: web::Path<i32>,
//     req: HttpRequest,
//     mut payload: Multipart
// ) -> Result<HttpResponse, AppError> {
//     let track_id = params.into_inner();
//     let _current_user = get_current_user(&req)?;
//     while let Some(mut field) = payload.try_next().await? {
//         let content_disposition = field.content_disposition();

//         let original_filename = content_disposition.get_filename().unwrap();
//         let new_filename = if let Some((_, ext)) = original_filename.rsplit_once('.') {
//             format!("{}{}", track_id, ext)
//         } else {
//             sanitize_filename::sanitize(Uuid::new_v4().to_string())
//         };

//         let filepath = format!("./static/audio/{new_filename}");

//         let mut f = web::block(|| std::fs::File::create(filepath)).await??;

//         while let Some(chunk) = field.try_next().await? {
//             f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
//         }
//     }
//     Ok(HttpResponse::Ok().json(json!({"result": 1})))
// }

pub async fn get_all_albums(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let albums = if current_user.is_admin {
        Album::get_all(&mut conn)?
    } else {
        Album::get_all_issued(&mut conn)?
    };
    let res = AlbumResponse::from((albums, &mut conn));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_all_unissued_albums(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    if current_user.is_admin {
        let albums = Album::get_all_unissued(&mut conn)?;
        let res = AlbumResponse::from((albums, &mut conn));
        Ok(HttpResponse::Ok().json(res))
    } else {
        Err(AppError::InternalServerError)
    }
}

// GET
pub async fn get_all_tracks(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let _current_user = get_current_user(&req)?;
    let tracks = Track::get_all(&mut conn)?;
    let res = TrackResponse::from((tracks, &mut conn));
    Ok(HttpResponse::Ok().json(res))
}

// GET
pub async fn play_album(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let _current_user = get_current_user(&req)?;
    let album_id = params.into_inner();
    let album = Album::play(&mut conn, album_id)?;
    let res = AlbumResponse::from((album, &mut conn));
    Ok(HttpResponse::Ok().json(res))
}

// GET
pub async fn play_track(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let _current_user = get_current_user(&req)?;
    let track_id = params.into_inner();
    let track = Track::play(&mut conn, track_id)?;
    let album_id = track.album_id;
    let res = TrackResponse::from((track, &mut conn));
    let _album = Album::play(&mut conn, album_id)?;
    Ok(HttpResponse::Ok().json(res))
}
