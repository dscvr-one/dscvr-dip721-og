use crate::management::is_fleek;
use crate::utils::*;
use ic_kit::candid::CandidType;
use ic_kit::ic;
use ic_kit::macros::*;
use serde::Deserialize;
use serde::Serialize;
use serde_bytes::ByteBuf;
use std::collections::HashMap;

pub fn assets<'a>() -> &'a mut AssetStore {
    ic_kit::ic::get_mut::<AssetStore>()
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, Default)]
pub struct AssetStore {
    pub assets: HashMap<u64, Vec<u8>>,
}

#[update]
async fn add_asset(index: u64, data: Vec<u8>) -> () {
    if !is_fleek(&ic::caller()) {
        return;
    }
    assets().assets.insert(index, data);
}

type HeaderField = (String, String);

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: ByteBuf,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpResponse {
    status_code: u16,
    headers: Vec<HeaderField>,
    body: Vec<u8>,
}

#[query]
async fn http_request(req: HttpRequest) -> HttpResponse {
    let parts: Vec<&str> = req.url.split('/').collect();
    let mut headers: Vec<HeaderField> = Vec::new();

    match parts[1].parse::<u64>() {
        Ok(token_id) => {
            if let Some(_) = ledger().tokens.get(&(token_id as u32)) {
                if let Some(data) = assets().assets.get(&(token_id % 17)) {
                    headers.push(("content-type".to_string(), "image/jpeg".to_string()));
                    headers.push((
                        "cache-control".to_string(),
                        "public, max-age=15552000".to_string(),
                    ));
                    return HttpResponse {
                        status_code: 200,
                        headers,
                        body: data.clone(),
                    };
                } else {
                    ic_cdk::println!("Asset not found: {}", (token_id % 17));
                }
            }
        }
        Err(_) => {
            ic_cdk::println!("Failed to parse: {}", parts[1]);
        }
    }

    return HttpResponse {
        status_code: 200,
        headers,
        body: Vec::new(),
    };
}
