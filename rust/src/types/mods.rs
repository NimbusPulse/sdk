use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChangeModsRequest {
    pub install_mods: Vec<ModConfigType>,
    pub uninstall_mods: Vec<ModConfigType>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ModConfigType {
    #[serde(rename = "srs")]
    Srs,
    #[serde(rename = "tacview")]
    TacView,
    #[serde(rename = "olympus")]
    Olympus,
    #[serde(rename = "lotatc")]
    LotAtc,
    #[serde(rename = "webconsole")]
    WebConsole,
    #[serde(rename = "enhanced_metrics")]
    EnhancedMetrics,
    #[serde(rename = "a_4e_c")]
    A4Ec,
    #[serde(rename = "bronco_ov_10a")]
    BroncoOv10a,
    #[serde(rename = "real_weather")]
    RealWeather,
    #[serde(rename = "sr_ea_proxy")]
    SrEaProxy,
    #[serde(rename = "aerosimics")]
    Aerosimics,
}

impl ModConfigType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Srs => "srs",
            Self::TacView => "tacview",
            Self::Olympus => "olympus",
            Self::LotAtc => "lotatc",
            Self::WebConsole => "webconsole",
            Self::EnhancedMetrics => "enhanced_metrics",
            Self::A4Ec => "a_4e_c",
            Self::BroncoOv10a => "bronco_ov_10a",
            Self::RealWeather => "real_weather",
            Self::SrEaProxy => "sr_ea_proxy",
            Self::Aerosimics => "aerosimics",
        }
    }
}

impl std::fmt::Display for ModConfigType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
