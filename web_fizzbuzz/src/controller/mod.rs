use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
mod md_fizzbuzz;
use md_fizzbuzz::calc_fizzbuzz;

#[derive(Debug, Serialize, Deserialize)]
struct Fugahoge {
    id: Option<u32>,
    content: String,
    done: bool,
}

//[get]入力されたfizzbuzzを評価する
#[get("/fizzbuzz/{_figure}")]
async fn get_input_fizzbuzz(_figure: web::Path<i32>) -> impl Responder {
    println!("get_input_fizzbuzz");
    let outputfizzbuzz = calc_fizzbuzz(*_figure);

    HttpResponse::Ok().body(format!("{} is {} !!!",*_figure , outputfizzbuzz))
}

//[get]1~100のfizzbuzzを評価する
#[get("/fizzbuzz")]
async fn get_fizzbuzz() -> impl Responder {
    println!("get_random__fizzbuzz");


    let mut outputfizzbuzz = "Start!! ".to_string();
    
    for chk_num in 1..101 {
        outputfizzbuzz += "
        ";
        outputfizzbuzz += &calc_fizzbuzz(chk_num);
    } 

    HttpResponse::Ok().body(format!("random {}",outputfizzbuzz))
}

//[post]fugahogeをpostする
#[post("/fugahoge")]
async fn post_fugahoge(fizzbuzz: web::Json<Fugahoge>) -> impl Responder {
    println!("post_fugahoge");
    println!("{:?}", fizzbuzz);
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
pub async fn get_main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_fizzbuzz).service(get_input_fizzbuzz))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[actix_web::main]
pub async fn post_main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(post_fugahoge))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}