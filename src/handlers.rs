use crate::db;
use crate::errors::AppError;
use crate::models::{CreateTodoList, ResultResponse, Status};
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Ok".to_string(),
    })
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(AppError::db_error)?;
    let result = db::get_todos(&client).await;
    result.map(|todos| HttpResponse::Ok().json(todos))
}

pub async fn get_items(
    db_pool: web::Data<Pool>,
    web::Path(list_id): web::Path<(i32,)>,
) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(AppError::db_error)?;
    let result = db::get_items(&client, list_id.0).await;

    result.map(|items| HttpResponse::Ok().json(items))
}

pub async fn create_todo(
    db_pool: web::Data<Pool>,
    json: web::Json<CreateTodoList>,
) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(AppError::db_error)?;
    let result = db::create_todo(&client, json.title.clone()).await;
    result.map(|todo| HttpResponse::Ok().json(todo))
}

pub async fn check_item(
    db_pool: web::Data<Pool>,
    web::Path((list_id, item_id)): web::Path<(i32, i32)>,
) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(AppError::db_error)?;
    let result = db::check_item(&client, list_id, item_id).await;
    result.map(|updated| HttpResponse::Ok().json(ResultResponse { success: updated }))
}
