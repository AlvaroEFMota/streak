use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

mod database;

use database::{get_database, Activity, User};

// type Error = Box<dyn std::error::Error>;
// type Result<T> = std::result::Result<T, Error>;

#[post("/user")]
async fn add_user(req: web::Json<User>) -> impl Responder {
    let db_locked = get_database();
    let db_unlocked = db_locked.lock().unwrap();
    let user = req.into_inner();
    db_unlocked.insert_user(&user).unwrap();

    HttpResponse::Ok().body(format!("received {:?}!", user))
}

#[post("/activity")]
async fn add_activity(req: web::Json<Activity>) -> impl Responder {
    let db_locked = get_database();
    let db_unlocked = db_locked.lock().unwrap();
    let activity = req.into_inner();
    db_unlocked.insert_activity(&activity).unwrap();

    HttpResponse::Ok().body(format!("received {:?}!", activity))
    // HttpResponse::Ok().body("Received activity!".to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    {
        let db_locked = get_database();
        let db_unlocked = db_locked.lock().unwrap();
        db_unlocked.create_database();
    }

    std::env::set_var("RUST_LOG", "debug");
    // std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    log::info!("Starting web server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(add_user)
            .service(add_activity)
            .app_data(web::JsonConfig::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
