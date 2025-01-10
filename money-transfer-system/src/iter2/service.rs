// services/mod.rs
use crate::db::DbPool;
use crate::repository::{get_account, record_transaction, update_account_balance};
use sqlx::{Error, Postgres, Transaction};

pub async fn transfer_money(
    pool: &DbPool,
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
        .ok_or_else(|| {
            tracing::error!("Account '{}' not found", from_acc_number);
            sqlx::Error::RowNotFound
        })?;

    let to_account = get_account(&mut transaction, to_acc_number)
        .await?
        .ok_or_else(|| {
            tracing::error!("Account '{}' not found", to_acc_number);
            sqlx::Error::RowNotFound
        })?;

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
    tracing::info!(
        "Debited '{}' by {}. New balance: {}",
        from_acc_number,
        amount,
        from_account.balance - amount
    );

    update_account_balance(&mut transaction, to_acc_number, to_account.balance + amount).await?;
    tracing::info!(
        "Credited '{}' by {}. New balance: {}",
        to_acc_number,
        amount,
        to_account.balance + amount
    );

    record_transaction(&mut transaction, from_acc_number, to_acc_number, amount).await?;
    tracing::info!(
        "Recorded transaction from '{}' to '{}'",
        from_acc_number,
        to_acc_number
    );

    transaction.commit().await?;
    tracing::info!("Transfer completed successfully");

    Ok(())
}
