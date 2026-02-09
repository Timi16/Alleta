use anyhow::Result;
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub arbitrum_one_rpc: String,
    pub arbitrum_sepolia_rpc: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();

        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse()
                .expect("PORT must be a valid number"),
            arbitrum_one_rpc: env::var("ARBITRUM_ONE_RPC")
                .unwrap_or_else(|_| "https://arb1.arbitrum.io/rpc".to_string()),
            arbitrum_sepolia_rpc: env::var("ARBITRUM_SEPOLIA_RPC")
                .unwrap_or_else(|_| "https://sepolia-rollup.arbitrum.io/rpc".to_string()),
        })
    }

    pub fn get_rpc_url(&self, chain: &str) -> String {
        match chain {
            "arbitrum-one" => self.arbitrum_one_rpc.clone(),
            "arbitrum-sepolia" => self.arbitrum_sepolia_rpc.clone(),
            _ => self.arbitrum_one_rpc.clone(), // Default to mainnet
        }
    }
}