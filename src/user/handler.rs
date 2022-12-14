use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{auth::get_current_user, error::AppError, AppState};

use super::{
    model::{UpdateUser, User},
    request::{LoginRequest, RegisterRequest, UpdateRequest},
    response::UserResponse,
};

pub async fn login(
    app_state: web::Data<AppState>,
    form: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let (user, token) = User::login(&mut conn, &form.user.email, &form.user.password)?;
    let res = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn register(
    app_state: web::Data<AppState>,
    form: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let (user, token) = User::register(
        &mut conn,
        &form.user.email,
        &form.user.username,
        &form.user.password,
    )?;
    let res = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_myself(req: HttpRequest) -> Result<HttpResponse, AppError> {
    let user = get_current_user(&req)?;
    let token = user.generate_token()?;
    let res = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn update(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<UpdateRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let updated_user = User::update(
        &mut conn,
        current_user.id,
        UpdateUser {
            email: form.user.email.clone(),
            username: form.user.username.clone(),
            password: form.user.password.clone(),
        },
    )?;
    let token = updated_user.generate_token()?;
    let res = UserResponse::from((updated_user, token));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_all_users(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    if current_user.is_admin {
        let users = User::get_all(&mut conn)?;
        let res = UserResponse::from(users);
        Ok(HttpResponse::Ok().json(res))
    } else {
        Err(AppError::InternalServerError)
    }
}

pub async fn remove_user(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let user_id = params.into_inner();
    if current_user.is_admin {
        let x = User::delete(&mut conn, user_id)?;
        Ok(HttpResponse::Ok().json(json!({"result": x})))
    } else {
        Err(AppError::InternalServerError)
    }
}