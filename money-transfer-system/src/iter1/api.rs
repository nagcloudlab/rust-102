// api/mod.rs
use crate::db::DbPool;
use crate::model::TransferRequest;
use crate::repository::get_transaction_history;
use crate::service::transfer_money;
use actix_web::{get, post, web, HttpResponse, Responder};
use validator::Validate;

#[get("/transactions")]
async fn transaction_history(pool: web::Data<DbPool>) -> impl Responder {
    match get_transaction_history(pool.get_ref()).await {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch transaction history"),
    }
}

#[post("/transfer")]
async fn transfer(pool: web::Data<DbPool>, req: web::Json<TransferRequest>) -> impl Responder {
    log::info!("Received transfer request: {:?}", req);

    // custom validation

    if let Err(errors) = req.validate() {
        log::warn!("Validation errors: {:?}", errors);
        return HttpResponse::BadRequest().json(errors);
    }

    let result = transfer_money(
        pool.get_ref(),
        &req.from_account,
        &req.to_account,
        req.amount,
    )
    .await;

    match result {
        Ok(_) => {
            log::info!(
                "Transfer from '{}' to '{}' for amount {} completed successfully",
                req.from_account,
                req.to_account,
                req.amount
            );
            HttpResponse::Ok().json(crate::model::TransferResponse {
                success: true,
                message: "Transfer successful".into(),
            })
        }
        Err(e) => {
            log::error!(
                "Transfer from '{}' to '{}' failed: {}",
                req.from_account,
                req.to_account,
                e
            );
            HttpResponse::BadRequest().json(crate::model::TransferResponse {
                success: false,
                message: format!("Transfer failed: {}", e),
            })
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(transfer);
    cfg.service(transaction_history);
}
