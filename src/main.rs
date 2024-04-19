use my_blockchain_project::contracts::my_contract::my_contract::MyContract;
use my_blockchain_project::dapps::storage_dapp::store_file;

#[tokio::main]
async fn main() {
    // Initialize and run your blockchain node or dApp
    let _result = run_node().await;
}

async fn run_node() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize your blockchain node

    // Deploy the smart contract
    let mut contract = MyContract::new(42);
    println!("Initialized MyContract with value: {}", contract.get());

    // Interact with the smart contract
    contract.increment(10);
    println!("Incremented MyContract value to: {}", contract.get());

    // Store a file using the decentralized storage dApp
    let file_name = "example.txt".to_string();
    let file_content = b"Hello, IPFS!".to_vec();
    let cid = store_file(file_name, file_content).await?;
    println!("File stored on IPFS with CID: {}", cid);

    Ok(())
}