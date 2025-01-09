// api/mod.rs
use crate::db::DbPool;
use crate::models::TransferRequest;
use crate::service::transfer_money;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/transfer")]
async fn transfer(pool: web::Data<DbPool>, req: web::Json<TransferRequest>) -> impl Responder {
    let result = transfer_money(
        pool.get_ref(),
        &req.from_account,
        &req.to_account,
        req.amount,
    )
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(crate::models::TransferResponse {
            success: true,
            message: "Transfer successful".into(),
        }),
        Err(e) => HttpResponse::BadRequest().json(crate::models::TransferResponse {
            success: false,
            message: format!("Transfer failed: {}", e),
        }),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(transfer);
}
