use soroban_sdk::{contracttype, symbol_short, Address, Bytes, Env, String, Vec};
use crate::request_id::RequestId;
use crate::Error;

/// Logging configuration for the contract
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoggingConfig {
    pub debug_mode: bool,
    pub log_requests: bool,
    pub log_responses: bool,
    pub redact_sensitive: bool,
    pub max_log_size: u32,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            debug_mode: false,
            log_requests: true,
            log_responses: true,
            redact_sensitive: true,
            max_log_size: 1024,
        }
    }
}

/// Log levels for structured logging
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// Structured log entry
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub message: String,
    pub request_id: Option<RequestId>,
    pub operation: Option<String>,
    pub actor: Option<Address>,
    pub metadata: Option<String>, // JSON-encoded metadata
}

/// Request/Response logging data
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestLog {
    pub request_id: RequestId,
    pub method: String,
    pub endpoint: String,
    pub timestamp: u64,
    pub duration_ms: Option<u64>,
    pub status: Option<String>,
    pub payload_size: u32,
    pub redacted_payload: Option<String>,
}

/// Sensitive data patterns to redact
const SENSITIVE_PATTERNS: &[&str] = &[
    "password",
    "secret",
    "key",
    "token",
    "auth",
    "credential",
    "private",
    "seed",
    "mnemonic",
];

/// Main logging interface
pub struct Logger;

impl Logger {
    /// Log a structured message
    pub fn log(
        env: &Env,
        level: LogLevel,
        message: String,
        request_id: Option<RequestId>,
        operation: Option<String>,
        actor: Option<Address>,
        metadata: Option<String>,
    ) {
        let config = Self::get_config(env);
        
        // Skip debug/trace logs if debug mode is disabled
        if !config.debug_mode && matches!(level, LogLevel::Debug | LogLevel::Trace) {
            return;
        }

        let entry = LogEntry {
            timestamp: env.ledger().timestamp(),
            level,
            message,
            request_id,
            operation,
            actor,
            metadata,
        };

        // Publish as Soroban event
        env.events().publish(
            (symbol_short!("log"), symbol_short!("entry")),
            entry,
        );
    }

    /// Log an error with context
    pub fn error(env: &Env, message: String, request_id: Option<RequestId>, error: Option<Error>) {
        let metadata = error.map(|e| format!("{{\"error_code\":{}}}", e as u32));
        Self::log(env, LogLevel::Error, message, request_id, None, None, metadata);
    }

    /// Log a warning
    pub fn warn(env: &Env, message: String, request_id: Option<RequestId>) {
        Self::log(env, LogLevel::Warn, message, request_id, None, None, None);
    }

    /// Log an info message
    pub fn info(env: &Env, message: String, request_id: Option<RequestId>) {
        Self::log(env, LogLevel::Info, message, request_id, None, None, None);
    }

    /// Log a debug message (only if debug mode enabled)
    pub fn debug(env: &Env, message: String, request_id: Option<RequestId>) {
        Self::log(env, LogLevel::Debug, message, request_id, None, None, None);
    }

    /// Log a trace message (only if debug mode enabled)
    pub fn trace(env: &Env, message: String, request_id: Option<RequestId>) {
        Self::log(env, LogLevel::Trace, message, request_id, None, None, None);
    }

    /// Log operation start
    pub fn operation_start(
        env: &Env,
        operation: String,
        actor: Address,
        request_id: RequestId,
        params: Option<String>,
    ) {
        let message = format!("Operation started: {}", operation);
        Self::log(
            env,
            LogLevel::Info,
            message,
            Some(request_id),
            Some(operation),
            Some(actor),
            params,
        );
    }

    /// Log operation completion
    pub fn operation_complete(
        env: &Env,
        operation: String,
        actor: Address,
        request_id: RequestId,
        duration_ms: u64,
        success: bool,
    ) {
        let level = if success { LogLevel::Info } else { LogLevel::Error };
        let status = if success { "success" } else { "failed" };
        let message = format!("Operation {}: {} ({}ms)", status, operation, duration_ms);
        let metadata = format!("{{\"duration_ms\":{},\"success\":{}}}", duration_ms, success);
        
        Self::log(
            env,
            level,
            message,
            Some(request_id),
            Some(operation),
            Some(actor),
            Some(metadata),
        );
    }

    /// Log HTTP request
    pub fn log_request(
        env: &Env,
        request_id: RequestId,
        method: String,
        endpoint: String,
        payload: Option<Bytes>,
    ) {
        let config = Self::get_config(env);
        if !config.log_requests {
            return;
        }

        let payload_size = payload.as_ref().map(|p| p.len()).unwrap_or(0);
        let redacted_payload = if config.redact_sensitive {
            payload.map(|p| Self::redact_sensitive_data(env, p))
        } else {
            payload.map(|p| Self::bytes_to_string(env, p))
        };

        let log = RequestLog {
            request_id,
            method,
            endpoint,
            timestamp: env.ledger().timestamp(),
            duration_ms: None,
            status: None,
            payload_size,
            redacted_payload,
        };

        env.events().publish(
            (symbol_short!("http"), symbol_short!("request")),
            log,
        );
    }

    /// Log HTTP response
    pub fn log_response(
        env: &Env,
        request_id: RequestId,
        status: String,
        duration_ms: u64,
        payload: Option<Bytes>,
    ) {
        let config = Self::get_config(env);
        if !config.log_responses {
            return;
        }

        let payload_size = payload.as_ref().map(|p| p.len()).unwrap_or(0);
        let redacted_payload = if config.redact_sensitive {
            payload.map(|p| Self::redact_sensitive_data(env, p))
        } else {
            payload.map(|p| Self::bytes_to_string(env, p))
        };

        let log = RequestLog {
            request_id,
            method: String::from_str(env, "RESPONSE"),
            endpoint: String::from_str(env, ""),
            timestamp: env.ledger().timestamp(),
            duration_ms: Some(duration_ms),
            status: Some(status),
            payload_size,
            redacted_payload,
        };

        env.events().publish(
            (symbol_short!("http"), symbol_short!("response")),
            log,
        );
    }

    /// Get current logging configuration
    fn get_config(env: &Env) -> LoggingConfig {
        // Try to get from storage, fallback to default
        env.storage()
            .persistent()
            .get(&symbol_short!("log_cfg"))
            .unwrap_or_else(|| LoggingConfig::default())
    }

    /// Update logging configuration
    pub fn set_config(env: &Env, config: LoggingConfig) {
        env.storage()
            .persistent()
            .set(&symbol_short!("log_cfg"), &config);
        
        Self::info(env, String::from_str(env, "Logging configuration updated"), None);
    }

    /// Redact sensitive data from payload
    fn redact_sensitive_data(env: &Env, payload: Bytes) -> String {
        let payload_str = Self::bytes_to_string(env, payload);
        let mut redacted = payload_str;

        // Simple pattern-based redaction
        for pattern in SENSITIVE_PATTERNS {
            let pattern_str = String::from_str(env, pattern);
            // This is a simplified redaction - in practice you'd use regex
            // For now, just replace the pattern with [REDACTED]
            if redacted.contains(&pattern_str) {
                redacted = String::from_str(env, "[REDACTED]");
                break;
            }
        }

        redacted
    }

    /// Convert bytes to string (truncated if too long)
    fn bytes_to_string(env: &Env, bytes: Bytes) -> String {
        let config = Self::get_config(env);
        let max_len = config.max_log_size as usize;
        
        if bytes.len() > max_len {
            let truncated = bytes.slice(0, max_len);
            let mut result = String::from_bytes(env, &truncated);
            result.push_str(&String::from_str(env, "...[TRUNCATED]"));
            result
        } else {
            String::from_bytes(env, &bytes)
        }
    }
}

/// Convenience macros for logging (if Rust macros were supported in Soroban)
/// These would be implemented as regular functions for now

pub fn log_error(env: &Env, msg: &str, request_id: Option<RequestId>) {
    Logger::error(env, String::from_str(env, msg), request_id, None);
}

pub fn log_warn(env: &Env, msg: &str, request_id: Option<RequestId>) {
    Logger::warn(env, String::from_str(env, msg), request_id);
}

pub fn log_info(env: &Env, msg: &str, request_id: Option<RequestId>) {
    Logger::info(env, String::from_str(env, msg), request_id);
}

pub fn log_debug(env: &Env, msg: &str, request_id: Option<RequestId>) {
    Logger::debug(env, String::from_str(env, msg), request_id);
}

pub fn log_trace(env: &Env, msg: &str, request_id: Option<RequestId>) {
    Logger::trace(env, String::from_str(env, msg), request_id);
}