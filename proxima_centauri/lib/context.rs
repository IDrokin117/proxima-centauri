use crate::auth::Database;
use crate::config::Config;
use crate::registry::Registry;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub(crate) struct Context {
    pub(crate) config: Arc<Config>,
    pub(crate) database: Arc<Database>,
    pub(crate) registry: Arc<Mutex<Registry>>,
}

impl Context {
    pub(crate) fn new(config: Config, database: Database, registry: Registry) -> Self {
        Self {
            config:Arc::new(config),
            database:Arc::new(database),
            registry:Arc::new(Mutex::new(registry)),
        }
    }
}
