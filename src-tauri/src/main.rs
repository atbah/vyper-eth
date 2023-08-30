// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
use serde_json::{Value, to_writer_pretty};
use std::{fs::File, io::BufReader, path::Path};
use vyper_rs::vyper::Vyper;

#[derive(Serialize, Deserialize)]
struct ContractWalletData {
    abi: Value,
    initcode: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    provider: String, 
    keystore: String,
}

impl ContractWalletData {
    fn new( abi: Value, initcode: String) -> ContractWalletData {
        Self {
            abi,
            initcode,
        }
    }
}

#[tauri::command]
async fn fetch_data(path: String) -> Result<ContractWalletData, String> {
    let cpath: &Path = Path::new(path.leak());
    let abi: &Path = Path::new("abi.json");
    let mut contract = Vyper::new(cpath, abi);
    contract.compile().map_err(|e| return e.to_string())?;
    contract.abi().map_err(|e| return e.to_string())?;
    
    let abifile = File::open(&abi).map_err(|e| e.to_string())?;
    let reader = BufReader::new(abifile);
    let abifile_json: Value = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    //println!("{:?}", contract.bytecode.clone().unwrap());
    println!("Back to TS!");
    Ok(ContractWalletData::new(
        abifile_json,
        contract.bytecode.unwrap(),
    ))
}

#[tauri::command]
async fn get_keys(key_path: String) -> Result<Value, String> {
    let keyfile = File::open(Path::new(&key_path)).map_err(|e| e.to_string())?;
    let reader = BufReader::new(keyfile);
    let keystore_json: Value = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(keystore_json)
}

#[tauri::command]
async fn set_config(provider: String, keystore: String) -> Result<(), String> {
    let config_path: &Path = Path::new("./vyper_deployer_config.json"); 
    let conf: Config = Config{provider, keystore};
    let file: File = File::create(config_path).map_err(|e| e.to_string())?;
    to_writer_pretty(file, &conf).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn get_config() -> Result<Config, String> {
    let file: File = File::open("./vyper_deployer_config.json").map_err(|e| e.to_string())?;
    let reader: BufReader<File> = BufReader::new(file);
    let conf: Config = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(conf)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_data, set_config, get_config, get_keys])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
