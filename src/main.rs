use actix_web::{delete, get, patch, post, web, App, HttpResponse, HttpServer, Responder};
// put is used to update or create a new resource
mod database;

use database::{get_database, ActivityCreate, ActivityUpdate, UserCreate};

// type Error = Box<dyn std::error::Error>;
// type Result<T> = std::result::Result<T, Error>;

#[post("/user")]
async fn add_user(req: web::Json<UserCreate>) -> impl Responder {
    let db_locked = get_database();
    let db_unlocked = db_locked.lock().unwrap();
    let user_create = req.into_inner();
    db_unlocked.insert_user(&user_create).unwrap();

    HttpResponse::Ok().body(format!("received {:?}!", user_create))
}

// #[put("/user")]
// async fn update_user(req: web)

#[post("/activity")]
async fn add_activity(req: web::Json<ActivityCreate>) -> impl Responder {
    let db_locked = get_database();
    let db_unlocked = db_locked.lock().unwrap();
    let activity = req.into_inner();
    db_unlocked.insert_activity(&activity).unwrap();

    HttpResponse::Ok().body(format!("received {:?}!", activity))
    // HttpResponse::Ok().body("Received activity!".to_string())
}

#[patch("/activity")]
async fn update_activity(req: web::Json<ActivityUpdate>) -> impl Responder {
    let db_locked = get_database();
    let db_unlocked = db_locked.lock().unwrap();
    let activity_update = req.into_inner();
    let _ = db_unlocked.update_activity(&activity_update);

    HttpResponse::Ok().body(format!("received {:?}!", activity_update))
}

#[get("/activity/{id}")]
async fn get_activity(id: web::Path<u64>) -> impl Responder {
    let db_locked = get_database();
    let db_unlocked = db_locked.lock().unwrap();
    let activity = db_unlocked.get_activity(*id);
    HttpResponse::Ok().body(format!("Activity: {:?}", activity))
}

#[delete("/activity/{id}")]
async fn delete_activity(id: web::Path<u64>) -> impl Responder {
    let db_locked = get_database();
    let db_unlocked = db_locked.lock().unwrap();
    let result = db_unlocked.delete_activity(*id);
    match result {
        Ok(_) => return HttpResponse::Ok().body("Deleted"),
        Err(_) => return HttpResponse::NotModified().body("Not Deleted"),
    }
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
            .service(get_activity)
            .service(update_activity)
            .service(delete_activity)
            .app_data(web::JsonConfig::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
