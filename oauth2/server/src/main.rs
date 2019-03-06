use actix_web::{server, App, HttpRequest, Responder};

const PORT: u16 = 8080;

fn greet(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(greet))
            .resource("/{name}", |r| r.f(greet))
    })
    .bind(&format!("127.0.0.1:{}", PORT))
    .expect(&format!("Can not bind to port {}", PORT))
    .run();
}
