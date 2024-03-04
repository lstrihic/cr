use async_trait::async_trait;

use crate::DB;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
}

#[async_trait]
pub trait UserDB {
    async fn find_all_users(&self) -> anyhow::Result<Vec<User>>;
    async fn find_user_by_id(&self, id: &str) -> anyhow::Result<User>;
}

#[async_trait]
impl UserDB for DB {
    async fn find_all_users(&self) -> anyhow::Result<Vec<User>> {
        let result = sqlx::query_as!(User, "SELECT id, email FROM users")
            .fetch_all(&self.pool)
            .await?;
        return Ok(result);
    }

    async fn find_user_by_id(&self, id: &str) -> anyhow::Result<User> {
        let result = sqlx::query_as!(User, "SELECT id, email FROM users WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await?;
        return Ok(result);
    }
}
