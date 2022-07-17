use api::{
    greeter_service_server::{GreeterService, GreeterServiceServer},
    HelloReply, HelloRequest,
};
use tonic::{transport::Server, Request, Response, Status};

use clap::Parser;

pub mod api {
    tonic::include_proto!("greet");
}

#[derive(Debug, Default)]
pub struct Greeter {}

#[tonic::async_trait]
impl GreeterService for Greeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "greet-server - a simple greet microservice", long_about = None)]
struct ServerCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "50052")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ServerCli::parse();
    let addr = format!("{}:{}", cli.server, cli.port).parse()?;
    let echo = Greeter::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(GreeterServiceServer::new(echo))
        .serve(addr)
        .await?;

    Ok(())
}
