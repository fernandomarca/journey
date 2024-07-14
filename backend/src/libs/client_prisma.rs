use super::PrismaClient;
use std::sync::Arc;

pub async fn prisma() -> Arc<PrismaClient> {
    let prisma_client = Arc::new(
        PrismaClient::_builder()
            .build()
            .await
            .expect("Failed to build PrismaClient"),
    );

    #[cfg(debug_assertions)]
    prisma_client
        ._db_push()
        .await
        .expect("Failed to push the database schema");
    prisma_client
}
