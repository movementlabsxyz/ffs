use mcr_network_kestrel_coordinator::Kestrel;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	Kestrel::new().run().await
}
