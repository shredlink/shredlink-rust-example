use std::collections::HashMap;

use anyhow::Result;
use futures::channel::mpsc::unbounded;
use shredlink_proto::shredlink::{SubscribeTransactionsRequest, SubscribeRequestFilterTransactions, SubscribeEntriesRequest, Transaction};
use shredlink_proto::shredlink::shredlink_service_client::ShredlinkServiceClient;
// use tonic::{Response, Streaming};


#[tokio::main]
async fn main() -> Result<()> {
    
    // Replace with your actual Shredlink URL
    let shredlink_url = "http://localhost:50051".to_string(); // Ask url in ticket https://discord.gg/sskBrcfX
    let mut client = ShredlinkServiceClient::connect(shredlink_url).await.unwrap();

    // TRANSACTIONS SUBSCRIBE EXAMPLE
    let request = pumpfun_new_mint_request();
    
    let (subscribe_tx, subscribe_rx) = unbounded();
    let response =
        client.subscribe_transactions(subscribe_rx).await.unwrap();

    let mut stream = response.into_inner();

    let _ = subscribe_tx.unbounded_send(request);

    while let Some(message) = stream.message().await.unwrap() {
        if let Some(tx_update) = message.transaction {
            if let Some(transaction) = tx_update.transaction {
                let tx_hash = bs58::encode(&transaction.signatures[0]).into_string();
                let mint_address = extract_mint_address(&transaction);
                
                println!("🪙 NEW MINT: {} | TX HASH: {}", 
                    mint_address.unwrap_or_else(|| "Unknown".to_string()),
                    tx_hash,
                );
            }
        }
    }

    // ENTRIES SUBSCRIBE EXAMPLE
    // let mut entries_stream = client
    //     .subscribe_entries(SubscribeEntriesRequest {})
    //     .await
    //     .unwrap()
    //     .into_inner();

    // while let Some(slot_entry) = entries_stream.message().await.unwrap() {
    //     let entries =
    //         match bincode::deserialize::<Vec<solana_entry::entry::Entry>>(&slot_entry.entries) {
    //             Ok(e) => e,
    //             Err(e) => {
    //                 println!("Deserialization failed with err: {e}");
    //                 continue;
    //             }
    //         };

    //     println!(
    //         "slot {}, entries: {}, transactions: {}",
    //         slot_entry.slot,
    //         entries.len(),
    //         entries.iter().map(|e| e.transactions.len()).sum::<usize>()
    //     );
    // }

    Ok(())

}

fn pumpfun_new_mint_request() -> SubscribeTransactionsRequest {
    const TARGET_MINT_AUTHORITY: &str = "TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM";

    let mut transactions = HashMap::new();
    transactions.insert(
        "pumpfun".to_string(),
        SubscribeRequestFilterTransactions {
            account_include: vec![],
            account_exclude: vec![],
            // PumpFun program ID
            account_required: vec![TARGET_MINT_AUTHORITY.to_string()],
        }
    );

    SubscribeTransactionsRequest { 
        transactions,
        request_type: None,
    }
}

fn extract_mint_address(transaction: &Transaction) -> Option<String> {
    if let Some(message) = &transaction.message {
        if let Some(account_key) = message.account_keys.get(1) {
            return Some(bs58::encode(account_key).into_string());
        }
    }
    None
}