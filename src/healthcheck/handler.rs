use actix_web::HttpResponse;
use serde_json::json;

pub async fn healthcheck1() -> HttpResponse {
    HttpResponse::Ok().body("ok")
}

pub async fn healthcheck2() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "name": "jrqjrqjrq",
        "array": [1, 2, 3],
        "objects": [
            {"key": 1, "value": "j"},
            {"key": 2, "value": "r"},
            {"key": 3, "value": "q"},
        ],
    }))
}