// repository/mod.rs
use crate::db::DbPool;
use sqlx::Error;

#[derive(Debug)]
pub struct Account {
    pub number: String,
    pub balance: f32,
}

pub async fn get_account(pool: &DbPool, account_number: &str) -> Result<Option<Account>, Error> {
    let account = sqlx::query_as!(
        Account,
        "SELECT number, balance FROM accounts WHERE number = $1",
        account_number
    )
    .fetch_optional(pool)
    .await?;

    Ok(account)
}

pub async fn update_account_balance(
    pool: &DbPool,
    account_number: &str,
    new_balance: f32,
) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE accounts SET balance = $1 WHERE number = $2",
        new_balance,
        account_number
    )
    .execute(pool)
    .await?;

    Ok(())
}
