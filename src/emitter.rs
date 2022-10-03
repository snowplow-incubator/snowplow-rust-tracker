// Copyright (c) 2022 Snowplow Analytics Ltd. All rights reserved.
//
// This program is licensed to you under the Apache License Version 2.0,
// and you may not use this file except in compliance with the Apache License Version 2.0.
// You may obtain a copy of the Apache License Version 2.0 at http://www.apache.org/licenses/LICENSE-2.0.
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the Apache License Version 2.0 is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the Apache License Version 2.0 for the specific language governing permissions and limitations there under.

use crate::payload::Payload;
use reqwest::Client;
use serde_json::json;

pub struct Emitter {
    pub collector_url: String,
    http_client: Client,
}

/// Emitter is responsible for emitting tracked events to the Snowplow Collector
impl Emitter {
    pub fn new(collector_url: &str) -> Emitter {
        Emitter {
            collector_url: collector_url.to_string(),
            http_client: Client::new(),
        }
    }

    /// Add event to be sent to the Collector
    pub async fn add(&self, payload: Payload) -> Result<(), reqwest::Error> {
        let result = self.post(payload).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn post(&self, payload: Payload) -> Result<String, reqwest::Error> {
        let collector_url = self.collector_url.to_string() + "/com.snowplowanalytics.snowplow/tp2";

        let payload = json!({
            "schema": "iglu:com.snowplowanalytics.snowplow/payload_data/jsonschema/1-0-4",
            "data": vec![payload]
        });

        let resp = self
            .http_client
            .post(collector_url)
            .json(&payload)
            .send()
            .await?;

        resp.text().await
    }
}
