#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Magnet {
    display_name: Option<String>,
    hash_type: Option<String>,
    hash: Option<String>,
    length: Option<u64>,
    source: Option<String>,
    trackers: Vec<String>,
    search_keywords: Option<String>,
    web_seed: Option<String>,
    acceptable_source: Option<String>,
    manifest: Option<String>,
}

impl Magnet {
    pub fn new(magnet_str: &str) -> Result<Magnet, String> {
        if !magnet_str.starts_with("magnet:?") {
            return Err(String::from("not a magnet url"));
        }

        let mut magnet = Magnet {
            display_name: None,
            hash_type: None,
            hash: None,
            length: None,
            source: None,
            trackers: Vec::new(),
            search_keywords: None,
            web_seed: None,
            acceptable_source: None,
            manifest: None,
        };

        // Skip the magnet:? prefix
        let params_str = magnet_str.trim_start_matches("magnet:?");
        // decode http charecter encoding
        let decoded = urlencoding::decode(params_str).expect("UTF-8");

        // Split parameters by &
        for param in decoded.split('&') {
            if let Some((key, value)) = param.split_once('=') {
                match key {
                    "dn" => magnet.display_name = Some(value.to_string()),
                    "xt" => {
                        // Handle xt=urn:hash_type:hash format
                        if let Some(urn_part) = value.strip_prefix("urn:") {
                            if let Some((hash_type, hash)) = urn_part.split_once(':') {
                                magnet.hash_type = Some(hash_type.to_string());
                                magnet.hash = Some(hash.to_string());
                            }
                        }
                    }
                    "xl" => {
                        if let Ok(len) = value.parse::<u64>() {
                            magnet.length = Some(len);
                        }
                    }
                    "tr" => magnet.trackers.push(value.to_string()),
                    "kt" => magnet.search_keywords = Some(value.to_string()),
                    "ws" => magnet.web_seed = Some(value.to_string()),
                    "xs" => magnet.source = Some(value.to_string()),
                    "as" => magnet.acceptable_source = Some(value.to_string()),
                    "mt" => magnet.manifest = Some(value.to_string()),
                    _ => {}
                }
            }
        }
        Ok(magnet)
    }
}
