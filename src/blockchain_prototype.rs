
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Transfer {
    from: String,
    to: String,
    amount: u64,
}

#[derive(Debug, Clone)]
struct Block {
    id: String,
    parent: Option<String>,
    work: u64,
    transactions: Vec<Transfer>,
}

struct Chain {
    blocks: HashMap<String, Block>,
    genesis_id: String,
    canonical_tip: String,
    genesis_balances: HashMap<String, u64>,
}

impl Chain {
    fn new(balances: HashMap<String, u64>) -> Self {
        let genesis_id = "genesis".to_string();

        let genesis = Block {
            id: genesis_id.clone(),
            parent: None,
            work: 0,
            transactions: vec![],
        };

        let mut blocks = HashMap::new();
        blocks.insert(genesis_id.clone(), genesis);

        Self {
            blocks,
            genesis_id: genesis_id.clone(),
            canonical_tip: genesis_id,
            genesis_balances: balances,
        }
    }

    // Calculate the total work from this block to genesis.
    fn accumulated_work(&self, id: &str) -> Result<u64, String> {
        let mut current = id.to_string();
        let mut total = 0u64;

        loop {
            let block = self.blocks.get(&current)
                .ok_or("Unknown block")?;

            total = total.checked_add(block.work)
                .ok_or("Accumulated work overflow")?;

            match &block.parent {
                Some(parent) => current = parent.clone(),
                None => break,
            }
        }

        Ok(total)
    }

    // Build the account state at a particular block.
    fn state_at(
        &self,
        tip: &str,
    ) -> Result<HashMap<String, u64>, String> {
        let mut path = Vec::new();
        let mut current = tip.to_string();

        // Walk backwards to genesis.
        loop {
            let block = self.blocks.get(&current)
                .ok_or("Unknown block")?;

            path.push(block.clone());

            match &block.parent {
                Some(parent) => current = parent.clone(),
                None => break,
            }
        }

        if current != self.genesis_id {
            return Err("Invalid chain ancestry".into());
        }

        // Replay transactions from oldest block to newest.
        let mut balances = self.genesis_balances.clone();

        for block in path.iter().rev() {
            for tx in &block.transactions {
                if tx.amount == 0 || tx.from == tx.to {
                    return Err("Invalid transfer".into());
                }

                let sender_balance =
                    *balances.get(&tx.from).unwrap_or(&0);

                if sender_balance < tx.amount {
                    return Err("Insufficient balance".into());
                }

                let receiver_balance =
                    *balances.get(&tx.to).unwrap_or(&0);

                let new_receiver = receiver_balance
                    .checked_add(tx.amount)
                    .ok_or("Balance overflow")?;

                balances.insert(
                    tx.from.clone(),
                    sender_balance - tx.amount,
                );
                balances.insert(tx.to.clone(), new_receiver);
            }
        }

        Ok(balances)
    }

    // Add a valid block and update the canonical tip if
    // this branch has strictly more accumulated work.
    fn add_block(&mut self, block: Block) -> Result<bool, String> {
        if self.blocks.contains_key(&block.id) {
            return Err("Duplicate block ID".into());
        }

        let parent_id = block.parent.as_ref()
            .ok_or("Non-genesis block needs a parent")?;

        if !self.blocks.contains_key(parent_id) {
            return Err("Unknown parent block".into());
        }

        if block.work == 0 {
            return Err("Block work must be positive".into());
        }

        // Validate the block's transactions against its parent state.
        self.state_at(parent_id)?;

        let mut trial = self.state_at(parent_id)?;

        for tx in &block.transactions {
            if tx.amount == 0 || tx.from == tx.to {
                return Err("Invalid transfer".into());
            }

            let sender = *trial.get(&tx.from).unwrap_or(&0);
            let receiver = *trial.get(&tx.to).unwrap_or(&0);

            if sender < tx.amount {
                return Err("Insufficient balance".into());
            }

            let new_receiver = receiver
                .checked_add(tx.amount)
                .ok_or("Balance overflow")?;

            trial.insert(tx.from.clone(), sender - tx.amount);
            trial.insert(tx.to.clone(), new_receiver);
        }

        let id = block.id.clone();
        self.blocks.insert(id.clone(), block);

        let candidate_work = self.accumulated_work(&id)?;
        let current_work =
            self.accumulated_work(&self.canonical_tip)?;

        if candidate_work > current_work {
            self.canonical_tip = id;
            Ok(true) // Canonical chain changed.
        } else {
            Ok(false) // Keep the existing canonical chain.
        }
    }

    fn balance(&self, account: &str) -> Result<u64, String> {
        let state = self.state_at(&self.canonical_tip)?;
        Ok(*state.get(account).unwrap_or(&0))
    }
}

fn main() {
    let mut balances = HashMap::new();
    balances.insert("Alice".to_string(), 100);

    let mut chain = Chain::new(balances);

    // Branch A: Alice sends Bob 80.
    chain.add_block(Block {
        id: "4A".into(),
        parent: Some("genesis".into()),
        work: 1,
        transactions: vec![Transfer {
            from: "Alice".into(),
            to: "Bob".into(),
            amount: 80,
        }],
    }).unwrap();

    println!("Tip after 4A: {}", chain.canonical_tip);
    println!("Alice: {}", chain.balance("Alice").unwrap());

    // Branch B: Alice sends Charlie 70.
    // Equal work means branch A remains canonical.
    chain.add_block(Block {
        id: "4B".into(),
        parent: Some("genesis".into()),
        work: 1,
        transactions: vec![Transfer {
            from: "Alice".into(),
            to: "Charlie".into(),
            amount: 70,
        }],
    }).unwrap();

    println!("Tip after 4B: {}", chain.canonical_tip);

    // Branch B accumulates more work.
    chain.add_block(Block {
        id: "5B".into(),
        parent: Some("4B".into()),
        work: 2,
        transactions: vec![],
    }).unwrap();

    println!("Tip after 5B: {}", chain.canonical_tip);
    println!("Alice: {}", chain.balance("Alice").unwrap());
    println!("Bob: {}", chain.balance("Bob").unwrap());
    println!("Charlie: {}", chain.balance("Charlie").unwrap());
}
