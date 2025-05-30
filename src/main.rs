#[allow(unused)]

use actix_web::{
    get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,delete ,put,
    middleware::Logger,
};

use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::Utc;
use mime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use actix_cors::Cors;
use dotenv;




use products::{ProductStore,get_products_with_limit,get_product_by_id, get_products_by_category,get_all_products,create_product , update_product , delete_product};


use crate::email::EmailService;

mod auth;
mod email;
mod products;









#[derive(Debug, Deserialize, Serialize)]
pub  struct ContactForm {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub preferred_method: Option<String>,
    pub message: String,
}




#[post("/contact")]
async fn contact(
    credentials: BearerAuth, form: web::Json<ContactForm>,
    email_service: web::Data<EmailService>,
) -> HttpResponse {
    if !auth::validate_token(credentials.token()) {
        return HttpResponse::Unauthorized().json(json!({
            "error": "Invalid token",
            "message":"Please provide a valid authentication token"
        }));
    }
    match email_service.send_contact_email(
        "agyapongrendosland53@gmail.com",
        &form.name,
        &form.email,
        &form.message,
    ) .await{
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Email sent successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e
        })),
    }
}




#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type(mime::APPLICATION_JSON)
        .json(json!({
            "message": "...",
            "status": "success",
            "timestamp": Utc::now().to_rfc3339()
        }))
}


#[post("/login")]
async fn login(login: web::Json<auth::LoginRequest>) -> HttpResponse {
    match auth::authenticate(&login.into_inner()) {
        Ok(token) => HttpResponse::Ok().json(json!({
            "token": token,
            "token_type": "bearer",
            "expires_in": 86400 // 24 hours in seconds
        })),
        Err(err) => HttpResponse::Unauthorized().json(json!({
            "error": err,
            "valid_users": [{"username": "rendosland", "password": "Migraine69"}]
        })),


    }
}


// #[post("/api/login")]
// async fn login(login_req: web::Json<LoginRequest>) -> impl Responder {
    // In production, verify against your user database
//     if login_req.username == "admin" && login_req.password == "admin123" {
//         let expiration = Utc::now()
//             .checked_add_signed(Duration::hours(24))
//             .expect("Invalid timestamp")
//             .timestamp() as usize;

//         let claims = Claims {
//             sub: login_req.username.clone(),
//             exp: expiration,
//         };

//         let token = encode(
//             &Header::default(),
//             &claims,
//             &EncodingKey::from_secret(SECRET.as_ref()),
//         ).unwrap();

//         HttpResponse::Ok().json(LoginResponse { token })
//     } else {
//         HttpResponse::Unauthorized().json("Invalid credentials")
//     }
// }


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let email_service = email::EmailService::new()
        .map_err(|e: String| {
            eprintln!("Failed to initialize email service: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, "Email service init failed")
        })?;

    let email_service = web::Data::new(email_service);
    let product_store = web::Data::new(ProductStore::new());


    HttpServer::new(move || {


        let cors = Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allow_any_header()
        .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(email_service.clone())
            .app_data(product_store.clone())
            .service(get_products_with_limit)
            .service(get_product_by_id)
            .service(get_products_by_category)
            .service(get_all_products)
            .service(hello)
            .service(login)
            .service(contact)
            .service(create_product)
            .service(update_product)
            .service(delete_product)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
