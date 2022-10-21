#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use config::Config;
use ecvrf_rs::SecretKey;

use crate::{beacon::Beacon, cosmos::wallet::Wallet};
mod beacon;
mod config;
mod cosmos;
mod types;

#[tokio::main]
async fn main() {
    let config = Config::load(&"config.json").unwrap_or_else(|_| {
        println!("No config file found, creating a new one");
        Config::default().save(&"config.json").unwrap();
        Config::default()
    });
    
    let seed = [75u8, 12u8, 31u8, 14u8, 15u8, 16u8, 17u8, 18u8, 19u8, 20u8, 21u8, 22u8, 23u8, 24u8, 25u8, 26u8, 27u8, 28u8, 29u8, 30u8, 31u8, 32u8, 33u8, 34u8, 35u8, 36u8, 37u8, 38u8, 39u8, 40u8, 41u8, 42u8];
    let sk = SecretKey::new(&seed);
    let (pk,_) = sk.extract_public_key_and_scalar().unwrap();

    println!("Public key: {:x?}", pk.to_string());
    println!("Private key: {:x?}", sk.to_string());

    if config.registered_keys.is_empty() {
        println!("No keys registered, please create and whitelist keys using entropycli, or add existing whitelisted keys to the config file");
        std::process::exit(1);
    }

    let network_name = config.default_network.unwrap_or_else(||
        std::env::var("NETWORK").unwrap_or_else(|_|{
            println!("No default network set, please set the default network in the config file, or set the NETWORK environment variable");
            std::process::exit(1);
        })
    );

    let network_info = config.networks.get(&network_name).unwrap_or_else(|| {
        println!(
            "No network configuration found with the name {}, please add the network to the config file manually or with entropycli",
            network_name
        );
        std::process::exit(1);
    });

    let beacon_address = network_info.network.deployed_beacon_address.clone().unwrap_or_else(|| {
        println!("No beacon address found for network {}, please add the beacon address to the config file manually or with entropycli", network_name);
        std::process::exit(1);
    });

    let mnemonic = network_info.signer_mnemonic.clone().unwrap_or_else(||
        std::env::var("MNEMONIC").unwrap_or_else(|_|{
            println!("No mnemonic found, please set the mnemonic in the config file, or set the MNEMONIC environment variable");
            std::process::exit(1);
        })
    );

    let beacon = Beacon::new(
        network_info.network.clone(),
        Wallet::new(mnemonic, network_info.network.clone()).unwrap_or_else(|_| {
            println!("Failed to create wallet from mnemonic, please check the mnemonic in the config file");
            std::process::exit(1);
        }),
        beacon_address,
    );

    println!("Starting entropy worker");
    println!(
        "{}",
        serde_json::to_string_pretty(&beacon.fetch_active_requests().await.unwrap()).unwrap()
    );
}
