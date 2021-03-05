/// Receive new crates by cargo exec command: cargo publish ....
///
/// The structure of cargo library is referenced.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewCrate {
    pub name: String,
    pub vers: String,
    pub deps: Vec<NewCrateDependency>,
    pub features: BTreeMap<String, Vec<String>>,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub homepage: Option<String>,
    pub readme: Option<String>,
    pub readme_file: Option<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub license: Option<String>,
    pub license_file: Option<String>,
    pub repository: Option<String>,
    pub badges: BTreeMap<String, BTreeMap<String, String>>,
    pub links: Option<String>,
}

/// The structure of cargo library is referenced.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewCrateDependency {
    pub optional: bool,
    pub default_features: bool,
    pub name: String,
    pub features: Vec<String>,
    pub version_req: String,
    pub target: Option<String>,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_name_in_toml: Option<String>,
}

// 2021-01-22 01:43:37:654200000 [INFO] <actix_server::builder:263>:Starting 4 workers
// 2021-01-22 01:43:37:655028000 [INFO] <actix_server::builder:277>:Starting "actix-web-service-127.0.0.1:8080" service on 127.0.0.1:8080
// 2021-01-22 01:49:13:270352000 [INFO] <pargo::api::server:11>:
// HttpRequest HTTP/1.1 PUT:/api/v1/crates/new
// headers:
// "accept": "application/json"
// "expect": "100-continue"
// "content-length": "73183"
// "authorization": "xxxadmin"
// "host": "localhost:8080"
// "user-agent": "cargo 1.49.0 (d00d64df9 2020-12-05)"
//
//

use crate::api::error::ApiResult;
use actix_web::{put, web, HttpMessage, HttpRequest, HttpResponse};
use byteorder::{LittleEndian, ReadBytesExt};
use futures_core::stream::Stream;
use futures_util::StreamExt;
use std::collections::BTreeMap;
use std::fs;
use std::io::Cursor;
use std::io::{Read, Write};
use std::path::Path;

#[put("/api/v1/crates/new")]
pub async fn new_crate(req: HttpRequest, mut body: web::Payload) -> ApiResult<HttpResponse> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }
    let mut cursor = Cursor::new(bytes);
    let metadata_len = cursor.read_u32::<LittleEndian>()?;
    let mut crate_buf = vec![0u8; metadata_len as usize];
    cursor.read_exact(&mut crate_buf)?;
    let new_crate: NewCrate = serde_json::from_slice(&crate_buf)?;

    let file_size = cursor.read_u32::<LittleEndian>()?;
    let mut file_bytes = vec![0u8; file_size as usize];
    cursor.read_exact(&mut file_bytes)?;
    let hash = sha256::digest_bytes(&file_bytes);
    info!("hash:{}", hash);

    let path = Path::new("rust_demo.crates");
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&path)
        .unwrap();
    file.write_all(file_bytes.as_ref()).unwrap();

    Ok(HttpResponse::Ok().finish())
}
