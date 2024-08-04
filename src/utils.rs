use ring::digest::{Context, SHA256};

pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut context = Context::new(&SHA256);
    context.update(data);
    let digest = context.finish();
    digest.as_ref().to_vec()
}

pub fn base58_encode(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}

pub fn base58_decode(data: &str) -> Vec<u8> {
    bs58::decode(data).into_vec().unwrap()
}
