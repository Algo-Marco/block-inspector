use ethers::prelude::*;
use std::ops::SubAssign;

#[tokio::main]
async fn main() {
    // Setup application
    let provider = Provider::<Http>::try_from(
        "https://eth-sepolia.g.alchemy.com/v2/...",
    )
    .expect("HTTP Provider should be created successfully.");
    // Define parameters
    let starting_block_nr = provider.get_block_number().await.unwrap();
    println!("Current block number: {}", starting_block_nr);
    let secs_in_year = 365 * 24 * 60 * 60;
    let num_of_years = 1;
    let starting_block_timestamp = provider
        .get_block(starting_block_nr)
        .await
        .unwrap()
        .unwrap()
        .timestamp;
    let final_timestamp = starting_block_timestamp - (num_of_years * secs_in_year);
    let contract_address = "0x419Fe9f14Ff3aA22e46ff1d03a73EdF3b70A62ED"
        .parse::<Address>()
        .unwrap();
    // Define retune value
    let mut matching_txs: Vec<String> = Vec::new();
    // Process blocks iteratively until final timestamp is reached
    let mut still_in_timerange = true;
    let mut next_block = starting_block_nr;
    while still_in_timerange {
        println!("Next block to be processed: {}", next_block);
        let block_with_txs = provider
            .get_block_with_txs(next_block)
            .await
            .unwrap()
            .unwrap();
        println!("Block timestamp: {}", block_with_txs.timestamp);
        if block_with_txs.timestamp < final_timestamp {
            still_in_timerange = false;
            println!("Final timestamp reached.");
        } else {
            println!("Checking mathing TXS in block...");
            let mut matches: Vec<String> = block_with_txs
                .transactions
                .iter()
                .filter(|tx| tx.to == Some(contract_address) || tx.from == contract_address)
                .map(|tx| tx.hash.to_string())
                .collect();
            if !matches.is_empty() {
                matching_txs.append(&mut matches);
            }
        }
        next_block.sub_assign(U64::one());
    }
    // Print results
    println!("Matching transactions: {:?}", matching_txs);
}
