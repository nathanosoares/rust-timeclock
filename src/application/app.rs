use async_trait::async_trait;

#[async_trait]
pub trait App {
    async fn start(&self);
}
