mod auth;
mod config;
mod db;
mod handlers;
mod middleware;
mod models;
mod utils;
mod tui;
mod tui_middleware;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer};
use config::Config;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tui::{create_logger, LogLevel};
use tui_middleware::TuiLogging;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create TUI logger
    let (tui_logger, rx) = create_logger();
    let tui_logger = Arc::new(tui_logger);
    
    // Spawn TUI in separate thread
    std::thread::spawn(move || {
        if let Err(e) = tui::run_tui(rx) {
            eprintln!("TUI error: {}", e);
        }
    });

    // Give TUI time to initialize
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load configuration
    let config = Config::from_env();
    tui_logger.log(
        LogLevel::Info,
        "Configuration loaded successfully".to_string(),
        Some("main".to_string()),
    );

    // Create database connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    tui_logger.log(
        LogLevel::Info,
        "Database connected successfully".to_string(),
        Some("database".to_string()),
    );

    // Run migrations
    db::run_migrations(&db_pool)
        .await
        .expect("Failed to run migrations");

    tui_logger.log(
        LogLevel::Info,
        "Database migrations completed".to_string(),
        Some("database".to_string()),
    );

    // Create shared state
    let state = Arc::new(config);

    // Get server port
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("0.0.0.0:{}", port);

    tui_logger.log(
        LogLevel::Info,
        format!("Starting server on {}", bind_address),
        Some("server".to_string()),
    );

    // Ensure directories exist
    std::fs::create_dir_all("./uploads").ok();
    std::fs::create_dir_all("./logos").ok();
    std::fs::create_dir_all("./backgrounds").ok();
    std::fs::create_dir_all("./avatars").ok();
    
    tui_logger.log(
        LogLevel::Debug,
        "Created required directories".to_string(),
        Some("filesystem".to_string()),
    );

    // Start background cleanup task
    let cleanup_pool = db_pool.clone();
    let cleanup_logger = Arc::clone(&tui_logger);
    tokio::spawn(async move {
        cleanup_logger.log(
            LogLevel::Info,
            "Started background cleanup task".to_string(),
            Some("cleanup".to_string()),
        );
        handlers::admin::cleanup_expired_uploads(cleanup_pool).await;
    });

    // Clone logger for request handling
    let request_logger = Arc::clone(&tui_logger);

    tui_logger.log(
        LogLevel::Info,
        format!("🚀 Server is now running and accepting connections on {}", bind_address),
        Some("server".to_string()),
    );

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                let allowed = std::env::var("ALLOWED_ORIGINS")
                    .unwrap_or_else(|_| "*".to_string());
                if allowed == "*" {
                    return true;
                }
                allowed.split(',').any(|o| o.trim() == origin.to_str().unwrap_or(""))
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::ORIGIN,
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::AUTHORIZATION,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(TuiLogging::new(Arc::clone(&request_logger)))
            .wrap(Logger::default())
            .wrap(cors)
            .wrap(middleware::SecurityHeaders)
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::from(state.clone()))
            .service(
                web::scope("/api")
                    // Auth routes
                    .route("/register", web::post().to(handlers::auth::register))
                    .route("/login", web::post().to(handlers::auth::login))
                    .route("/logout", web::post().to(handlers::auth::logout))
                    .route("/me", web::get().to(handlers::auth::me))
                    .route("/avatar", web::post().to(handlers::auth::upload_avatar))
                    // Upload routes
                    .route("/upload", web::post().to(handlers::upload::upload))
                    .route("/uploads", web::get().to(handlers::upload::get_uploads))
                    .route("/uploads/{id}", web::delete().to(handlers::upload::delete_upload))
                    .route("/uploads/{id}/availability", web::put().to(handlers::upload::toggle_availability))
                    .route("/uploads/{id}/expiration", web::put().to(handlers::upload::update_expiration))
                    // Reverse share routes
                    .route("/reverse-tokens", web::post().to(handlers::reverse::create_token))
                    .route("/reverse-tokens", web::get().to(handlers::reverse::get_tokens))
                    .route("/reverse-tokens/{id}", web::delete().to(handlers::reverse::delete_token))
                    .route("/reverse-upload/{token}", web::post().to(handlers::reverse::reverse_upload))
                    // Download routes
                    .route("/download/{id}", web::get().to(handlers::download::download))
                    .route("/file/{id}/{filename}", web::get().to(handlers::download::download_file))
                    .route("/files/{id}", web::get().to(handlers::download::get_file_metadata))
                    // Admin routes
                    .route("/admin/settings", web::post().to(handlers::admin::update_settings))
                    .route("/admin/stats", web::get().to(handlers::admin::get_stats))
                    .route("/admin/users", web::get().to(handlers::admin::get_users))
                    .route("/admin/users/{id}/block", web::post().to(handlers::admin::block_user))
                    .route("/admin/users/{id}/promote", web::post().to(handlers::admin::promote_user))
                    .route("/admin/quick-settings", web::post().to(handlers::admin::quick_settings))
                    // Settings route (public)
                    .route("/settings", web::get().to(handlers::settings::get_settings))
                    // Legacy admin promotion
                    .route("/promote-first-admin", web::post().to(handlers::admin::promote_first_admin))
            )
            // Serve static files
            .service(fs::Files::new("/logos", "./logos").show_files_listing())
            .service(fs::Files::new("/backgrounds", "./backgrounds").show_files_listing())
            .service(fs::Files::new("/avatars", "./avatars").show_files_listing())
    })
    .bind(&bind_address)?
    .run()
    .await
}
