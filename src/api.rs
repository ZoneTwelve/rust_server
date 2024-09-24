use actix_web::{get, HttpResponse, Responder, web};

#[get("/info")]
async fn info() -> impl Responder {
    HttpResponse::Ok().body("API Info: service is ready!")
}

// Add other API routes here as needed
#[get("/status")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("API Status: All good!")
}

// Function to configure and register all routes for this API module
pub fn init_api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(info)
       .service(status);
}
