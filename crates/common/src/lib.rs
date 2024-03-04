use std::sync::Arc;

use database::user::UserDB;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct DBConfig {
    pub url: Option<String>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Configuration {
    pub db: Option<DBConfig>,
}

pub struct Context<T>
where
    T: UserDB + 'static,
{
    pub db: Arc<T>,
    pub config: Arc<Configuration>,
}

impl Configuration {
    pub fn db_url(&self) -> String {
        match self.db {
            None => panic!("Database configuration not found"),
            Some(ref db) => match db.url {
                None => panic!("Database URL not found"),
                Some(ref url) => url.clone(),
            },
        }
    }
}
