#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3030,
            database_url: "inmemory".to_string(),
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3030".to_string())
                .parse()
                .unwrap_or(3030),
            database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| "inmemory".to_string()),
        }
    }
}
