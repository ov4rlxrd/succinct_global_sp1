use axum::{
    routing::{get, post},
    Router,
    Json,
};
use tower_http::cors::{CorsLayer, Any};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use sp1_sdk::{include_elf, ProverClient, SP1ProofWithPublicValues, SP1Stdin, HashableKey};
const ELF: &[u8] = include_elf!("program");
#[derive(Serialize, Deserialize)]
struct Score {
    number: u32
}
#[derive(Serialize, Deserialize)]
struct ProofData {
    proof: String,         
    public_inputs: String, 
    vkey_hash: String,     
}
async fn ping() -> &'static str {
    "Welcome to Succinct Global"
}

async fn generate_proof(Json(score): Json<Score>) -> Json<ProofData>{
    println!("Starting proving");
    let proof_path = format!("../proofs/proof.bin");
    let json_path = format!("../proofs/proof.json");
    let mut stdin = SP1Stdin::new();
    stdin.write(&score.number);

    let client = ProverClient::from_env();
  
    let (pk, vk) = client.setup(ELF);
    let proof = match client.prove(&pk, &stdin).groth16().run() {
        Ok(p) => p,
        Err(e) => {
            println!("Ошибка при генерации proof: {}", e);
            return Json(ProofData {
                proof: "".to_string(),
                public_inputs: "".to_string(),
                vkey_hash: "".to_string(),
            });
        }
    };    println!("Got proved");
	
    proof.save(&proof_path).expect("Failed to save proof");
    println!("Proof saved");
	
    let proof = SP1ProofWithPublicValues::load(&proof_path).expect("Failed to load proof");
    println!("Proof Loaded");
	
    let fixture = ProofData {
        proof: hex::encode(proof.bytes()),
        public_inputs: hex::encode(proof.public_values),
        vkey_hash: vk.bytes32(),
    };
    println!("Fixture Extracted");
	
    let json_proof = serde_json::to_string(&fixture).expect("Failed to serialize proof");
    println!("Serialized");
	
    std::fs::write(json_path, json_proof).expect("Failed to write JSON proof");
    println!("JSON Written");
    
    

    println!("Successfully generated json proof for the program!");

    Json(fixture)
}

#[tokio::main]
async fn main() {
	let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);
    let app = Router::new()
        .route("/ping", get(ping))             
        .route("/score", post(generate_proof)) 
		.layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Сервер запущен на {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
