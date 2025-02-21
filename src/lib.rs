use bip39::{Mnemonic};
use bip32::{ExtendedPrivateKey,XPrv};
use solana_sdk::signer::{keypair::Keypair, SeedDerivable, Signer,
                        keypair::keypair_from_seed_and_derivation_path,
                        keypair::keypair_from_seed,
                        };

use serde_json::json;
use std::fs::File;
use std::io::Write;
use serde_json::Value;
use dirs::config_dir;

//ok  like solana-keygen recover
pub fn process_indexes2(menmonic :&Mnemonic){
    let bytes = menmonic.to_seed("");
    let seed = bytes.as_ref();
    let keypair = keypair_from_seed(seed).expect("fail to import from mnemonic");
    println!("process_indexes2:public key {}",bs58::encode(keypair.pubkey().to_bytes()).into_string());
}




//ok  相当于 solana-keygen recover -o t.json
pub fn process_indexes_0(input_phrase :&str){
    let kp = Keypair::from_seed_phrase_and_passphrase(&input_phrase,"")
    .expect("frome seed pass failed");

    println!("process_indexes_0:{}",bs58::encode(kp.pubkey().to_bytes()).into_string());
    let pk_json = json!(kp.to_bytes().to_vec()).to_string();
    println!("json {}",&pk_json);

}


