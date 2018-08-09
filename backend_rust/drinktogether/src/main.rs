extern crate actix_web;
use actix_web::{server, App, HttpRequest, HttpResponse, http, Json};
#[macro_use] extern crate serde_derive;

#[derive(Debug, Deserialize, Serialize)]
struct UserLogin {
    username: String,
    password: String
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    username: String,
    password: String,
    desc: String
}

fn index(req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .body("sikeres get keres az index fuggvenyre hihi")
}

fn handle_login(data: Json<UserLogin>) -> Json<User>{
    let user = User{
        username: data.username.to_string(),
        password: data.password.to_string(),
        desc: "This is the description tho".to_string()
    };
    Json(user)
}

fn main() {
    server::new(|| App::new()
        .resource("/", |r| r.f(index))
        .resource("/login", |r| r.method(http::Method::POST).with(handle_login)))
        .bind("127.0.0.1:3001")
        .unwrap()
        .run();
}