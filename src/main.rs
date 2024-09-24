use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware};
use env_logger;
use env_logger::Env;

mod api;  // Add this line to declare the api module

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/")]
async fn hello_in_post() -> impl Responder {
    HttpResponse::Ok().body("Hello world! in POST")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    
    env_logger::init_from_env(Env::default().default_filter_or("info"));


    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .service(hello)
            .service(web::scope("/api").configure(api::init_api_routes))  // Scope everything under /api

            // .service(info)
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

