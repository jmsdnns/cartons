use tonic::{transport::Server, Request, Response, Status};
use vibing::{
    vibing_server::{Vibing, VibingServer},
    VibeRequest, VibeResponse,
};

pub mod vibing {
    tonic::include_proto!("vibing");
}

#[derive(Debug, Default)]
pub struct VibingService {}

#[tonic::async_trait]
impl Vibing for VibingService {
    async fn vibe(&self, request: Request<VibeRequest>) -> Result<Response<VibeResponse>, Status> {
        let r = request.into_inner();
        match r.vibe {
            0 => {
                println!("[bussin] {}", r.comment);
                return Ok(Response::new(vibing::VibeResponse {
                    confirmation: { format!("[bussin] {}", r.comment) },
                }));
            }
            1 => {
                println!("[skrrt]  {}", r.comment);
                return Ok(Response::new(vibing::VibeResponse {
                    confirmation: { format!("[skrrt]  {}", r.comment) },
                }));
            }
            _ => Err(Status::new(tonic::Code::OutOfRange, "Invalid vibe")),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:5454".parse().unwrap();
    let service = VibingService::default();

    Server::builder()
        .add_service(VibingServer::new(service))
        .serve(address)
        .await?;
    Ok(())
}
