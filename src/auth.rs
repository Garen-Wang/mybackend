use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::Method,
    web, Error, HttpMessage, HttpRequest, HttpResponse,
};
use diesel::PgConnection;
use futures::future::LocalBoxFuture;
use lazy_static::lazy_static;
use serde_json::json;
use std::future::{ready, Ready};

use crate::{error::AppError, token, user::model::User, AppState};

struct SkipAuthRoute {
    path: &'static str,
    method: Method,
}

impl SkipAuthRoute {
    fn is_slug_path(text: &str) -> bool {
        let first = text.chars().next().unwrap_or(' ');
        let last = text.chars().last().unwrap_or(' ');
        first == '{' && last == '}'
    }

    fn matches_path_and_method(&self, path: &str, method: &Method) -> bool {
        self.matches_path(path) && self.matches_method(method)
    }

    fn matches_path(&self, path: &str) -> bool {
        let self_paths: Vec<&str> = self.path.split('/').collect();
        let paths: Vec<&str> = path.split('/').collect();
        if self_paths.len() != paths.len() {
            return false;
        }
        for (self_path, path) in self_paths.iter().zip(paths.iter()) {
            if Self::is_slug_path(self_path) {
                continue;
            }
            if self_path != path {
                return false;
            }
        }
        return true;
    }

    fn matches_method(&self, method: &Method) -> bool {
        self.method == method
    }
}

lazy_static! {
    // TODO: not complete
    static ref SKIP_AUTH_ROUTES: Vec<SkipAuthRoute> = vec![
        SkipAuthRoute {
            path: "/healthcheck",
            method: Method::GET,
        },
        SkipAuthRoute {
            path: "/auth/login",
            method: Method::POST,
        },
        SkipAuthRoute {
            path: "/auth/register",
            method: Method::POST,
        },
        SkipAuthRoute {
            path: "/artist/{artist_name}",
            method: Method::GET,
        },
        SkipAuthRoute {
            path: "/album/{album_name}",
            method: Method::GET,
        },
        SkipAuthRoute {
            path: "/comment/{album_id}",
            method: Method::GET,
        },
    ];
}

fn should_skip_auth(req: &ServiceRequest) -> bool {
    let method = req.method();
    if method == Method::OPTIONS {
        true
    } else {
        SKIP_AUTH_ROUTES
            .iter()
            .any(|route| route.matches_path_and_method(req.path(), req.method()))
    }
}

fn get_user_id_from_header(req: &ServiceRequest) -> Result<i32, &str> {
    req.headers()
        .get("authorization")
        .ok_or("authorization key-value not found in key-value header")
        .and_then(|header| header.to_str().map_err(|_| "cannot stringify"))
        .and_then(|str| {
            if str.starts_with("Bearer") {
                log::debug!("yes! get user id from header!");
                Ok(str.trim_start_matches("Bearer").trim())
            } else {
                Err("Invalid token")
            }
        })
        .and_then(|str| token::decode_token(str).map_err(|_| "cannot decode token"))
        .map(|token| token.claims.user_id)
}

fn find_auth_user(conn: &mut PgConnection, user_id: i32) -> Result<User, AppError> {
    let user = User::find(conn, user_id)?;
    Ok(user)
}

// used when request provided
fn fetch_user(req: &ServiceRequest) -> Result<User, &str> {
    let user_id = get_user_id_from_header(req)?;

    let mut conn = req
        .app_data::<web::Data<AppState>>()
        .ok_or("cannot get state")
        .and_then(|app_state| app_state.conn().map_err(|_| "cannot get db conn"))?;

    let user = find_auth_user(&mut conn, user_id).map_err(|_| "cannot find auth user")?;
    Ok(user)
}

// used when requeset provided
fn set_auth_user(req: &mut ServiceRequest) -> bool {
    match fetch_user(req) {
        Ok(user) => {
            req.extensions_mut().insert(user);
            true
        }
        Err(e) => {
            log::info!("cannot fetch user. {}", e);
            false
        }
    }
}

// get from request local data
pub fn get_current_user(req: &HttpRequest) -> Result<User, AppError> {
    let user = req.extensions().get::<User>().map(|user| user.to_owned());
    user.ok_or(AppError::Unauthorized(json!({
        "error": "unauthorized user. need a auth token in header"
    })))
}

// introducing authentication middleware
// and authorization (impl Transform)

pub struct Authorization;

impl<S, B> Transform<S, ServiceRequest> for Authorization
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;

    type Error = Error;

    type Transform = AuthorizationMiddleware<S>;

    type InitError = ();

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthorizationMiddleware { service }))
    }
}

pub struct AuthorizationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthorizationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;

    type Error = Error;

    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        log::info!("Hi from start. You requested: {}", req.path());
        let verified = if should_skip_auth(&req) {
            true
        } else {
            set_auth_user(&mut req)
        };
        if verified {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?.map_into_left_body();
                log::info!("Hi from response. Verified.");
                Ok(res)
            })
        } else {
            Box::pin(async move {
                let (req, _payload) = req.into_parts();
                let res = HttpResponse::Unauthorized().finish().map_into_right_body();
                log::info!("Hi from response. Not verified.");
                Ok(ServiceResponse::new(req, res))
            })
        }
        // let fut = self.service.call(req);
        // Box::pin(async move {
        //     let res = fut.await?;
        //     log::info!("Hi from response.");
        //     Ok(res)
        // })
    }
}

#[cfg(test)]
mod tests {
    use actix_web::http::Method;

    use super::SkipAuthRoute;

    #[test]
    fn test_match_path_and_method() {
        let route = SkipAuthRoute {
            path: "/healthcheck",
            method: Method::GET,
        };
        assert!(route.matches_path_and_method("/healthcheck", &Method::GET));
        let route = SkipAuthRoute {
            path: "/auth/login",
            method: Method::GET,
        };
        assert!(route.matches_path_and_method("/auth/login", &Method::GET));
        let route = SkipAuthRoute {
            path: "/{slug}/healthcheck",
            method: Method::GET,
        };
        assert!(route.matches_path_and_method("/6324/healthcheck", &Method::GET));
    }
}
