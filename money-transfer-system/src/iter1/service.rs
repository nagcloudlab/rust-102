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
    log::info!(
        "Starting transfer from '{}' to '{}' for amount {}",
        from_acc_number,
        to_acc_number,
        amount
    );

    // Start a transaction
    let mut transaction: Transaction<'_, Postgres> = pool.begin().await?;

    // step-1
    // Load account details
    let from_account = get_account(&mut transaction, from_acc_number)
        .await?
        .ok_or_else(|| {
            log::error!("Account '{}' not found", from_acc_number);
            sqlx::Error::RowNotFound
        })?;

    // step-2
    // Load account details
    let to_account = get_account(&mut transaction, to_acc_number)
        .await?
        .ok_or_else(|| {
            log::error!("Account '{}' not found", to_acc_number);
            sqlx::Error::RowNotFound
        })?;

    // Check sufficient balance
    if from_account.balance < amount {
        log::error!(
            "Insufficient balance in account '{}'. Available: {}, Required: {}",
            from_acc_number,
            from_account.balance,
            amount
        );
        return Err(Error::ColumnDecode {
            index: "InsufficientBalance".into(),
            source: Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Insufficient balance",
            )),
        });
    }

    // step-3
    // Debit and Credit
    update_account_balance(
        &mut transaction,
        from_acc_number,
        from_account.balance - amount,
    )
    .await?;

    log::info!(
        "Debited '{}' by {}. New balance: {}",
        from_acc_number,
        amount,
        from_account.balance - amount
    );

    // step-4
    update_account_balance(&mut transaction, to_acc_number, to_account.balance + amount).await?;

    log::info!(
        "Credited '{}' by {}. New balance: {}",
        to_acc_number,
        amount,
        to_account.balance + amount
    );

    // step-5
    // Record the transaction
    record_transaction(&mut transaction, from_acc_number, to_acc_number, amount).await?;
    log::info!(
        "Recorded transaction from '{}' to '{}'",
        from_acc_number,
        to_acc_number
    );

    // Commit transaction
    transaction.commit().await?;
    log::info!("Transfer completed successfully");

    Ok(())
}
