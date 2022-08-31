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

use image::Luma;
use qrcode::QrCode;

#[tauri::command]
async fn generate(
    dir_path: String,
    number_of_wallet: usize,
    qr_code: bool,
    csv: bool,
) -> Result<String, String> {
    let now = Instant::now();
    let count = Arc::new(Mutex::new(0));
    let str = Arc::new(dir_path);
    let is_qr_enabled = Arc::new(qr_code);
    let is_csv_enabled = Arc::new(csv);

    tokio::fs::create_dir_all(format!("/{}/json", &str))
        .await
        .expect("Failed to crete json dir");

    if *is_qr_enabled {
        tokio::fs::create_dir_all(format!("/{}/qr", &str))
            .await
            .expect("Failed to create qr dir");
    }

    if *is_csv_enabled {
        let mut csv = OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("/{}/record.csv", &str))
            .await
            .expect("Failed to crete csv file");
        csv.write(b"private_key,wallet_address\n")
            .await
            .expect("Failed to initialize csv file");
    }

    for _ in 0..number_of_wallet {
        let my_count = Arc::clone(&count);
        let dir = Arc::clone(&str);
        let qr = Arc::clone(&is_qr_enabled);
        let csv = Arc::clone(&is_csv_enabled);

        tokio::spawn(async move {
            let mut lock = my_count.lock().await;
            *lock += 1;
            let key = Wallet::new(&mut rand::thread_rng());

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(format!("/{}/json/{}.json", &dir, lock))
                .await?;

            let address = key.address();
            let private_key = key.signer().to_bytes().encode_hex::<String>();

            file.write_all(
                json!({
                    "address": address,
                    "private_key": private_key,
                })
                .to_string()
                .as_bytes(),
            )
            .await?;

            if *qr {
                let code = QrCode::new(private_key.clone()).unwrap();
                let image = code.render::<Luma<u8>>().build();
                image
                    .save(format!("/{}/qr/{}.png", &dir, lock))
                    .expect("Failed to save image");
            }

            if *csv {
                let mut record = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .open(format!("/{}/record.csv", &dir))
                    .await?;

                record
                    .write(format!("{},{:?}\n", private_key, address).as_ref())
                    .await?;
            }

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
