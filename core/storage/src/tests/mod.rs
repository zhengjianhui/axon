extern crate test;

macro_rules! exec {
    ($func: expr) => {
        futures::executor::block_on(async { $func.await.unwrap() })
    };
}

mod adapter;
mod storage;

use rand::random;

use protocol::types::Bytes;
use protocol::types::{
    Block, Hash, Hasher, Header, Proof, Receipt, SignatureComponents, SignedTransaction,
    Transaction, TransactionAction, UnverifiedTransaction,
};

const _ADDRESS_STR: &str = "0xCAB8EEA4799C21379C20EF5BAA2CC8AF1BEC475B";

fn mock_signed_tx(tx_hash: Hash) -> SignedTransaction {
    // let nonce = Hasher::digest(Bytes::from("XXXX"));

    SignedTransaction {
        transaction: UnverifiedTransaction {
            unsigned:  Transaction {
                chain_id:                 0,
                nonce:                    Default::default(),
                max_priority_fee_per_gas: Default::default(),
                max_fee_per_gas:          Default::default(),
                gas_limit:                Default::default(),
                action:                   TransactionAction::Create,
                value:                    Default::default(),
                input:                    vec![],
                access_list:              vec![],
                odd_y_parity:             false,
                r:                        Default::default(),
                s:                        Default::default(),
            },
            signature: SignatureComponents {
                standard_v: 0,
                r:          Default::default(),
                s:          Default::default(),
            },
            chain_id:  None,
            hash:      tx_hash,
        },
        sender:      Default::default(),
        public:      None,
    }
}

fn mock_receipt(_tx_hash: Hash) -> Receipt {
    Receipt {
        state_root: Default::default(),
        used_gas:   Default::default(),
        logs_bloom: Default::default(),
        logs:       vec![],
    }
}

fn mock_block(height: u64, _block_hash: Hash) -> Block {
    let _nonce = Hasher::digest(Bytes::from("XXXX"));
    let header = Header {
        parent_hash:       Default::default(),
        uncles_hash:       Default::default(),
        author:            Default::default(),
        state_root:        Default::default(),
        transactions_root: Default::default(),
        receipts_root:     Default::default(),
        log_bloom:         Default::default(),
        difficulty:        Default::default(),
        timestamp:         0,
        number:            height,
        gas_used:          Default::default(),
        gas_limit:         Default::default(),
        extra_data:        Default::default(),
        mixed_hash:        None,
        nonce:             Default::default(),
        base_fee_per_gas:  None,
    };

    Block {
        header,
        transactions: vec![],
        uncles: vec![],
    }
}

fn mock_proof(block_hash: Hash) -> Proof {
    Proof {
        number: 0,
        round: 0,
        block_hash,
        signature: Default::default(),
        bitmap: Default::default(),
    }
}

fn get_random_bytes(len: usize) -> Bytes {
    let vec: Vec<u8> = (0..len).map(|_| random::<u8>()).collect();
    Bytes::from(vec)
}