use sqlx::{PgPool, Postgres, Transaction};

#[derive(Debug)]
struct Transfer {
    from: String,
    to: String,
    amount: i64,
    idempotency_key: String,
}

async fn transfer(
    pool: &PgPool,
    transfer: Transfer,
) -> Result<(), sqlx::Error> {
    let mut tx: Transaction<'_, Postgres> =
        pool.begin().await?;

    // 1. Atomically deduct the money only if
    //    sufficient balance exists.
    let result = sqlx::query!(
        r#"
        UPDATE accounts
        SET balance = balance - $1
        WHERE user_id = $2
          AND balance >= $1
        "#,
        transfer.amount,
        transfer.from
    )
    .execute(&mut *tx)
    .await?;

    // Nothing was updated => insufficient balance
    if result.rows_affected() != 1 {
        return Err(sqlx::Error::RowNotFound);
    }

    // 2. Record the transfer in the same DB transaction.
    sqlx::query!(
        r#"
        INSERT INTO transfers
            (idempotency_key, sender, receiver, amount, status)
        VALUES ($1, $2, $3, $4, 'PENDING')
        "#,
        transfer.idempotency_key,
        transfer.from,
        transfer.to,
        transfer.amount
    )
    .execute(&mut *tx)
    .await?;

    // 3. Commit both operations atomically.
    tx.commit().await?;

    Ok(())
}