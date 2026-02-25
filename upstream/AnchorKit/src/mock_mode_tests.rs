#[cfg(test)]
mod tests {
    use super::super::mock_mode::{MockAnchor, MockWebhook};
    use crate::anchor_adapter::AnchorAdapter;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_mock_deposit() {
        let env = Env::default();
        let mock = MockAnchor::new();
        let anchor = Address::generate(&env);
        
        let auth = mock.authenticate(&env, &anchor, &Default::default());
        assert_eq!(auth.token.to_string(), "mock_token_12345");
    }

    #[test]
    fn test_mock_webhook() {
        let webhook = MockWebhook::new();
        webhook.trigger("deposit.completed");
        
        let triggers = webhook.get_triggers();
        assert_eq!(triggers.len(), 1);
        assert_eq!(triggers[0], "deposit.completed");
    }
}
