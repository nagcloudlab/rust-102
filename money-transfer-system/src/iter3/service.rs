// services/mod.rs
use crate::db::DbPool;
use crate::repository::{get_account, record_transaction, update_account_balance};
use serde_json::json;
use sqlx::Error;

pub async fn transfer_money(
    pool: &DbPool,
    kafka_producer: &crate::kafka::KafkaProducer,
    from_acc_number: &str,
    to_acc_number: &str,
    amount: f32,
) -> Result<(), Error> {
    tracing::info!(
        "Starting transfer from '{}' to '{}' for amount {}",
        from_acc_number,
        to_acc_number,
        amount
    );

    let mut transaction = pool.begin().await?;

    let from_account = get_account(&mut transaction, from_acc_number)
        .await?
        .ok_or_else(|| sqlx::Error::RowNotFound)?;

    let to_account = get_account(&mut transaction, to_acc_number)
        .await?
        .ok_or_else(|| sqlx::Error::RowNotFound)?;

    if from_account.balance < amount {
        tracing::error!(
            "Insufficient balance in account '{}'. Available: {}, Required: {}",
            from_acc_number,
            from_account.balance,
            amount
        );
        return Err(sqlx::Error::RowNotFound);
    }

    update_account_balance(
        &mut transaction,
        from_acc_number,
        from_account.balance - amount,
    )
    .await?;
    update_account_balance(&mut transaction, to_acc_number, to_account.balance + amount).await?;
    record_transaction(&mut transaction, from_acc_number, to_acc_number, amount).await?;

    transaction.commit().await?;
    tracing::info!("Transfer completed successfully");

    // Publish message to Kafka
    let message = json!({
        "from_account": from_acc_number,
        "to_account": to_acc_number,
        "amount": amount,
        "status": "completed"
    });

    kafka_producer
        .send_message(from_acc_number, &message.to_string())
        .await;

    Ok(())
}
