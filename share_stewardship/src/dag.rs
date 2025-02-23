use crate::transaction::Transaction;
use std::collections::HashMap;

pub struct DAG {
    pub graph: HashMap<String, Transaction>,
}

impl DAG {
    pub fn new() -> Self {
        DAG {
            graph: HashMap::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.graph.insert(transaction.tx_id.clone(), transaction);
    }

    pub fn get_transaction(&self, tx_id: &str) -> Option<&Transaction> {
        self.graph.get(tx_id)
    }

    pub fn get_recent_transactions(&self, limit: usize) -> Vec<&Transaction> {
        let mut transactions: Vec<&Transaction> = self.graph.values().collect();
        transactions.reverse();
        transactions.into_iter().take(limit).collect()
    }
}
