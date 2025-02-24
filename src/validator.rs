use crate::transaction::Transaction;

pub struct Validator {
    pub node_id: String,
    pub reputation: f64,
}

impl Validator {
    pub fn new(node_id: String) -> Self {
        Validator {
            node_id,
            reputation: 1.0,
        }
    }

    pub fn validate_transaction(&self, transaction: &Transaction) -> bool {
        transaction.verify_signature()
    }
}
