use proto::calculator_server::{Calculator, CalculatorServer};
use tonic::transport::Server;

mod proto {
    tonic::include_proto!("calculator");
}

#[derive(Default, Debug)]
struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
        let response = proto::CalculationResponse {
            result: request.get_ref().a + request.get_ref().b,
        };
        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let calculator_service = CalculatorService::default();
    Server::builder()
        .add_service(CalculatorServer::new(calculator_service))
        .serve(addr)
        .await?;
    Ok(())
}
