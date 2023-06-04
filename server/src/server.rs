use std::{
    env,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    path::PathBuf,
    str::FromStr,
};

use axum::{
    body::{boxed, Body},
    http::{Response, StatusCode},
    routing::get,
    Router,
};
use sea_orm::{Database, DatabaseConnection};
use tokio::fs;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::{services::ServeDir, trace::TraceLayer};

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

pub async fn run_server() {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap_or("sqlite://reader.db?mode=rwc".to_string());
    let static_dir = env::var("STATIC_DIR").unwrap_or("./dist".to_string());
    let addr = env::var("ADDR").unwrap_or("::1".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .unwrap();
    let log_level = env::var("LOG_LEVEL").unwrap_or("debug".to_string());
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", log_level))
    }

    tracing_subscriber::fmt::init();

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    // TODO: Migrator::up(&conn, None).await.unwrap();

    let state = AppState { conn };

    let app = Router::new()
        .fallback_service(get(|req| async move {
            match ServeDir::new(&static_dir).oneshot(req).await {
                Ok(res) => {
                    let status = res.status();
                    match status {
                        StatusCode::NOT_FOUND => {
                            let index_path = PathBuf::from(&static_dir).join("index.html");
                            let index_content = match fs::read_to_string(index_path).await {
                                Err(_) => {
                                    return Response::builder()
                                        .status(StatusCode::NOT_FOUND)
                                        .body(boxed(Body::from("index file not found")))
                                        .unwrap()
                                }
                                Ok(index_content) => index_content,
                            };
                            Response::builder()
                                .status(StatusCode::OK)
                                .body(boxed(Body::from(index_content)))
                                .unwrap()
                        }
                        _ => res.map(boxed),
                    }
                }
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
            }
        }))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(state);

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        port,
    ));

    log::info!("listening on http://{}", sock_addr);

    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}
