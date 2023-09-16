use esp8266_hal::prelude::*;
use esp8266_hal::wifi::station::Station;
use esp8266_hal::wifi::Mac;
use nb::block;

struct WifiControler {
    ssid: str,
    passwd: str,
}

impl WifiControler {
    fn new(ssid: &str, passwd: &str) -> Self {
        WifiControler {
            ssid,
            passwd
        }
    }
    fn test_connection(&self) {
        match wifi.get_status() {
            Ok(status) => {
                if status == esp8266_hal::wifi::StationStatus::GotIp {
                    println!("Connecté à Internet via Wi-Fi");
                }
            }
            Err(_) => {
                println!("Échec de la connexion Wi-Fi");
            }
        }
    }
    fn setup(&self) {
        let dp = pac::Peripherals::take().unwrap();

        // Configure les broches GPIO nécessaires pour la connexion Wi-Fi
        let mut gpio = dp.GPIO.split();
        let _ = gpio.gpio2.into_function::<esp8266_hal::gpio::Input>();
        let _ = gpio.gpio0.into_function::<esp8266_hal::gpio::Input>();
        let _ = gpio.gpio15.into_function::<esp8266_hal::gpio::Input>();

        // Initialise le matériel Wi-Fi
        let mut wifi = Station::new(dp.TMRA, dp.TMRB, dp.WIFI, dp.SYSCON, dp.CLOCK);

        // Configure le réseau Wi-Fi


        wifi.connect(
            self.ssid, self.passwd, None,
            &Mac::broadcast(), None, None,
        ).unwrap();

        // Attendez que la connexion soit établie
    }
}
