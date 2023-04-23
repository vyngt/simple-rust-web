use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

mod views;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");

    format!("Hello {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .configure(views::views_factory) // extract view
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/{name}", web::get().to(greet));

        return app;
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
