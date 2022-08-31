#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ethers::{
    prelude::{rand, Signer, Wallet},
    utils::hex::ToHex,
};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

use serde_json::json;
use tokio::fs::OpenOptions;
use tokio::io;
use tokio::io::AsyncWriteExt;

#[tauri::command]
async fn generate(dir_path: String, number_of_wallet: usize) -> Result<String, String> {
    let now = Instant::now();
    let count = Arc::new(Mutex::new(0));
    let str = Arc::new(dir_path);

    for _ in 0..number_of_wallet {
        let my_count = Arc::clone(&count);
        let dir = Arc::clone(&str);

        tokio::spawn(async move {
            let mut lock = my_count.lock().await;
            *lock += 1;
            let key = Wallet::new(&mut rand::thread_rng());

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(format!("/{}/{:?}.json", &dir, lock))
                .await?;

            file.write_all(
                json!({
                    "address": key.address(),
                    "private_key": key.signer().to_bytes().encode_hex::<String>(),
                })
                .to_string()
                .as_bytes(),
            )
            .await?;
            Ok::<_, io::Error>(())
        });
    }

    loop {
        if *count.lock().await >= number_of_wallet {
            break;
        }
    }

    let elapsed = now.elapsed();
    Ok(format!("Generated! It took {:.2?}.", elapsed).into())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, generate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
