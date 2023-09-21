
pub struct WifiService {
    ssid: str,
    passwd: str,
    wifi: Station,
    host: Vec<u8>,
    port: u16,
}

impl WifiService {
    pub fn new(ssid: &str, passwd: &str) -> Self { /**/ }
    pub fn is_connected(&self) -> bool { /*am i connected*/ }
    pub fn setup(&self) {/*Configure Wi-Fi*/}
}
