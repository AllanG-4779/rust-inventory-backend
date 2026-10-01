use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
#[derive(Clone)]
pub struct AppState {
    pub pg_pool: PgPool,
    pub(crate) env: EnvironmentProperties,
}

impl AppState {
    pub async fn new() -> Self {
        let env = EnvironmentProperties::new();
        let db_url = format!(
            "{}://{}:{}@{}:{}/{}",
            env.db_driver, env.db_username, env.db_password, env.db_host, env.db_port, env.db_name
        );
        let connection_pool = PgPoolOptions::new()
            .max_connections(env.db_pool_size as u32)
            .connect(db_url.as_str())
            .await;
        if connection_pool.is_ok() {
            AppState {
                pg_pool: connection_pool.unwrap(),
                env,
            }
        } else {
            println!("Failed to connect to database {:?}", connection_pool.err());
            panic!("Database initialization failed");
        }
    }
}

#[derive(Clone)]
pub struct EnvironmentProperties {
    pub db_pool_size: usize,
    pub db_username: String,
    pub db_password: String,
    pub db_name: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_driver: String,
    pub service_port: u16,
}

impl EnvironmentProperties {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();

        let db_pool_size = Self::read_environment("DB_POOL_SIZE");
        let db_username = Self::read_environment("DB_USERNAME");
        let db_password = Self::read_environment("DP_PASSWORD"); // Kept DP_ per your exact key
        let db_name = Self::read_environment("DB_NAME");
        let db_host = Self::read_environment("DB_HOST");
        let db_port = Self::read_environment("DB_PORT");
        let db_driver = Self::read_environment("DB_DRIVER");
        let server_port: u16 = std::env::var("SERVER_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3000);


        EnvironmentProperties {
            db_pool_size: db_pool_size.parse().unwrap(),
            db_username,
            db_password,
            db_name,
            db_host,
            db_port: db_port.parse().unwrap(),
            db_driver,
            service_port: server_port,
        }
    }

    fn read_environment(name: &str) -> String {
        std::env::var(name)
            .ok()
            .unwrap_or_else(|| panic!("Environment variable {} not set", name))
    }
}
