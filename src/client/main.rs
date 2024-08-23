use std::env;

use clap::{Parser, Subcommand};

use authentication::{auth_client::AuthClient, SignInRequest, SignOutRequest, SignUpRequest};
use log::info;
use tonic::Request;

pub mod authentication {
    tonic::include_proto!("authentication");
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    SignIn {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },

    Signup {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },

    SignOut {
        #[arg(short, long)]
        session_token: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let default_ip = "[::0]".to_owned();
    let auth_ip = env::var("AUTH_SERVICE_IP").unwrap_or(default_ip);
    let mut client = AuthClient::connect(format!("http://{}:50051", auth_ip)).await?;

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::SignIn { username, password }) => {
            let request = Request::new(SignInRequest {
                username: username.to_owned(),
                password: password.to_owned(),
            });

            let response = client.sign_in(request).await?.into_inner();

            info!("{:?}", response);
        }

        Some(Commands::Signup { username, password }) => {
            let request = Request::new(SignUpRequest {
                username: username.to_owned(),
                password: password.to_owned(),
            });

            let response = client.sign_up(request).await?.into_inner();

            info!("{:?}", response);
        }

        Some(Commands::SignOut { session_token }) => {
            let request = Request::new(SignOutRequest {
                session_token: session_token.to_owned(),
            });

            let response = client.sign_out(request).await?.into_inner();

            info!("{:?}", response);
        }

        None => {}
    }

    Ok(())
}
