use std::path::Path;

use rocket::fs::NamedFile;

use crate::rocket;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("public/index.html")).await.ok()
}

#[get("/static/<file>")]
async fn static_file(file: String) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).await.ok()
}

pub async fn serve() {
    rocket::build()
        .mount("/", routes![index, static_file])
        .launch()
        .await
        .unwrap();
}
