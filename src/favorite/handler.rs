use actix_web::{web, HttpRequest, HttpResponse};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde_json::json;

use crate::{
    album::model::Album, auth::get_current_user, error::AppError, schema::albums, AppState,
};

use super::{model::Favorite, response::FavoriteResponse};

pub async fn search_favorites(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let albums = Favorite::find_by_user_id(&mut conn, current_user.id)?;
    let res = FavoriteResponse::from(albums);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn add_to_favorites(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let album_id = params.into_inner();
    let n = Favorite::create(&mut conn, current_user.id, album_id)?;
    Ok(HttpResponse::Ok().json(json!({"result": n})))
}

pub async fn remove_from_favorites(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let album_id = params.into_inner();
    let n = Favorite::delete(&mut conn, current_user.id, album_id)?;
    Ok(HttpResponse::Ok().json(json!({"result": n})))
}
