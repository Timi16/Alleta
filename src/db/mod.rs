use anyhow::Result;
use std::sync::Arc;

// Prisma client will be generated at build time
include!(concat!(env!("OUT_DIR"), "/prisma.rs"));

pub type PrismaClient = Arc<prisma::PrismaClient>;

pub async fn create_client(database_url: &str) -> Result<PrismaClient> {
    let client = prisma::PrismaClient::_builder()
        .with_url(database_url.to_string())
        .build()
        .await?;

    Ok(Arc::new(client))
}

pub async fn save_report(
    client: &PrismaClient,
    id: String,
    tx_hash: String,
    chain: String,
    report_data: serde_json::Value,
    expires_at: chrono::DateTime<chrono::Utc>,
) -> Result<()> {
    client
        .report()
        .create(
            id,
            tx_hash,
            chain,
            report_data,
            expires_at.into(),
            vec![],
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn get_report(
    client: &PrismaClient,
    report_id: String,
) -> Result<Option<prisma::report::Data>> {
    let report = client
        .report()
        .find_unique(prisma::report::id::equals(report_id))
        .exec()
        .await?;

    Ok(report)
}

pub async fn find_report_by_tx(
    client: &PrismaClient,
    tx_hash: String,
    chain: String,
) -> Result<Option<prisma::report::Data>> {
    let report = client
        .report()
        .find_first(vec![
            prisma::report::tx_hash::equals(tx_hash),
            prisma::report::chain::equals(chain),
        ])
        .exec()
        .await?;

    Ok(report)
}