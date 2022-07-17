use clap::Parser;

use api::{greeter_service_client::GreeterServiceClient, HelloRequest};

pub mod api {
    tonic::include_proto!("greet");
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "greet - a simple CLI to send messages to a server", long_about = None)]
struct ClientCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "50052")]
    port: u16,
    /// The message to send
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ClientCli::parse();

    let mut client = GreeterServiceClient::connect(format!("http://{}:{}", cli.server, cli.port)).await?;

    let request = tonic::Request::new(HelloRequest { name: cli.name });

    let response = client.say_hello(request).await?;

    println!("{:?}", response.into_inner().message);

    Ok(())
}
