use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceConfig {
    pub device_name: String,
    pub fps: u8,
}

impl DeviceConfig {
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self)
    }

    pub fn from_json(json_str: &str) -> Result<Self> {
        serde_json::from_str(json_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_succeeds() {
        let config = DeviceConfig {
            device_name: "Camera A".to_string(),
            fps: 60,
        };

        let json = config.to_json().unwrap();
        assert_eq!(json, r#"{"device_name":"Camera A","fps":60}"#);

        let deserialized: DeviceConfig = DeviceConfig::from_json(&json).unwrap();
        assert_eq!(deserialized.device_name, "Camera A");
        assert_eq!(deserialized.fps, 60);
    }
}
