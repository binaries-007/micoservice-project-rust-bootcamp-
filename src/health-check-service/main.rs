pub mod authentication {
    tonic::include_proto!("authentication");
}

use authentication::{auth_client::AuthClient, SignInRequest, SignOutRequest, SignUpRequest};
use log::info;
use std::{env, error, time::Duration};
use tokio::time::sleep;
use tonic::Request;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    let default = String::from("[::0]");
    let auth_hostname = env::var("AUTH_SERVICE_HOST_NAME").unwrap_or(default);

    let mut connection = AuthClient::connect(format!("http://{}:50051", auth_hostname)).await?;

    loop {
        let username = Uuid::new_v4().to_string();
        let password = Uuid::new_v4().to_string();

        let request = Request::new(SignUpRequest {
            username: username.clone(),
            password: password.clone(),
        });

        let response = connection.sign_up(request).await?;

        info!(
            "SIGN UP RESPONSE STATUS: {:?}",
            response.into_inner().status_code
        );

        let request = Request::new(SignInRequest {
            username: username.clone(),
            password: password.clone(),
        });

        let response = connection.sign_in(request).await?;

        info!(
            "SIGN IN RESPONSE STATUS: {:?}",
            response.get_ref().status_code
        );

        let request = Request::new(SignOutRequest {
            session_token: response.into_inner().session_token.clone(),
        });

        let response = connection.sign_out(request).await?;

        info!(
            "SIGN OUT RESPONSE STATUS: {:?}",
            response.get_ref().status_code
        );

        info!("=========================================");

        sleep(Duration::from_millis(3000)).await;
    }
}
