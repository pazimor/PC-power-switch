use esp8266_hal::prelude::*;
use esp8266_hal::wifi::station::Station;
use esp8266_hal::wifi::Mac;
use nb::block;

pub struct WifiService {
    ssid: str,
    passwd: str,
    wifi: Station,
    host: Vec<u8>,
    port: u16,
}

impl WifiService {
    pub fn new(ssid: &str, passwd: &str) -> Self {
        //faire un controller GPIO
        let dp = pac::Peripherals::take().unwrap();
        // Configure les broches GPIO nécessaires pour la connexion Wi-Fi
        let mut gpio = dp.GPIO.split();
        let _ = gpio.gpio2.into_function::<esp8266_hal::gpio::Input>();
        let _ = gpio.gpio0.into_function::<esp8266_hal::gpio::Input>();
        let _ = gpio.gpio15.into_function::<esp8266_hal::gpio::Input>();

        // Initialise le matériel Wi-Fi
        return WifiController {
            wifi: Station::new(dp.TMRA, dp.TMRB, dp.WIFI, dp.SYSCON, dp.CLOCK),
            ssid,
            passwd,
            host: [127, 0, 0, 1],
            port: 8080,
        }
    }
    pub fn is_connected(&self) -> bool {
        let status = Ok(wifi.get_status());
        if status == esp8266_hal::wifi::StationStatus::GotIp {
            return true;
        }
        return false;
    }
    pub fn setup(&self) {
        // Configure le réseau Wi-Fi
        wifi.connect(
            self.ssid, self.passwd, None,
            &Mac::broadcast(), None, None,
        ).unwrap();
        self.is_connected(self)
    }
}
