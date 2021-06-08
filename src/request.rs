use anyhow::anyhow;
use reqwest::header::CONTENT_TYPE;
use reqwest::StatusCode;

use crate::{Element, Error};

pub struct Request {}

impl Request {
    pub fn new() -> Self {
        Self {}
    }

    pub fn request_url(&mut self, url: &String) -> anyhow::Result<Element> {
        let resp = reqwest::blocking::get(url)?;

        if !resp.status().is_success() {
            return Err(anyhow!("http status error: {:?}", resp.status()));
        }

        let option = resp.headers()
            .get(CONTENT_TYPE)
            .map(|h| h.to_str().unwrap_or(""));

        let ele = match option {
            Some("text/html") => Element::HTML(resp.text()?),

            Some("application/json") => Element::JSON(resp.text()?),

            Some(t) if t.starts_with("text/") => Element::TEXT {
                subtype: t.strip_prefix("text/").unwrap().to_string(),
                body: resp.text()?,
            },

            Some(t) if t.starts_with("image/") => Element::IMAGE {
                subtype: t.strip_prefix("image/").unwrap().to_string(),
                body: resp.bytes()?,
            },

            Some(t) => Element::OTHER {
                c_type: t.split_once("/").map(|t| t.0).unwrap_or("").to_string(),
                subtype: t.split_once("/").map(|t| t.1).unwrap_or("").to_string(),
                body: resp.bytes()?,
            },

            None => Element::OTHER {
                body: resp.bytes()?,
                c_type: "".to_string(),
                subtype: "".to_string(),
            },
        };

        Ok(ele)
    }
}