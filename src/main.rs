use rouille::Request;

mod services::WifiService;
use services::WifiService::WifiService;

#[actix_web::main]
async fn main() {
    let mut wifi = WifiService::new("", ""); //ssid, passwd
    rouille::start_server("0.0.0.0:8080", move |request| {
        router(request)
    });    
    //let addr = SocketAddr::from((wifi.host, wifi.port)); // old version
}

fn router(request: &Request) -> rouille::Response {
    router!(request,
        (GET) (/) => {
            rouille::Response::text("Hello, World!")
        },
        _ => rouille::Response::empty_404()
    )
}
