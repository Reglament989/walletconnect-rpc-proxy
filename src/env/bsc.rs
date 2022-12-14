use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BscConfig {
    #[serde(default = "default_bsc_supported_chains")]
    pub supported_chains: HashMap<String, String>,
}

impl Default for BscConfig {
    fn default() -> Self {
        Self {
            supported_chains: HashMap::from([
                // BNB smart chain
                ("eip155:56".into(), "binance".into()),
                ("eip155:97".into(), "binance_test".into()),
            ]),
        }
    }
}

fn default_bsc_supported_chains() -> HashMap<String, String> {
    HashMap::from([
        // BNB smart chain
        ("eip155:56".into(), "binance".into()),
        ("eip155:97".into(), "binance_test".into()),
    ])
}
