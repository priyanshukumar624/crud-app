mod db;
mod models;
mod handlers;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use db::connect_db;

// Import from handlers/user_handler.rs
use handlers::user_handlers::{
    register_user,
    login_user,
    get_users,
    update_user,
    delete_user,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = connect_db().await;

    println!("✅ Server running at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
            .route("/users", web::get().to(get_users))
            .route("/users/{id}", web::put().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
