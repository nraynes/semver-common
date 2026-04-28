use derive_getters::Getters;
use r_log::LogLevel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters)]
pub struct PluginConfig<T> {
    log_level: LogLevel,
    config: T,
}
