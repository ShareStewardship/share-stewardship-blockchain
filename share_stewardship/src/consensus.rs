use crate::dag::DAG;
use crate::validator::Validator;
use crate::transaction::Transaction;

pub struct Consensus<'a> {
    pub dag: &'a mut DAG,
    pub validators: Vec<Validator>,
}

impl<'a> Consensus<'a> {
    pub fn new(dag: &'a mut DAG, validators: Vec<Validator>) -> Self {
        Consensus { dag, validators }
    }

    pub fn reach_consensus(&mut self, transaction: Transaction) -> bool {
        let mut approvals = 0;
        let required_approvals = (self.validators.len() * 2) / 3;

        for validator in &self.validators {
            if validator.validate_transaction(&transaction) {
                approvals += 1;
            }
            if approvals >= required_approvals {
                self.dag.add_transaction(transaction);
                return true;
            }
        }
        false
    }
}
