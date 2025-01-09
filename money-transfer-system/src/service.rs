// services/mod.rs
use crate::db::DbPool;
use crate::repository::{get_account, update_account_balance};
use sqlx::Error;

pub async fn transfer_money(
    pool: &DbPool,
    from_acc_number: &str,
    to_acc_number: &str,
    amount: f32,
) -> Result<(), Error> {
    // Start a transaction
    //let mut transaction: Transaction<'_, Postgres> = pool.begin().await?;

    // Load account details
    let from_account = get_account(pool, from_acc_number)
        .await?
        .ok_or_else(|| sqlx::Error::RowNotFound)?;

    let to_account = get_account(pool, to_acc_number)
        .await?
        .ok_or_else(|| sqlx::Error::RowNotFound)?;

    // Check sufficient balance
    if from_account.balance < amount {
        return Err(Error::ColumnDecode {
            index: "InsufficientBalance".into(),
            source: Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Insufficient balance",
            )),
        });
    }

    // Debit and Credit
    update_account_balance(pool, from_acc_number, from_account.balance - amount).await?;

    update_account_balance(pool, to_acc_number, to_account.balance + amount).await?;

    // dedit transaction
    // credit transaction

    // insert into transaction_history

    // Commit transaction
    // transaction.commit().await?;

    Ok(())
}
