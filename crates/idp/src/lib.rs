use actix_web::{get, HttpResponse, Responder, web};

use common::Context;
use database::DB;
use database::user::UserDB;

#[derive(Debug)]
enum OAuth2Provider {
    Google,
    Github,
}

#[get("/{provider}/authorize")]
async fn auth(provider: web::Path<String>, context: web::Data<Context<DB>>) -> impl Responder {
    format!("Hello {}!", provider);

    let provider = match provider.as_str() {
        "google" => OAuth2Provider::Google,
        "github" => OAuth2Provider::Github,
        _ => return HttpResponse::NotFound().finish(),
    };

    let result = context.db.find_all_users().await.unwrap();
    println!("{:?}", result);

    return HttpResponse::Ok().body(format!("Hello {:?}!", provider));
}

pub fn oauth2_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(auth);
}
