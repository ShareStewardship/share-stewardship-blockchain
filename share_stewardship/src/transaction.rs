use sha2::{Digest, Sha256};

pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    pub signature: String,
    pub tx_id: String,
}

impl Transaction {
    pub fn new(sender: &str, recipient: &str, amount: f64, signature: &str) -> Self {
        let tx_data = format!("{{\"sender\":\"{}\",\"recipient\":\"{}\",\"amount\":{}}}", sender, recipient, amount);
        let mut hasher = Sha256::new();
        hasher.update(tx_data);
        let tx_id = format!("{:x}", hasher.finalize());

        Transaction {
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            amount,
            signature: signature.to_string(),
            tx_id,
        }
    }

    pub fn verify_signature(&self) -> bool {
        !self.signature.is_empty()
    }
}
