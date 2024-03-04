use sqlx::Pool;

pub mod user;

#[derive(Clone)]
pub struct DB {
    pool: Pool<sqlx::Postgres>,
}

impl DB {
    pub fn new(pool: Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }
}
