use std::collections::HashMap;

use serde_derive::Deserialize;

#[derive(Deserialize, Default)]
pub struct Configuration {
    #[serde(flatten)]
    pub actions: HashMap<String, String>
}

