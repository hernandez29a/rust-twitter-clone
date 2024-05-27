//127.0.0.1:8000/tweets -> GEt: obtiene tweets. POST: crea un tweet
//127.0.0.1:8000/tweets/:id ->  GET: se obtiene con Id, DELETE se borra tweet

//127.0.0.1:8000/tweets/:id/likes -> GET: obtiene likes, 
//                                  -> POST: crea un like
//                                  -> DELETE: borra un like
use actix_web::{web, App, HttpServer, Responder, HttpResponse, get, post, delete, put};
use actix_rt; // Add this line

async fn saludar() -> impl Responder {
    HttpResponse::Ok().body("Hola mundo")
}

#[actix_rt::main] // Change this line
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(saludar));
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
