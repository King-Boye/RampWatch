use soroban_sdk::contracttype;

/// Standardized response wrapper for query operations.
/// Provides consistent structure for off-chain integrations.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Response<T: Clone> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<u32>,
}

impl<T: Clone> Response<T> {
    pub fn ok(data: T) -> Self {
        Response {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(error_code: u32) -> Self {
        Response {
            success: false,
            data: None,
            error: Some(error_code),
        }
    }
}
