extern crate util;
extern crate crypto;
extern crate chain;
#[macro_use]
extern crate log;
extern crate network;
extern crate bincode;
extern crate parking_lot;
extern crate tx_pool;

use std::sync::mpsc::Sender;
use chain::chain::Chain;
use chain::block::Block;
use std::thread;
use std::time::Duration;
use util::hash::H256;
use util::Hashable;
use std::sync::Arc;
use network::connection::Operation;
use util::config::SleepyConfig;
use bincode::{serialize, Infinite};
use parking_lot::RwLock;
use network::msgclass::MsgClass;
use tx_pool::Pool;

pub fn start_miner(tx: Sender<(u32, Operation, Vec<u8>)>,
                   chain: Arc<Chain>,
                   config: Arc<RwLock<SleepyConfig>>,
                   tx_pool: Arc<RwLock<Pool>>) {
    
    let tx = tx.clone();
    let chain = chain.clone();
    let config = config.clone();
    let tx_pool = tx_pool.clone();
    thread::spawn(move || {
        info!("start mining!");
        let mut time = match {config.read().ntp_now()} {
            Some(t) => t,
            _ => panic!("NTP Error"),
        };
        loop {
            if let Some(new_time) = {config.read().ntp_now()} {
                if time < new_time {
                    time = new_time;
                    let (height, hash) = chain.get_status();
                    let miner_privkey = {config.read().get_miner_private_key()};
                    let anc_hash = chain.anc_hash(height, hash).unwrap();
                    
                    let sig = Block::gen_proof(miner_privkey, time, height + 1, anc_hash);
                    let proof = sig.sha3();
                    let difficulty: H256 = {config.read().get_difficulty().into()};

                    if proof < difficulty {               
                        let id = {config.read().get_id()};
                        let (tx_list, hash_list) = { tx_pool.write().package() };
                        let signed_blk = chain.gen_block(height, hash, time, sig, tx_list);
                        { tx_pool.write().update(&hash_list) };
                        info!("generate block at timestamp {}", time);
                        let msg = MsgClass::BLOCK(signed_blk);
                        let message = serialize(&msg, Infinite).unwrap();
                        tx.send((id, Operation::BROADCAST, message)).unwrap();             
                    }
                }
            } else {
                info!("ntp error!!");
            }
            
            thread::sleep(Duration::from_millis(100 / {config.read().nps}));
        }
    });
}
