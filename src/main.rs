use bip39::Mnemonic;
use solana_sdk::signer::{keypair::Keypair,  Signer,
                        keypair::keypair_from_seed_and_derivation_path,
                        };
use solana_derivation_path::DerivationPath;
//use solana_keypair::{keypair_from_seed};
use std::str::FromStr;
use serde_json;
use std::fs::File;
use std::io::Write;
use serde_json::Value;
use dirs::config_dir;
//use ed25519_dalek::{SigningKey,SecretKey};

const BIP44_PATH : &str = "m/44'/501'/0'/0'";
/**
 * 这个助记词导入类似 solana-keygen recover -o t.json  生成的公钥和私钥
 */
fn read_phase() ->String{
    println!("input mnemonics:");
    let mut buf : String = String::new();
    _ = std::io::stdin().read_line(&mut buf).expect("input 12 words");
    let input = buf.trim();
    String::from(input)
}

pub fn generate_keypair(menmonic :&Mnemonic) -> Keypair{
    let bytes = menmonic.to_seed("");
    let seed = bytes.as_ref();
    let derivation_path = DerivationPath::from_absolute_path_str(BIP44_PATH).expect("from _str fail");
    let path:Option<DerivationPath> = Some(derivation_path);
    let keypair = keypair_from_seed_and_derivation_path(seed, path).expect("keypair_from_seed_and_derivation_path failed");
    keypair
    
}

pub fn main(){
    let phrase = read_phase();
    let mnemonic = bip39::Mnemonic::from_str(&phrase).expect("invalid seed phrase");
    let keypair = generate_keypair(&mnemonic);

    println!("pk bs58:{}", bs58::encode(keypair.secret().as_bytes()).into_string());
    _ = write_keypair(keypair);
}

fn write_keypair(keypair:Keypair) -> Result<(), Box<dyn std::error::Error>> {

    // 获取 Keypair 的私钥字节数组
    let private_key_bytes = keypair.to_bytes();
    let addr = bs58::encode(keypair.pubkey().to_bytes()).into_string();
    println!("public key {}",addr);

    // 将字节数组转换为 JSON 数组
    let json_array: Vec<u8> = private_key_bytes.to_vec();
    let json_value: Value = serde_json::to_value(json_array)?;

    // 将 JSON 数据转换为格式化的字符串
    let json_string = serde_json::to_string_pretty(&json_value)?;
    let file_name = format!("{}.json",&addr);
    let file_path = get_file_path(&file_name).unwrap();
    // 打开或创建 id.json 文件
    let mut file: File = File::create(&file_path)?;

    // 将 JSON 字符串写入文件
    file.write_all(json_string.as_bytes())?;
    println!("Keypair private key has been saved to {}",&file_path);
    println!("switch to the uer: \n solana config set --keypair {}",&file_path);

    Ok(())
}

fn get_file_path(file :&str) ->Result<String,std::io::Error> {
    
    // 获取用户的配置目录
    if let Some(config_path) = config_dir() {
        // 构建 Solana 配置目录路径
        let solana_config_path = config_path.join("solana");

        // 确保 Solana 配置目录存在
        if std::fs::create_dir_all(&solana_config_path).is_ok() {
            // 构建要写入的文件路径
            let file_path = solana_config_path.join(file);
            let s  = file_path.to_str();
            return Ok(String::from_str( s.unwrap()).unwrap());
        } else {
            eprintln!("Failed to create the Solana configuration directory");
        }
    } else {
        eprintln!("Could not find the user's configuration directory");
    }
    panic!("fail to write file")
    //std::io::Error::new(std::io::ErrorKind::InvalidData,&"fail to write file")
}