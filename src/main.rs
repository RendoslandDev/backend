#[allow(unused)]

use actix_web::{
    get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,delete ,put,
    middleware::Logger,
};
use actix_session::Session;

use chrono::Utc;
use mime;
// use serde::{Deserialize, Serialize};
use serde_json::json;
use actix_cors::Cors;
use dotenv;




use products::{ProductStore,get_products_with_limit,get_product_by_id, get_products_by_category,get_all_products,create_product , update_product , delete_product};


mod auth;
mod products;









// #[derive(Debug, Deserialize, Serialize)]
// // pub  struct ContactForm {
// //     pub name: String,
// //     pub email: String,
// //     pub phone: Option<String>,
// //     pub preferred_method: Option<String>,
// //     pub message: String,
// // }




// #[post("/contact")]
// async fn contact(
//     credentials: BearerAuth, form: web::Json<ContactForm>,
//     email_service: web::Data<EmailService>,
// ) -> HttpResponse {
//     if !auth::validate_token(credentials.token()) {
//         return HttpResponse::Unauthorized().json(json!({
//             "error": "Invalid token",
//             "message":"Please provide a valid authentication token"
//         }));
//     }
//     match email_service.send_contact_email(
//         "agyapongrendosland53@gmail.com",
//         &form.name,
//         &form.email,
//         &form.message,
//     ) .await{
//         Ok(_) => HttpResponse::Ok().json(json!({
//             "status": "success",
//             "message": "Email sent successfully"
//         })),
//         Err(e) => HttpResponse::InternalServerError().json(json!({
//             "error": e
//         })),
//     }
// }




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

#[post("/logout")]
 pub async fn logout(session: Session) -> impl Responder {
    // Clear all session data
    // session.purge();

    // Or if you want to just remove specific values:
    session.remove("user_id");
    session.remove("username");

    HttpResponse::Ok().json(serde_json::json!({
        "message": "Logged out successfully"
    }))

}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));


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
            .app_data(product_store.clone())
            .service(logout)
            .service(get_products_with_limit)
            .service(get_product_by_id)
            .service(get_products_by_category)
            .service(get_all_products)
            .service(hello)
            .service(login)
            .service(create_product)
            .service(update_product)
            .service(delete_product)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
