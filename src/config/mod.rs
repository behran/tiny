use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub app: App,
    pub mongo: Mongo,
}

#[derive(Debug, Clone)]
pub struct App {
    pub hostname: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct Mongo {
    pub dsn: String,
    pub database: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            app: App {
                hostname: env::var("APP_HOST").expect("host name"),
                port: env::var("APP_PORT").
                    expect("env `APP_PORT` isn't exis").
                    parse::<u16>().
                    unwrap_or(80),
            },
            mongo: Mongo {
                dsn: env::var("DB_MONGO_DSN").expect("env `DB_MONGO_DSN` isn't exist"),
                database: env::var("DB_MONGO_DATABASE")
                    .expect("env `DB_MONGO_DATABASE` isn't exist"),
            },
        }
    }
}
