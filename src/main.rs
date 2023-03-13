use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use webdocker::run;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World, This a feature extractor!")
}

#[post("/predict")]
async fn predict(image: web::Bytes) -> impl Responder {
    let img = image::load_from_memory(&image)
        .expect("Failed to load image from bytes");

    match run(&img) {
        Ok(output) => {
            let output_json = serde_json::to_string(&output).expect("Failed to serialize output as JSON");
            HttpResponse::Ok().body(output_json.into_bytes())
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running the service");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(predict)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
