use base64::{Engine as _, engine::general_purpose};
use hex;
use bip39::Mnemonic;
use std::str::FromStr;
use slip10::{ derive_key_from_path,BIP32Path};


fn read_phrase() ->String{
    println!("input mnemonics:");
    let mut buf : String = String::new();
    _ = std::io::stdin().read_line(&mut buf).expect("input 12 words");
    let input = buf.trim();
    String::from(input)
}

const BIP44_SUI_PATH : &str = "m/44'/784'/0'/0'/0'";
fn derive_sui_ed25519_key(mnemonic: &str, derivation_path: &str) -> slip10::Key {
    // 从助记词生成种子
    let mnemonic = Mnemonic::from_str(mnemonic).unwrap();
    let seed = mnemonic.to_seed(""); // 空密码
    let path = BIP32Path::from_str(derivation_path).expect("bip44 path derive fail");
    // 使用 SLIP-0010 派生 Ed25519 私钥
    let ext_priv_key = derive_key_from_path(&seed,slip10::Curve::Ed25519, &path)
        .expect("Failed to derive key from path");
    return ext_priv_key
}

fn main() {
    // 示例助记词（需替换为生成预期密钥的正确助记词）
    let mnemonic = read_phrase();

    // 派生私钥
    let keypair : slip10::Key = derive_sui_ed25519_key(&mnemonic, BIP44_SUI_PATH);
    println!("Private Key (hex): {}", hex::encode(&keypair.key));

    // 生成 sui.keystore
    let mut private_key_with_flag = vec![0x00]; // Ed25519 标志
    private_key_with_flag.extend_from_slice(&keypair.key);
    let keystore_entry = general_purpose::STANDARD.encode(&private_key_with_flag);
    println!("sui.keystore Entry: {}", keystore_entry);


    let public_key = keypair.public_key().to_vec();

    println!("publickey(hex) {}",hex::encode(&public_key));
    let address_bytes = blake2b(&public_key);
    let sui_address = format!("0x{}", hex::encode(&address_bytes));
    println!("Sui Address: {}", sui_address);

    // 验证预期值
    let expected_keystore = "AE2R0B21Sg4Y6/PaKK/o+P2vAwR2Q7ne1mhG7Ghwkc6x";
    let expected_address = "0xafe36044ef56d22494bfe6231e78dd128f097693f2d974761ee4d649e61f5fa2";
    println!("Matches expected keystore: {}", keystore_entry == expected_keystore);
    println!("Matches expected address: {}", sui_address == expected_address);
}

use blake2::{Blake2b, Digest};
use typenum::U32;
fn blake2b(data: &[u8]) -> Vec<u8> {
    let mut hasher = Blake2b::<U32>::new();
    hasher.update(data);
    hasher.finalize()[..32].to_vec()
}