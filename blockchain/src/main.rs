use crate::runtime::Config;

mod balances;
mod proof_of_existence;
mod runtime;
mod support;
mod system;

mod types {
    use crate::support;

    pub type BlockNumber = u32;
    pub type Identity = String;
    pub type Extrensic = support::Extrinsic<Identity, crate::RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrensic>;
}

fn main() {
    let mut runtime = runtime::Runtime::new();

    let alice = "Max".to_string();
    let bob = "Joe".to_string();
    let charlie = "Bob".to_string();

    runtime.balances.set_balance(&alice, 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: bob.clone(),
                    amount: 60,
                }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: charlie.clone(),
                    amount: 20,
                }),
            },
        ],
    };

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
                    content: "Document".to_string(),
                }),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
                    content: "bob's doc".to_string(),
                }),
            },
        ],
    };

    runtime.execute_block(block_1).expect("Wrong block!");

    runtime.execute_block(block_2).expect("Wrong block!");

    println!("{:#?}", runtime)
}

pub enum RuntimeCall {
    Balances(balances::Call<Config>),
    ProofOfExistence(proof_of_existence::Call<Config>),
}
