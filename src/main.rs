use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use capture::Capture;
use clap::Parser;
use cli::Cli;
use log::{error, info, LevelFilter};
use std::{path::PathBuf, process::exit};

mod capture;
mod cli;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_module_level("actix_server", LevelFilter::Off)
        .init()
        .unwrap();

    let bind_ip = cli.ip.as_deref().unwrap_or("0.0.0.0");
    let bind_port = cli.port.unwrap_or(8080);
    let webroot = cli.webroot.unwrap_or_else(|| PathBuf::from("www"));
    let output_path = cli.output.unwrap_or_else(|| PathBuf::from("capture.csv"));
    let capturer = Data::new(Capture::new(output_path).await?);

    if !webroot.exists() {
        error!("Specified webroot directory does not exist");
        exit(1);
    }

    info!("Serving on http://{}:{}/", bind_ip, bind_port);

    HttpServer::new(move || {
        App::new()
            .app_data(capturer.clone())
            .service(web::scope("/api").service(routes::login))
            .service(actix_files::Files::new("/", &webroot).index_file("index.html"))
    })
    .bind((bind_ip, bind_port))?
    .run()
    .await
}
