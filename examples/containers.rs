use containerd::containerd::services::containers::v1::client::ContainersClient;
use containerd::containerd::services::containers::v1::{
    ListContainersRequest
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://127.0.0.1:9000".to_string();
    let filters = Vec::new();
    let mut client = ContainersClient::connect(url.clone()).await?;
    let mut request = tonic::Request::new(ListContainersRequest {
        filters: filters,
    });

    // required context, and for docker containers we specify "moby"
    let request_map = request.metadata_mut();
    request_map.insert("containerd-namespace", "moby".parse().unwrap());

    let response = client.list(request).await?.into_inner();
    for container in response.containers {
        println!("{}", container.id);
    }
    Ok(())
}
