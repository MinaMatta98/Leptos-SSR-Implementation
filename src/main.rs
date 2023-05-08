#![feature(proc_macro_hygiene, decl_macro)]
#![feature(iterator_try_collect)]
#![feature(type_alias_impl_trait)]
pub mod pages;
pub use augment;
pub use footer;
pub use index;
pub use navbar;
pub use pages::*;
pub mod functions;
pub mod styles;
// use actix_web::web;
use actix_web::{get, http::StatusCode, App, HttpResponse, HttpServer, Responder, Result};
pub use actix_web::{post, web::Bytes, web::BytesMut, web::Payload};
pub use footer::{Footer, FooterProps};
use functions::*;
pub use index::*;
use leptos::*;
use leptos_actix::*;

#[post("/submit")]
pub async fn upload_server(mut form_data: Payload) -> Result<String, std::io::Error> {
    use futures_util::StreamExt;

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let pdf_path = format!("upload/{}{}", timestamp, ".pdf");

    let pdf_thumbnail_path = format!("upload/thumbnails/{}{}", timestamp, ".png");

    let mut bytes = BytesMut::new();

    while let Some(item) = form_data.next().await {
        bytes.extend_from_slice(&item.expect("Internal Server Error. Try again in a few minutes"));
    }

    std::fs::write(&pdf_path, bytes)?;

    create_thumbnail(
        std::path::Path::new(&pdf_path),
        std::path::Path::new(&pdf_thumbnail_path),
    )
    .await?;
    Ok(String::from(""))
    // Err(std::io::Error::new(std::io::ErrorKind::Other, "error"))
}

#[get("/pdf/{id}")]
async fn pdf(path: actix_web::web::Path<String>) -> Result<impl Responder> {
    use actix_web::http::header::ContentType;

    let binding = format!("../target/upload/{path}.pdf");

    let pdf_path = std::path::Path::new(&binding).to_owned();

    std::fs::metadata(&pdf_path)?;

    let pdf_file = tokio::fs::read(&pdf_path).await?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType(mime_guess::mime::APPLICATION_PDF))
        .status(StatusCode::OK)
        .body(pdf_file))
}

async fn create_thumbnail(
    pdf_path: &std::path::Path,
    out_path: &std::path::Path,
) -> std::result::Result<(), std::io::Error> {
    use cairo::{Format, ImageSurface};
    use poppler::PopplerDocument;

    let doc = PopplerDocument::new_from_file(pdf_path, "")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Unsupported, e.message()))?;

    let page = doc.get_page(0).ok_or(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Invalid Pdf for conversion",
    ))?;

    let input_dimensions = page.get_size();
    let output_dimensions = (input_dimensions.0 / 5.0, input_dimensions.1 / 5.0);
    let surface = ImageSurface::create(
        Format::Rgb24,
        output_dimensions.0 as i32,
        output_dimensions.1 as i32,
    )
    .unwrap();
    // surface.set_device_scale(0.07, 0.1);
    let ctxt = cairo::Context::new(&surface).unwrap();
    ctxt.scale(
        output_dimensions.0 / input_dimensions.0,
        output_dimensions.1 / input_dimensions.1,
    );
    ctxt.set_source_surface(&surface, 0.0, 0.0).unwrap();
    ctxt.set_source_rgb(1.0, 1.0, 1.0);
    ctxt.paint().unwrap();
    page.render(&ctxt);

    let mut f = std::fs::File::create(out_path).unwrap();
    surface.write_to_png(&mut f).unwrap();
    Ok(())
}

#[cfg(feature = "ssr")]
#[get("/thumbnails/{id}")]
async fn thumbnails(path: actix_web::web::Path<String>) -> Result<impl Responder> {
    let thumbnail = tokio::fs::read(format!("../target/upload/thumbnails/{}", path)).await?;

    let mut bytes = BytesMut::new();
    for byte in thumbnail.into_iter() {
        bytes.extend_from_slice(&[byte])
    }
    let stream = futures_util::stream::once(async { Ok(actix_web::web::Bytes::from(bytes)) });
    Ok(HttpResponse::Ok().streaming::<_, std::io::Error>(stream))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::middleware::{Compress, Logger, NormalizePath};
    set_env();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let conf = leptos::get_configuration(Some("../Cargo.toml"))
        .await
        .unwrap();
    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;

        let _ = augment::ExampleFunc::register();
        let _ = augment::ThumbnailRenderer::register();
        let _ = augment::callback::RemoveFile::register();
        let _ = augment::callback::Random::register();
        let routes = generate_route_list(|cx| {
            view! {cx,
                <RenderIndex/>
            }
        });
        println!("{routes:?}");
        // let site_root = &leptos_options.site_root;
        App::new()
            .service(upload_server)
            .service(pdf)
            .service(thumbnails)
            .route("/{tail:.*}", leptos_actix::handle_server_fns())
            // .app_data(TempFileConfig::default().directory("./upload"))
            .wrap(Logger::new("%r %U").log_target("actix"))
            .wrap(NormalizePath::new(
                actix_web::middleware::TrailingSlash::Trim,
            ))
            .wrap(Compress::default())
            .leptos_routes(
                leptos_options.to_owned(),
                routes,
                |cx| view! { cx, <RenderIndex/> },
            )
            .service(actix_files::Files::new("/", "../target/site"))
    })
    .workers(4)
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

fn set_env() {
    std::env::set_current_dir(std::env::current_exe().unwrap().ancestors().nth(3).unwrap())
        .unwrap();
}
