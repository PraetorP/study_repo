use actix_web::{
    body::EitherBody,
    web::{self, to},
    HttpResponse, Responder, get, post, put,
};

use deadpool_postgres::Pool;
use eyre::Result;
use crate::{db, models::{Status, CreateToDoList, ResultResponse}};

pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "Ok".to_string(),
    })
}
#[get("todos")]
pub async fn get_todos(db_pool: web::Data<Pool>) -> Result< impl Responder, Box<dyn std::error::Error>> {
    let client = db_pool.get().await?;

    let res = db::get_todos(&client).await?;

    Ok(HttpResponse::Ok().json(res))
    // let client = db_pool.get().await.expect("foo");
    // let res = db::get_todos(&client).await;
    
    // match res {
    //    Ok(todos) => HttpResponse::Ok().json(todos),
    //    Err(_) => HttpResponse::InternalServerError().into() 
    // }
}

#[get("todos/{list_id}/items")]
pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<i32>) -> Result< impl Responder, Box<dyn std::error::Error>> {
    let client = db_pool.get().await?;

    let res = db::get_items(&client, path.into_inner()).await?;

    Ok(HttpResponse::Ok().json(res))
}

#[post("todos")]
pub async fn create_todo(db_pool: web::Data<Pool>, path: web::Json<CreateToDoList>) -> Result< impl Responder, Box<dyn std::error::Error>> {
    let client = db_pool.get().await?;

    let res = db::create_todo(&client, path.into_inner().title).await?;

    Ok(HttpResponse::Ok().json(res))
}


#[put("todos/{list_id}/items/{item_id}")]
pub async fn check_items(db_pool: web::Data<Pool>, path: web::Path<(i32, i32)>) ->  Result< impl Responder, Box<dyn std::error::Error>> {
    let client = db_pool.get().await?;
    let (list_id, item_id) = path.into_inner();
    let res = db::check_todo(&client, list_id, item_id).await?;
    
    Ok(HttpResponse::Ok().json(Into::<ResultResponse>::into(res)))
}
