// repository/mod.rs
use sqlx::{Error, Executor};

use crate::model::{Account, TransactionHistory};

pub async fn get_account<'a, E>(executor: E, account_number: &str) -> Result<Option<Account>, Error>
where
    E: Executor<'a, Database = sqlx::Postgres>,
{
    tracing::debug!("Fetching account '{}'", account_number);
    let account = sqlx::query_as!(
        Account,
        "SELECT number, balance FROM accounts WHERE number = $1",
        account_number
    )
    .fetch_optional(executor)
    .await?;

    if account.is_some() {
        tracing::debug!("Account '{}' fetched successfully", account_number);
    } else {
        tracing::warn!("Account '{}' not found", account_number);
    }

    Ok(account)
}

// Generic function to update an account balance
pub async fn update_account_balance<'a, E>(
    executor: E,
    account_number: &str,
    new_balance: f32,
) -> Result<(), Error>
where
    E: Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query!(
        "UPDATE accounts SET balance = $1 WHERE number = $2",
        new_balance,
        account_number
    )
    .execute(executor)
    .await?;

    Ok(())
}

// Function to record a transaction in the history table
pub async fn record_transaction<'a, E>(
    executor: E,
    from_account: &str,
    to_account: &str,
    amount: f32,
) -> Result<(), Error>
where
    E: Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query!(
        "INSERT INTO transaction_history (from_account, to_account, amount) VALUES ($1, $2, $3)",
        from_account,
        to_account,
        amount
    )
    .execute(executor)
    .await?;

    Ok(())
}

// Function to fetch transaction history
pub async fn get_transaction_history<'a, E>(executor: E) -> Result<Vec<TransactionHistory>, Error>
where
    E: Executor<'a, Database = sqlx::Postgres>,
{
    let history =
        sqlx::query!("SELECT from_account, to_account, amount, timestamp FROM transaction_history")
            .fetch_all(executor)
            .await?;

    Ok(history
        .into_iter()
        .map(|row| TransactionHistory {
            from_account: row.from_account,
            to_account: row.to_account,
            amount: row.amount,
            timestamp: row.timestamp,
        })
        .collect())
}
