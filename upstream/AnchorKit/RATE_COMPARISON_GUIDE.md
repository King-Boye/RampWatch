# AnchorKit Rate Comparison Guide

## Overview

AnchorKit now supports rate comparison functionality that allows you to compare rates across multiple anchors and automatically select the best option. This feature enables efficient price discovery and optimal routing for cross-border payments, stablecoin operations, and fiat on/off-ramp services.

## Key Features

- **Quote Submission**: Anchors can submit real-time quotes with rates, fees, and validity periods
- **Rate Comparison**: Compare rates across multiple anchors for a given asset pair
- **Best Rate Selection**: Automatically identify the most cost-effective option including fees
- **Time-based Validity**: Quotes expire automatically to prevent stale rate usage
- **Audit Trail**: Full traceability of rate submissions and selections

## Core Data Structures

### QuoteData
```rust
pub struct QuoteData {
    pub anchor: Address,           // Anchor providing the quote
    pub base_asset: String,        // Source asset (e.g., "USD")
    pub quote_asset: String,       // Target asset (e.g., "USDC")
    pub rate: u64,                 // Exchange rate in basis points (10000 = 1.0)
    pub fee_percentage: u32,       // Fee in basis points
    pub minimum_amount: u64,       // Minimum transaction amount
    pub maximum_amount: u64,       // Maximum transaction amount
    pub valid_until: u64,          // Unix timestamp when quote expires
    pub quote_id: u64,            // Unique quote identifier
}
```

### QuoteRequest
```rust
pub struct QuoteRequest {
    pub base_asset: String,        // Source asset
    pub quote_asset: String,       // Target asset
    pub amount: u64,              // Transaction amount
    pub operation_type: ServiceType, // Deposits or Withdrawals
}
```

### RateComparison
```rust
pub struct RateComparison {
    pub best_quote: QuoteData,     // The most cost-effective quote
    pub all_quotes: Vec<QuoteData>, // All valid quotes for comparison
    pub comparison_timestamp: u64,  // When the comparison was performed
}
```

## Usage Examples

### 1. Submit a Quote (Anchor)

```rust
// Anchor submits a USD to USDC quote
let quote_id = contract.submit_quote(
    anchor_address,
    String::from_str(&env, "USD"),      // base_asset
    String::from_str(&env, "USDC"),     // quote_asset
    10050,                              // rate: 1.005 (0.5% markup)
    25,                                 // fee: 0.25%
    100_000000,                         // min: $100
    10000_000000,                       // max: $10,000
    env.ledger().timestamp() + 300,     // valid for 5 minutes
)?;
```

### 2. Compare Rates Across Anchors

```rust
// Create a quote request
let request = QuoteRequest {
    base_asset: String::from_str(&env, "USD"),
    quote_asset: String::from_str(&env, "USDC"),
    amount: 1000_000000, // $1,000
    operation_type: ServiceType::Deposits,
};

// Compare rates from specific anchors
let mut anchors = Vec::new(&env);
anchors.push_back(remittance_anchor);
anchors.push_back(fiat_ramp_anchor);
anchors.push_back(stablecoin_issuer);

let comparison = contract.compare_rates_for_anchors(request, anchors)?;

// Access the best quote
let best_anchor = comparison.best_quote.anchor;
let best_rate = comparison.best_quote.rate;
let total_cost = calculate_total_cost(&comparison.best_quote, 1000_000000);
```

### 3. Get Specific Quote

```rust
// Retrieve a specific quote by anchor and quote ID
let quote = contract.get_quote(anchor_address, quote_id)?;

// Check if quote is still valid
if quote.valid_until > env.ledger().timestamp() {
    // Quote is valid, can be used
    process_transaction(&quote);
}
```

## Rate Calculation

The system calculates effective rates including fees:

```rust
fn calculate_effective_rate(quote: &QuoteData, amount: u64) -> u64 {
    let base_rate = quote.rate;
    let fee_amount = (amount * quote.fee_percentage as u64) / 10000;
    let effective_amount = amount + fee_amount;
    
    // Return effective rate per unit
    (base_rate * effective_amount) / amount
}
```

## Integration Patterns

### 1. Real-time Rate Feeds

Anchors can integrate with external price feeds to submit updated quotes:

```rust
// Pseudo-code for rate feed integration
async fn update_rates_from_feed() {
    let market_rate = fetch_market_rate("USD", "USDC").await?;
    let markup = 0.005; // 0.5% markup
    let rate_with_markup = (market_rate * (1.0 + markup) * 10000.0) as u64;
    
    contract.submit_quote(
        anchor_address,
        "USD".to_string(),
        "USDC".to_string(),
        rate_with_markup,
        25, // 0.25% fee
        min_amount,
        max_amount,
        current_time + 60, // 1 minute validity
    )?;
}
```

### 2. Best Rate Selection

Applications can automatically select the best rate:

```rust
fn find_best_rate_for_user(amount: u64) -> Result<Address, Error> {
    let request = QuoteRequest {
        base_asset: String::from_str(&env, "USD"),
        quote_asset: String::from_str(&env, "USDC"),
        amount,
        operation_type: ServiceType::Deposits,
    };
    
    let comparison = contract.compare_rates_for_anchors(request, all_anchors)?;
    
    // Return the anchor with the best rate
    Ok(comparison.best_quote.anchor)
}
```

### 3. Rate History and Analytics

Track rate performance over time:

```rust
fn analyze_anchor_performance(anchor: Address, days: u32) -> RateAnalytics {
    // Query historical quotes and calculate metrics
    // - Average spread
    // - Rate stability
    // - Availability percentage
    // - Volume handled
}
```

## Error Handling

The rate comparison system includes comprehensive error handling:

- `InvalidQuote`: Quote parameters are malformed or invalid
- `StaleQuote`: Quote has expired
- `NoQuotesAvailable`: No valid quotes found for the request
- `QuoteNotFound`: Specific quote ID doesn't exist
- `UnauthorizedAttestor`: Only registered anchors can submit quotes
- `ServicesNotConfigured`: Anchor hasn't configured quote services

## Security Considerations

1. **Rate Validation**: All quotes are validated for reasonable parameters
2. **Expiry Enforcement**: Stale quotes are automatically rejected
3. **Anchor Authorization**: Only registered attestors can submit quotes
4. **Replay Protection**: Prevents duplicate quote submissions
5. **Audit Trail**: All rate operations are logged for compliance

## Performance Optimization

1. **TTL-based Cleanup**: Expired quotes are cleaned up automatically
2. **Efficient Storage**: Quotes use persistent storage with appropriate TTL
3. **Batch Comparison**: Multiple anchors can be compared in a single call
4. **Event Emission**: Minimal gas usage for quote submission events

## Compliance and Auditability

The system provides full traceability:

- All quote submissions are logged with events
- Rate comparisons include timestamps
- Session tracking for complex workflows
- Audit logs for regulatory compliance

## Next Steps

1. **Enhanced Quote Discovery**: Implement anchor registry iteration
2. **Rate Aggregation**: Support for weighted average rates
3. **Slippage Protection**: Automatic rate tolerance checking
4. **Volume-based Pricing**: Tiered rates based on transaction size
5. **Cross-chain Support**: Multi-network rate comparison

This rate comparison system provides a solid foundation for building efficient, transparent, and competitive anchor services within the AnchorKit ecosystem.