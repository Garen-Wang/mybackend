use actix_web::{web, HttpResponse};

use crate::{error::AppError, AppState, artist::response::ArtistResponse};

use super::model::Artist;

pub async fn search_artists_by_name(
    app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let name = params.into_inner();
    let mut conn = app_state.conn()?;
    let artists = Artist::search(&mut conn, name)?;
    let res = ArtistResponse::from(artists);
    Ok(HttpResponse::Ok().json(res))
}

