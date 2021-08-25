use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

use rocket::{fs::NamedFile, http::ContentType, tokio::io::AsyncReadExt};

use crate::rocket;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("public/index.html")).await.ok()
}

#[get("/static/<file..>")]
async fn static_file(file: PathBuf) -> Option<NamedFile> {
    let result = NamedFile::open(Path::new("public/").join(file)).await.ok();

    result
}

pub async fn serve() {
    rocket::build()
        .mount("/", routes![index, static_file])
        .launch()
        .await
        .unwrap();
}
