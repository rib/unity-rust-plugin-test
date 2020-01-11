#![allow(unused_imports)]

use std::sync::Once;

use std::env;
use std::time::{Duration, SystemTime};
use std::net::SocketAddr;

use futures::stream::Stream;
use futures::stream::StreamExt;
use futures_util::pin_mut;

use tracing::{trace, debug, info, warn, error, span};
use tracing::{trace_span, debug_span, info_span, warn_span, error_span};
use tracing::{Level, instrument};
use tracing_futures::Instrument;
use tracing_subscriber;

use tokio::{time};
use tokio::runtime::Runtime;

use tonic::{
    body::BoxBody,
    transport::{Identity, Server, ServerTlsConfig, Channel, Certificate},
    Request, Response, Status, Streaming
};

use async_stream::try_stream;

pub mod test_grpc {
    tonic::include_proto!("test.grpc");
}

use test_grpc::{
    test_client::{TestClient},
    Empty, Ping, Pong
};

struct TestService {}

async fn connect_to_test_server() -> TestClient<Channel>
{
    let port = env::var("GRPC_PORT").unwrap_or("50051".to_string());
    let uri = format!("http://localhost:{}", port).parse::<http::Uri>().unwrap();

    let endpoint: tonic::transport::Endpoint = uri.into();

    for _ in 0..60 {
        match TestClient::connect(endpoint.clone()).await {
            Ok(client) => {
                info!("Connected to test server");
                return client;
            }
            Err(_) => {
                error!("Failed to connect to test server");
                tokio::time::delay_for(Duration::from_secs(1)).await;
                continue;
            }
        }
    }

    panic!("Failed to connect to test server");
}

#[no_mangle]
fn hello() -> i32
{
    info!("hello");
    println!("hello");

    11
}

#[no_mangle]
fn spawn_tokio_runtime() -> i32
{
    println!("spawn_tokio_runtime");

    static START: Once = Once::new();

    START.call_once(|| {
        let mut rt = Runtime::new().unwrap();


        // I think we need a 'root task'...
        rt.block_on(async {
            loop {
                tokio::time::delay_for(Duration::from_secs(1)).await;
                info!("Root task tick");
                println!("root task tick");
            }
        });
    });

    return 2;
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {

    let _channel = connect_to_test_server().await;

    Ok(())
}

#[no_mangle]
fn test() -> i32
{
    println!("test");

    tokio::spawn(async {
        println!("task");
        /*
        match run().await {
            Ok(()) => println!("OK"),
            Err(e) => println!("ERROR: Failed to run test {:?}", e)
        }
        */
    });

    5
}

//#[tokio::main]
//async fn main() -> Result<(), Box<dyn std::error::Error>> {
//    tracing_subscriber::fmt::init();
//    run().instrument(trace_span!("run")).await
//}
