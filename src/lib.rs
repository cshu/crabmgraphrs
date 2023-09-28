use crabrs::*;
use crabwebrs::*;
//use log::*;
use reqwest::*;
//use serde::{Deserialize, Serialize};
use std::time::*;

pub fn mk_client(mut access_token: String) -> CustRes<reqwest::blocking::Client> {
    access_token.insert_str(0, "Bearer ");
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", access_token.parse()?);
    let client = reqwest::blocking::Client::builder()
        .connect_timeout(Duration::from_secs(30))
        .default_headers(headers)
        .build()?;
    Ok(client)
}

pub fn me(cli: &reqwest::blocking::Client) -> CustRes<serde_json::Map<String, serde_json::Value>> {
    let rebder = cli.get("https://graph.microsoft.com/v1.0/me");
    let bytes = easy_http_bytes(rebder)?;
    let robj: serde_json::Value = serde_json::from_slice(&bytes)?;
    if let serde_json::Value::Object(retval) = robj {
        Ok(retval)
    } else {
        dummy_err("Not obj")
    }
}

/// "userPrincipalName", usually same as "mail" field
pub fn get_upn(cli: &reqwest::blocking::Client) -> CustRes<String> {
    let sjv: serde_json::Value = me(cli)?
        .get("userPrincipalName")
        .ok_or("UPN not found")?
        .to_owned();
    match sjv {
        serde_json::Value::String(inner) => Ok(inner),
        _ => dummy_err("Type mismatch"),
    }
}
