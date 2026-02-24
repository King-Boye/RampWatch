use soroban_sdk::{contracttype, Address, String, Vec};

/// Skeleton loader state for anchor information
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnchorInfoSkeleton {
    pub anchor: Address,
    pub is_loading: bool,
    pub has_error: bool,
    pub error_message: Option<String>,
}

impl AnchorInfoSkeleton {
    pub fn loading(anchor: Address) -> Self {
        Self {
            anchor,
            is_loading: true,
            has_error: false,
            error_message: None,
        }
    }

    pub fn loaded(anchor: Address) -> Self {
        Self {
            anchor,
            is_loading: false,
            has_error: false,
            error_message: None,
        }
    }

    pub fn error(anchor: Address, message: String) -> Self {
        Self {
            anchor,
            is_loading: false,
            has_error: true,
            error_message: Some(message),
        }
    }
}

/// Skeleton loader state for transaction status
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionStatusSkeleton {
    pub transaction_id: u64,
    pub is_loading: bool,
    pub has_error: bool,
    pub error_message: Option<String>,
    pub progress_percentage: u32, // 0-10000 (100.00%)
}

impl TransactionStatusSkeleton {
    pub fn loading(transaction_id: u64) -> Self {
        Self {
            transaction_id,
            is_loading: true,
            has_error: false,
            error_message: None,
            progress_percentage: 0,
        }
    }

    pub fn loading_with_progress(transaction_id: u64, progress: u32) -> Self {
        Self {
            transaction_id,
            is_loading: true,
            has_error: false,
            error_message: None,
            progress_percentage: progress,
        }
    }

    pub fn loaded(transaction_id: u64) -> Self {
        Self {
            transaction_id,
            is_loading: false,
            has_error: false,
            error_message: None,
            progress_percentage: 10000,
        }
    }

    pub fn error(transaction_id: u64, message: String) -> Self {
        Self {
            transaction_id,
            is_loading: false,
            has_error: true,
            error_message: Some(message),
            progress_percentage: 0,
        }
    }
}

/// Skeleton loader state for authentication validation
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthValidationSkeleton {
    pub attestor: Address,
    pub is_validating: bool,
    pub is_valid: bool,
    pub has_error: bool,
    pub error_message: Option<String>,
    pub validation_steps: Vec<ValidationStep>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationStep {
    pub step_name: String,
    pub is_complete: bool,
    pub is_loading: bool,
}

impl AuthValidationSkeleton {
    pub fn validating(env: &soroban_sdk::Env, attestor: Address) -> Self {
        Self {
            attestor,
            is_validating: true,
            is_valid: false,
            has_error: false,
            error_message: None,
            validation_steps: Vec::new(env),
        }
    }

    pub fn validating_with_steps(attestor: Address, steps: Vec<ValidationStep>) -> Self {
        Self {
            attestor,
            is_validating: true,
            is_valid: false,
            has_error: false,
            error_message: None,
            validation_steps: steps,
        }
    }

    pub fn validated(env: &soroban_sdk::Env, attestor: Address) -> Self {
        Self {
            attestor,
            is_validating: false,
            is_valid: true,
            has_error: false,
            error_message: None,
            validation_steps: Vec::new(env),
        }
    }

    pub fn error(env: &soroban_sdk::Env, attestor: Address, message: String) -> Self {
        Self {
            attestor,
            is_validating: false,
            is_valid: false,
            has_error: true,
            error_message: Some(message),
            validation_steps: Vec::new(env),
        }
    }
}

impl ValidationStep {
    pub fn new(step_name: String) -> Self {
        Self {
            step_name,
            is_complete: false,
            is_loading: true,
        }
    }

    pub fn complete(step_name: String) -> Self {
        Self {
            step_name,
            is_complete: true,
            is_loading: false,
        }
    }
}
