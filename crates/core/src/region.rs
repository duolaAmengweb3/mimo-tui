//! Region selection (CN / SGP / AMS).

use std::time::Duration;

use mimo_tui_anthropic_client::Region;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RegionConfig {
    Cn,
    Sgp,
    Ams,
}

impl RegionConfig {
    pub fn to_client_region(self) -> Region {
        match self {
            RegionConfig::Cn => Region::Cn,
            RegionConfig::Sgp => Region::Sgp,
            RegionConfig::Ams => Region::Ams,
        }
    }

    pub fn label(self) -> &'static str {
        self.to_client_region().label()
    }

    pub fn all() -> [RegionConfig; 3] {
        [RegionConfig::Cn, RegionConfig::Sgp, RegionConfig::Ams]
    }
}

impl Default for RegionConfig {
    fn default() -> Self {
        RegionConfig::Sgp
    }
}

/// Measured latency to one region.
#[derive(Debug, Clone)]
pub struct Latency {
    pub region: RegionConfig,
    pub ms: Option<u64>,
}

/// Ping each region's `/anthropic/v1/messages` with a HEAD-ish probe.
/// We can't actually issue a Messages request without billing, so we just
/// open a TCP+TLS handshake to the host.
pub async fn measure_all() -> Vec<Latency> {
    let mut out = Vec::new();
    for region in RegionConfig::all() {
        let ms = measure_one(region).await;
        out.push(Latency { region, ms });
    }
    out
}

async fn measure_one(region: RegionConfig) -> Option<u64> {
    let url = format!(
        "{}/v1/messages",
        region.to_client_region().base_url()
    );
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .ok()?;
    let start = std::time::Instant::now();
    // GET /v1/messages will 404/405 but the handshake is what we time.
    let _ = client.get(&url).send().await.ok()?;
    Some(start.elapsed().as_millis() as u64)
}

/// Best region by latency, falling back to SGP.
pub fn best_region(measurements: &[Latency]) -> RegionConfig {
    measurements
        .iter()
        .filter_map(|m| m.ms.map(|ms| (m.region, ms)))
        .min_by_key(|(_, ms)| *ms)
        .map(|(r, _)| r)
        .unwrap_or(RegionConfig::Sgp)
}
