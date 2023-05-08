use actix_web::{App, HttpResponse, HttpServer, Responder, HttpRequest, web};
use actix_web::web::{BytesMut};
use futures::StreamExt;
use std::fs::File;
use std::io::Write;

async fn exfiltrate(req: HttpRequest, mut payload: web::Payload) -> impl Responder {    let content_disposition = req.headers().get("Content-Disposition");
    let filename = match content_disposition {
        Some(cd) => {
            let cd_str = cd.to_str().unwrap();
            let filename_index = cd_str.find("filename=").unwrap();
            let filename = &cd_str[filename_index + 9..];
            filename.trim_matches('"').to_owned()
        }
        None => "uploaded_file".to_owned(),
    };

    let mut body = BytesMut::new();
    while let Some(item) = payload.next().await {
        let chunk = item.unwrap();
        body.extend_from_slice(&chunk);
    }

    // Save file to disk
    let mut file = File::create(&filename).unwrap();
    file.write_all(&body).unwrap();

    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Server started successfully");

    HttpServer::new(|| {
        App::new()
            .route("/", web::post().to(exfiltrate))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
