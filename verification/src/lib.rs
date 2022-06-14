mod cli;
mod compiler;
mod config;
mod http_server;
mod solidity;
mod types;

#[cfg(test)]
mod tests;

pub use self::config::Config;
pub use http_server::run as run_http_server;
pub use http_server::{
    handlers::verification::{VerificationResponse, VerificationResult, VerificationStatus},
    AppRouter,
};
