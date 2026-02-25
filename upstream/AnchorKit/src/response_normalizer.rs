use crate::anchor_adapter::{DepositResponse, WithdrawResponse};
use crate::errors::Error;
use crate::types::QuoteData;
use soroban_sdk::{contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct NormalizedResponse {
    pub status: String,
    pub amount: u64,
    pub asset: String,
    pub fee: u64,
    pub id: String,
}

pub struct ResponseNormalizer;

impl ResponseNormalizer {
    pub fn normalize_deposit(
        env: &Env,
        response: &DepositResponse,
        amount: u64,
        asset: String,
        fee: u64,
    ) -> NormalizedResponse {
        NormalizedResponse {
            status: response.status.clone(),
            amount,
            asset,
            fee,
            id: response.transaction_id.clone(),
        }
    }

    pub fn normalize_withdraw(
        env: &Env,
        response: &WithdrawResponse,
        amount: u64,
        asset: String,
        fee: u64,
    ) -> NormalizedResponse {
        NormalizedResponse {
            status: response.status.clone(),
            amount,
            asset,
            fee,
            id: response.transaction_id.clone(),
        }
    }

    pub fn normalize_quote(
        env: &Env,
        quote: &QuoteData,
        amount: u64,
        id_prefix: String,
    ) -> NormalizedResponse {
        let fee = Self::calculate_fee(amount, quote.fee_percentage);

        NormalizedResponse {
            status: String::from_str(env, "quoted"),
            amount,
            asset: quote.quote_asset.clone(),
            fee,
            id: id_prefix,
        }
    }

    fn calculate_fee(amount: u64, fee_percentage: u32) -> u64 {
        ((amount as u128 * fee_percentage as u128) / 10000) as u64
    }

    pub fn validate(response: &NormalizedResponse) -> Result<(), Error> {
        if response.status.is_empty() {
            return Err(Error::ProtocolInvalidPayload);
        }
        if response.asset.is_empty() {
            return Err(Error::UnsupportedAsset);
        }
        if response.id.is_empty() {
            return Err(Error::ProtocolInvalidPayload);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::anchor_adapter::{DepositResponse, WithdrawResponse};
    use crate::types::QuoteData;
    use soroban_sdk::{testutils::Address as _, Env, String};

    #[test]
    fn test_normalize_deposit() {
        let env = Env::default();
        let deposit = DepositResponse {
            transaction_id: String::from_str(&env, "dep_123"),
            status: String::from_str(&env, "pending"),
            deposit_address: String::from_str(&env, "GDEPOSIT..."),
            expires_at: 1000,
        };

        let normalized = ResponseNormalizer::normalize_deposit(
            &env,
            &deposit,
            100_0000000,
            String::from_str(&env, "USDC"),
            1_0000000,
        );

        assert_eq!(normalized.status, String::from_str(&env, "pending"));
        assert_eq!(normalized.amount, 100_0000000);
        assert_eq!(normalized.asset, String::from_str(&env, "USDC"));
        assert_eq!(normalized.fee, 1_0000000);
        assert_eq!(normalized.id, String::from_str(&env, "dep_123"));
    }

    #[test]
    fn test_normalize_withdraw() {
        let env = Env::default();
        let withdraw = WithdrawResponse {
            transaction_id: String::from_str(&env, "wd_456"),
            status: String::from_str(&env, "processing"),
            estimated_completion: 2000,
        };

        let normalized = ResponseNormalizer::normalize_withdraw(
            &env,
            &withdraw,
            50_0000000,
            String::from_str(&env, "USDC"),
            500000,
        );

        assert_eq!(normalized.status, String::from_str(&env, "processing"));
        assert_eq!(normalized.amount, 50_0000000);
        assert_eq!(normalized.asset, String::from_str(&env, "USDC"));
        assert_eq!(normalized.fee, 500000);
        assert_eq!(normalized.id, String::from_str(&env, "wd_456"));
    }

    #[test]
    fn test_normalize_quote() {
        let env = Env::default();
        let anchor = Address::generate(&env);
        let quote = QuoteData {
            anchor: anchor.clone(),
            base_asset: String::from_str(&env, "USD"),
            quote_asset: String::from_str(&env, "USDC"),
            rate: 10000,
            fee_percentage: 50,
            minimum_amount: 1_0000000,
            maximum_amount: 1000_0000000,
            valid_until: 3000,
            quote_id: 789,
        };

        let normalized = ResponseNormalizer::normalize_quote(
            &env,
            &quote,
            100_0000000,
            String::from_str(&env, "quote_789"),
        );

        assert_eq!(normalized.status, String::from_str(&env, "quoted"));
        assert_eq!(normalized.amount, 100_0000000);
        assert_eq!(normalized.asset, String::from_str(&env, "USDC"));
        assert_eq!(normalized.fee, 5000000); // 50 basis points = 0.5% of 1000000000
        assert_eq!(normalized.id, String::from_str(&env, "quote_789"));
    }

    #[test]
    fn test_calculate_fee() {
        // fee_percentage is in basis points (1/10000)
        // 50 basis points = 0.5%
        assert_eq!(ResponseNormalizer::calculate_fee(100_0000000, 50), 5000000);
        // 100 basis points = 1%
        assert_eq!(
            ResponseNormalizer::calculate_fee(100_0000000, 100),
            10_000000
        );
        assert_eq!(ResponseNormalizer::calculate_fee(100_0000000, 0), 0);
    }

    #[test]
    fn test_validate_success() {
        let env = Env::default();
        let response = NormalizedResponse {
            status: String::from_str(&env, "pending"),
            amount: 100,
            asset: String::from_str(&env, "USDC"),
            fee: 1,
            id: String::from_str(&env, "tx_123"),
        };

        assert!(ResponseNormalizer::validate(&response).is_ok());
    }

    #[test]
    fn test_validate_empty_status() {
        let env = Env::default();
        let response = NormalizedResponse {
            status: String::from_str(&env, ""),
            amount: 100,
            asset: String::from_str(&env, "USDC"),
            fee: 1,
            id: String::from_str(&env, "tx_123"),
        };

        assert_eq!(
            ResponseNormalizer::validate(&response),
            Err(Error::ProtocolInvalidPayload)
        );
    }

    #[test]
    fn test_validate_empty_asset() {
        let env = Env::default();
        let response = NormalizedResponse {
            status: String::from_str(&env, "pending"),
            amount: 100,
            asset: String::from_str(&env, ""),
            fee: 1,
            id: String::from_str(&env, "tx_123"),
        };

        assert_eq!(
            ResponseNormalizer::validate(&response),
            Err(Error::UnsupportedAsset)
        );
    }

    #[test]
    fn test_validate_empty_id() {
        let env = Env::default();
        let response = NormalizedResponse {
            status: String::from_str(&env, "pending"),
            amount: 100,
            asset: String::from_str(&env, "USDC"),
            fee: 1,
            id: String::from_str(&env, ""),
        };

        assert_eq!(
            ResponseNormalizer::validate(&response),
            Err(Error::ProtocolInvalidPayload)
        );
    }
}
