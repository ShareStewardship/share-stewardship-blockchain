mod consensus;
mod dag;
mod transaction;
mod validator;

use consensus::Consensus;
use dag::DAG;
use transaction::Transaction;
use validator::Validator;

fn main() {
    let mut stewardship_dag = DAG::new();

    let validators: Vec<Validator> = (0..10)
        .map(|i| Validator::new(format!("validator_{}", i)))
        .collect();

    let mut consensus_system = Consensus::new(&mut stewardship_dag, validators);

    let sample_tx = Transaction::new("Alice", "Bob", 100.0, "valid_signature");

    if consensus_system.reach_consensus(sample_tx) {
        println!("Transaction successfully added to DAG.");
    } else {
        println!("Transaction failed consensus.");
    }
}
