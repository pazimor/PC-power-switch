use async_hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::sync::Mutex;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080)); // Adresse IP et port à utiliser
    let make_svc = make_service_fn(|_conn| {
        let state = State::default();
        async { Ok::<_, Infallible>(service_fn(move |req| handle_request(req, state.clone()))) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

#[derive(Default)]
struct State {
    led_on: Mutex<bool>,
}

async fn handle_request(
    req: Request<Body>,
    state: State,
) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/api/led/toggle") => {
            let mut led_on = state.led_on.lock().await;
            *led_on = !*led_on;

            // Simuler le clignotement de la LED en basculant son état toutes les 1 seconde
            let delay = time::sleep(Duration::from_secs(1));
            tokio::pin!(delay);
            while delay.await.is_pending() {
                if *led_on {
                    // Allumez la LED ici
                } else {
                    // Éteignez la LED ici
                }
            }

            // Répondez avec un message indiquant l'état actuel de la LED
            let response_text = if *led_on { "LED allumée" } else { "LED éteinte" };
            Ok(Response::new(Body::from(response_text)))
        }
        _ => Ok(Response::builder()
            .status(404)
            .body(Body::empty())
            .unwrap()),
    }
}
