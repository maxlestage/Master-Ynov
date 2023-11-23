#[allow(warnings, unused)]
mod prisma;

use prisma::PrismaClient;
use prisma_client_rust::NewClientError;

#[tokio::main]
async fn main() {
    println!("Hello there, you're now on ‘Thunder’ api!");
    let client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;
}
