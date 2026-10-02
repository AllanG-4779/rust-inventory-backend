use crate::services::tenants::persistence::repository::TenantRepository;
use crate::services::tenants::service::tenant_service::TenantService;
use sqlx::PgPool;
#[derive(Clone)]
pub struct AppState {
    pub pg_pool: PgPool,
    pub(crate) env: EnvironmentProperties,
    pub tenant_service: TenantService,
}

impl AppState {
    pub fn new(pg_pool: PgPool, env: EnvironmentProperties) -> Self {
        Self {
            tenant_service: TenantService::new(TenantRepository::new(pg_pool.clone())),
            pg_pool,
            env,
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
