pub mod aes;
mod api;
use crate::pb::*;
use aes::*;
use once_cell::sync::OnceCell;

use anyhow::{anyhow, Result};
use base64::{decode, encode};
use prost::Message;
use worker::Fetcher;

// static HOST: &str = "https://bone.saltedfish.fun/_api/paste";
// static PREFIX: &str = "##PasteMe##";
#[derive(Debug)]
pub enum DeleteType {
    NoDelete,
    DeleteToken(String),
}
#[derive(Debug)]
pub enum PasteType {
    Once,
    OneDay,
    SevenDays,
    Forever,
}

impl From<i32> for PasteType {
    fn from(i: i32) -> Self {
        match i {
            0 => PasteType::Once,
            1 => PasteType::OneDay,
            2 => PasteType::SevenDays,
            3 => PasteType::Forever,
            _ => PasteType::Once,
        }
    }
}

#[derive(Debug)]
pub struct PasteConfig {
    pub(crate) delete_config: DeleteType,
    pub(crate) paste_type: PasteType,
}

impl Default for PasteConfig {
    fn default() -> Self {
        PasteConfig {
            delete_config: DeleteType::NoDelete,
            paste_type: PasteType::Once,
        }
    }
}

// fn encrypt(content: String, password: &str) -> Result<String> {
//     if password.is_empty() {
//         Ok(content)
//     } else {
//         Ok(aes_encrypt(&format!("{}{}", PREFIX, content), password)?)
//     }
// }

// pub fn decrypt(content: &str, password: &str) -> Result<String> {
//     if password.is_empty() {
//         Ok(content.to_owned())
//     } else {
//         let content = aes_decrypt(content, password)?;
//         // println!("{}", content);
//         if content.starts_with(PREFIX) {
//             Ok(content.split_at(PREFIX.len()).1.to_owned())
//         } else {
//             Err(anyhow!("Decrypt Error!"))
//         }
//     }
// }
// impl Paste {
//     pub fn decrypt(&self, password: &str) -> Option<String> {
//         if password.is_empty() {
//             return None;
//         }
//         // Some(decrypt(&self.content, password).unwrap())
//         decrypt(&self.content, password).ok()
//     }
// }

