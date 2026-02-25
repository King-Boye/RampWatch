extern crate alloc;

use crate::types::{HealthStatus, QuoteData};
use crate::Error;
use crate::logging::Logger;
use crate::request_id::RequestId;
use soroban_sdk::{Bytes, Env, String};

/// Transport request types
#[derive(Clone, Debug, PartialEq)]
pub enum TransportRequest {
    GetQuote {
        endpoint: String,
        base_asset: String,
        quote_asset: String,
        amount: u64,
    },
    SubmitAttestation {
        endpoint: String,
        payload: Bytes,
    },
    CheckHealth {
        endpoint: String,
    },
    VerifyKYC {
        endpoint: String,
        subject_id: String,
    },
}

/// Transport response types
#[derive(Clone, Debug, PartialEq)]
pub enum TransportResponse {
    Quote(QuoteData),
    AttestationConfirmed { transaction_id: String },
    Health(HealthStatus),
    KYCVerified { status: String, level: String },
    Error { code: u32, message: String },
}

/// Transport abstraction trait for anchor communication
/// This allows for both real HTTP implementations and mock implementations for testing
pub trait AnchorTransport {
    /// Send a request to an anchor and receive a response
    fn send_request(
        &mut self,
        env: &Env,
        request: TransportRequest,
    ) -> Result<TransportResponse, Error>;

    /// Send a request with logging support
    fn send_request_with_logging(
        &mut self,
        env: &Env,
        request: TransportRequest,
        request_id: RequestId,
    ) -> Result<TransportResponse, Error> {
        let start_time = env.ledger().timestamp();
        
        // Log the request
        let (method, endpoint, payload) = match &request {
            TransportRequest::GetQuote { endpoint, base_asset, quote_asset, amount } => {
                let method = String::from_str(env, "GET_QUOTE");
                let payload_str = format!("{{\"base_asset\":\"{}\",\"quote_asset\":\"{}\",\"amount\":{}}}", 
                    base_asset, quote_asset, amount);
                let payload = Bytes::from_slice(env, payload_str.as_bytes());
                (method, endpoint.clone(), Some(payload))
            },
            TransportRequest::SubmitAttestation { endpoint, payload } => {
                (String::from_str(env, "SUBMIT_ATTESTATION"), endpoint.clone(), Some(payload.clone()))
            },
            TransportRequest::CheckHealth { endpoint } => {
                (String::from_str(env, "CHECK_HEALTH"), endpoint.clone(), None)
            },
            TransportRequest::VerifyKYC { endpoint, subject_id } => {
                let method = String::from_str(env, "VERIFY_KYC");
                let payload_str = format!("{{\"subject_id\":\"{}\"}}", subject_id);
                let payload = Bytes::from_slice(env, payload_str.as_bytes());
                (method, endpoint.clone(), Some(payload))
            },
        };

        Logger::log_request(env, request_id, method, endpoint, payload);

        // Execute the request
        let result = self.send_request(env, request);
        
        // Calculate duration and log response
        let end_time = env.ledger().timestamp();
        let duration_ms = (end_time - start_time) * 1000; // Convert to milliseconds

        match &result {
            Ok(response) => {
                let (status, response_payload) = match response {
                    TransportResponse::Quote(quote) => {
                        let status = String::from_str(env, "200_OK");
                        let payload_str = format!("{{\"rate\":\"{}\",\"expires_at\":{}}}", 
                            quote.rate, quote.expires_at);
                        let payload = Bytes::from_slice(env, payload_str.as_bytes());
                        (status, Some(payload))
                    },
                    TransportResponse::AttestationConfirmed { transaction_id } => {
                        let status = String::from_str(env, "200_OK");
                        let payload_str = format!("{{\"transaction_id\":\"{}\"}}", transaction_id);
                        let payload = Bytes::from_slice(env, payload_str.as_bytes());
                        (status, Some(payload))
                    },
                    TransportResponse::Health(health) => {
                        let status = String::from_str(env, "200_OK");
                        let status_str = match health {
                            HealthStatus::Healthy => "healthy",
                            HealthStatus::Degraded => "degraded",
                            HealthStatus::Unhealthy => "unhealthy",
                        };
                        let payload_str = format!("{{\"status\":\"{}\"}}", status_str);
                        let payload = Bytes::from_slice(env, payload_str.as_bytes());
                        (status, Some(payload))
                    },
                    TransportResponse::KYCVerified { status: kyc_status, level } => {
                        let status = String::from_str(env, "200_OK");
                        let payload_str = format!("{{\"status\":\"{}\",\"level\":\"{}\"}}", 
                            kyc_status, level);
                        let payload = Bytes::from_slice(env, payload_str.as_bytes());
                        (status, Some(payload))
                    },
                    TransportResponse::Error { code, message } => {
                        let status = format!("{}_ERROR", code);
                        let status_str = String::from_str(env, &status);
                        let payload_str = format!("{{\"error\":\"{}\"}}", message);
                        let payload = Bytes::from_slice(env, payload_str.as_bytes());
                        (status_str, Some(payload))
                    },
                };
                Logger::log_response(env, request_id, status, duration_ms, response_payload);
            },
            Err(error) => {
                let status = format!("ERROR_{}", *error as u32);
                let status_str = String::from_str(env, &status);
                Logger::log_response(env, request_id, status_str, duration_ms, None);
            }
        }

        result
    }

    /// Check if the transport is available
    fn is_available(&self) -> bool;

    /// Get transport name for debugging
    fn name(&self) -> &str;
}

/// Mock transport implementation for deterministic testing
/// Allows pre-configured responses without actual HTTP calls
pub struct MockTransport {
    responses: alloc::vec::Vec<(TransportRequest, TransportResponse)>,
    call_count: u32,
    should_fail: bool,
}

impl MockTransport {
    /// Create a new mock transport
    pub fn new() -> Self {
        Self {
            responses: alloc::vec::Vec::new(),
            call_count: 0,
            should_fail: false,
        }
    }

    /// Add a mock response for a specific request
    pub fn add_response(&mut self, request: TransportRequest, response: TransportResponse) {
        self.responses.push((request, response));
    }

    /// Configure the transport to fail all requests
    pub fn set_should_fail(&mut self, should_fail: bool) {
        self.should_fail = should_fail;
    }

    /// Get the number of requests made
    pub fn get_call_count(&self) -> u32 {
        self.call_count
    }

    /// Reset the mock transport state
    pub fn reset(&mut self) {
        self.responses.clear();
        self.call_count = 0;
        self.should_fail = false;
    }

    /// Find matching response for a request
    fn find_response(&self, request: &TransportRequest) -> Option<TransportResponse> {
        for (req, resp) in &self.responses {
            if Self::requests_match(req, request) {
                return Some(resp.clone());
            }
        }
        None
    }

    /// Check if two requests match (for mock lookup)
    fn requests_match(a: &TransportRequest, b: &TransportRequest) -> bool {
        match (a, b) {
            (
                TransportRequest::GetQuote {
                    endpoint: e1,
                    base_asset: b1,
                    quote_asset: q1,
                    amount: a1,
                },
                TransportRequest::GetQuote {
                    endpoint: e2,
                    base_asset: b2,
                    quote_asset: q2,
                    amount: a2,
                },
            ) => e1 == e2 && b1 == b2 && q1 == q2 && a1 == a2,
            (
                TransportRequest::SubmitAttestation {
                    endpoint: e1,
                    payload: p1,
                },
                TransportRequest::SubmitAttestation {
                    endpoint: e2,
                    payload: p2,
                },
            ) => e1 == e2 && p1 == p2,
            (
                TransportRequest::CheckHealth { endpoint: e1 },
                TransportRequest::CheckHealth { endpoint: e2 },
            ) => e1 == e2,
            (
                TransportRequest::VerifyKYC {
                    endpoint: e1,
                    subject_id: s1,
                },
                TransportRequest::VerifyKYC {
                    endpoint: e2,
                    subject_id: s2,
                },
            ) => e1 == e2 && s1 == s2,
            _ => false,
        }
    }
}

impl AnchorTransport for MockTransport {
    fn send_request(
        &mut self,
        _env: &Env,
        request: TransportRequest,
    ) -> Result<TransportResponse, Error> {
        self.call_count += 1;

        if self.should_fail {
            return Err(Error::EndpointNotFound);
        }

        match self.find_response(&request) {
            Some(response) => Ok(response),
            None => Err(Error::EndpointNotFound),
        }
    }

    fn is_available(&self) -> bool {
        !self.should_fail
    }

    fn name(&self) -> &str {
        "MockTransport"
    }
}

impl Default for MockTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String as SorobanString};

    #[test]
    fn test_mock_transport_creation() {
        let transport = MockTransport::new();
        assert_eq!(transport.get_call_count(), 0);
        assert!(transport.is_available());
        assert_eq!(transport.name(), "MockTransport");
    }

    #[test]
    fn test_mock_transport_add_response() {
        let env = Env::default();
        let mut transport = MockTransport::new();

        let endpoint = SorobanString::from_str(&env, "https://anchor.example.com");
        let base_asset = SorobanString::from_str(&env, "USD");
        let quote_asset = SorobanString::from_str(&env, "USDC");

        let request = TransportRequest::GetQuote {
            endpoint: endpoint.clone(),
            base_asset: base_asset.clone(),
            quote_asset: quote_asset.clone(),
            amount: 1000,
        };

        let anchor = Address::generate(&env);
        let quote = QuoteData {
            anchor: anchor.clone(),
            base_asset: base_asset.clone(),
            quote_asset: quote_asset.clone(),
            rate: 10000,
            fee_percentage: 25,
            minimum_amount: 100,
            maximum_amount: 10000,
            valid_until: 1000000,
            quote_id: 1,
        };

        let response = TransportResponse::Quote(quote.clone());
        transport.add_response(request.clone(), response);

        let result = transport.send_request(&env, request);
        assert!(result.is_ok());
        assert_eq!(transport.get_call_count(), 1);

        if let Ok(TransportResponse::Quote(returned_quote)) = result {
            assert_eq!(returned_quote.anchor, anchor);
            assert_eq!(returned_quote.rate, 10000);
        } else {
            panic!("Expected Quote response");
        }
    }

    #[test]
    fn test_mock_transport_not_found() {
        let env = Env::default();
        let mut transport = MockTransport::new();

        let endpoint = SorobanString::from_str(&env, "https://anchor.example.com");
        let base_asset = SorobanString::from_str(&env, "USD");
        let quote_asset = SorobanString::from_str(&env, "USDC");

        let request = TransportRequest::GetQuote {
            endpoint,
            base_asset,
            quote_asset,
            amount: 1000,
        };

        let result = transport.send_request(&env, request);
        assert_eq!(result, Err(Error::EndpointNotFound));
        assert_eq!(transport.get_call_count(), 1);
    }

    #[test]
    fn test_mock_transport_should_fail() {
        let env = Env::default();
        let mut transport = MockTransport::new();
        transport.set_should_fail(true);

        assert!(!transport.is_available());

        let endpoint = SorobanString::from_str(&env, "https://anchor.example.com");
        let request = TransportRequest::CheckHealth { endpoint };

        let result = transport.send_request(&env, request);
        assert_eq!(result, Err(Error::EndpointNotFound));
    }

    #[test]
    fn test_mock_transport_reset() {
        let env = Env::default();
        let mut transport = MockTransport::new();

        let endpoint = SorobanString::from_str(&env, "https://anchor.example.com");
        let request = TransportRequest::CheckHealth {
            endpoint: endpoint.clone(),
        };

        let anchor = Address::generate(&env);
        let health = HealthStatus {
            anchor,
            latency_ms: 50,
            failure_count: 0,
            availability_percent: 9999,
            last_check: 1000,
        };

        transport.add_response(request.clone(), TransportResponse::Health(health));
        let _ = transport.send_request(&env, request.clone());

        assert_eq!(transport.get_call_count(), 1);

        transport.reset();
        assert_eq!(transport.get_call_count(), 0);

        let result = transport.send_request(&env, request);
        assert_eq!(result, Err(Error::EndpointNotFound));
    }

    #[test]
    fn test_mock_transport_multiple_requests() {
        let env = Env::default();
        let mut transport = MockTransport::new();

        let endpoint = SorobanString::from_str(&env, "https://anchor.example.com");
        let anchor = Address::generate(&env);

        // Add health check response
        let health_request = TransportRequest::CheckHealth {
            endpoint: endpoint.clone(),
        };
        let health = HealthStatus {
            anchor: anchor.clone(),
            latency_ms: 50,
            failure_count: 0,
            availability_percent: 9999,
            last_check: 1000,
        };
        transport.add_response(health_request.clone(), TransportResponse::Health(health));

        // Add KYC verification response
        let kyc_request = TransportRequest::VerifyKYC {
            endpoint: endpoint.clone(),
            subject_id: SorobanString::from_str(&env, "user123"),
        };
        transport.add_response(
            kyc_request.clone(),
            TransportResponse::KYCVerified {
                status: SorobanString::from_str(&env, "approved"),
                level: SorobanString::from_str(&env, "advanced"),
            },
        );

        // Make requests
        let health_result = transport.send_request(&env, health_request);
        assert!(health_result.is_ok());
        assert_eq!(transport.get_call_count(), 1);

        let kyc_result = transport.send_request(&env, kyc_request);
        assert!(kyc_result.is_ok());
        assert_eq!(transport.get_call_count(), 2);
    }
}
