
use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};

struct Transfer {
    from: String,
    to: String,
    amount: u64,
    reply: oneshot::Sender<Result<(), String>>,
}

async fn state_writer(
    mut rx: mpsc::Receiver<Transfer>,
) {
    let mut balances = HashMap::new();
    balances.insert("Alice".to_string(), 100_u64);

    // Only this task can modify the balances.
    while let Some(transfer) = rx.recv().await {
        let result = (|| {
            if transfer.amount == 0 {
                return Err("Amount must be positive".to_string());
            }

            if transfer.from == transfer.to {
                return Err("Self-transfer not allowed".to_string());
            }

            let sender_balance =
                *balances.get(&transfer.from).unwrap_or(&0);

            if sender_balance < transfer.amount {
                return Err("Insufficient balance".to_string());
            }

            let receiver_balance =
                *balances.get(&transfer.to).unwrap_or(&0);

            let new_receiver = receiver_balance
                .checked_add(transfer.amount)
                .ok_or_else(|| "Balance overflow".to_string())?;

            balances.insert(
                transfer.from.clone(),
                sender_balance - transfer.amount,
            );

            balances.insert(
                transfer.to.clone(),
                new_receiver,
            );

            println!(
                "{} sent {} coins to {}",
                transfer.from, transfer.amount, transfer.to
            );

            Ok(())
        })();

        // Reply to the request that submitted this transfer.
        let _ = transfer.reply.send(result);

        println!("Current balances: {balances:?}");
    }
}

async fn send_transfer(
    tx: &mpsc::Sender<Transfer>,
    from: &str,
    to: &str,
    amount: u64,
) -> Result<(), String> {
    let (reply_tx, reply_rx) = oneshot::channel();

    tx.send(Transfer {
        from: from.to_string(),
        to: to.to_string(),
        amount,
        reply: reply_tx,
    })
    .await
    .map_err(|_| "State writer unavailable".to_string())?;

    reply_rx
        .await
        .map_err(|_| "State writer stopped".to_string())?
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<Transfer>(100);

    // Start the one state-owning task.
    let writer = tokio::spawn(state_writer(rx));

    // Two requests arrive concurrently.
    let tx1 = tx.clone();
    let request_a = tokio::spawn(async move {
        send_transfer(&tx1, "Alice", "Bob", 80).await
    });

    let tx2 = tx.clone();
    let request_b = tokio::spawn(async move {
        send_transfer(&tx2, "Alice", "Charlie", 70).await
    });

    println!("Request A: {:?}", request_a.await.unwrap());
    println!("Request B: {:?}", request_b.await.unwrap());

    // Shut down the writer after all requests finish.
    drop(tx);
    writer.await.unwrap();
}
