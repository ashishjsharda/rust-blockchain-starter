use futures::stream::TryStreamExt;
use ipfs_api::{IpfsClient, IpfsApi};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Serialize, Deserialize, Debug)]
struct File {
    name: String,
    content: Vec<u8>,
}

pub async fn store_file(name: String, content: Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    let client = IpfsClient::default();

    let file = File {
        name,
        content,
    };

    let serialized = serde_json::to_vec(&file)?;
    let cursor = Cursor::new(serialized);
    let response = client.add(cursor).await?;

    Ok(response.hash)
}

pub async fn retrieve_file(hash: &str) -> Result<File, Box<dyn std::error::Error>> {
    let client = IpfsClient::default();

    let retrieved = client
        .get(hash)
        .map_ok(|chunk| chunk.to_vec())
        .try_collect::<Vec<Vec<u8>>>()
        .await?
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>();

    let deserialized: File = serde_json::from_slice(&retrieved)?;

    Ok(deserialized)
}