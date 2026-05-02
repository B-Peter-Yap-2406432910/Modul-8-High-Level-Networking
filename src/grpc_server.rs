use tonic::{transport::Server, Request, Response, Status};
use services::{payment_service_server::{PaymentService, PaymentServiceServer}, ProcessPaymentRequest, ProcessPaymentResponse};



#[derive(Default)]
pub struct MyPaymentService {}

#[tonic::async_trait]
impl PaymentService for MyPaymentService {
    async fn process_payment(
        &self,
        request: Request<ProcessPaymentRequest>,
    ) -> Result<Response<ProcessPaymentResponse>, Status> {
        println!("Received payment request: {:?}", request);

        // Process the request and return a response
        // This example immediately return a successful response for demonstration purposes
        Ok(Response::new(ProcessPaymentResponse { success: true }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let payment_service = MyPaymentService::default();

    println!("PaymentServiceServer listening on {}", addr);

    Server::builder()
        .add_service(PaymentServiceServer::new(payment_service))
        .serve(addr)
        .await?;

    Ok(())
}

pub mod services {
    tonic::include_proto!("services");
}

