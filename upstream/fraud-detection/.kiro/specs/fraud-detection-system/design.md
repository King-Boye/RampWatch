# Design Document: Real-Time Fraud & Anomaly Detection System

## Overview

The Real-Time Fraud & Anomaly Detection System is a high-performance, scalable platform for identifying fraudulent payment transactions in real-time. The system processes incoming transactions through a multi-stage pipeline: ingestion, feature engineering, model scoring, decision classification, and alerting. It combines unsupervised anomaly detection with supervised classification to catch both known and novel fraud patterns.

The architecture prioritizes low latency (p95 < 100ms end-to-end), high throughput (1000+ tx/sec), and explainability. Every flagged transaction includes a human-readable explanation powered by SHAP values, enabling analysts to make informed decisions and provide feedback for continuous improvement.

Key design principles:
- **Real-time processing**: Sub-100ms latency for fraud scoring
- **Hybrid detection**: Combine unsupervised (Isolation Forest/Autoencoder) and supervised (XGBoost/LightGBM) models
- **Cost-aware decisions**: Optimize thresholds based on business costs of false positives and false negatives
- **Explainability-first**: Every decision includes interpretable explanations
- **Horizontal scalability**: Stateless services that scale independently

## Architecture

The system follows a microservices architecture with the following components:

```
┌─────────────────┐
│  Payment System │
└────────┬────────┘
         │ Transaction Events
         ▼
┌─────────────────────────────────────────────────────────┐
│              Ingestion Pipeline                          │
│  (Kafka Consumer / API Gateway)                         │
└────────┬────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────┐
│              Feature Engineering Service                 │
│  - Velocity Calculator                                   │
│  - Deviation Calculator                                  │
│  - Geo-Time Analyzer                                     │
└────────┬────────────────────────────────────────────────┘
         │ Features
         ▼
┌─────────────────────────────────────────────────────────┐
│              Model Service                               │
│  - Isolation Forest (unsupervised)                      │
│  - XGBoost/LightGBM (supervised)                        │
│  - Ensemble Scorer                                       │
└────────┬────────────────────────────────────────────────┘
         │ Fraud Score
         ▼
┌─────────────────────────────────────────────────────────┐
│              Decision Engine                             │
│  - Threshold Classifier                                  │
│  - Cost-Based Optimizer                                  │
└────────┬────────────────────────────────────────────────┘
         │ Decision (approve/review/block)
         ├──────────────────┬──────────────────┐
         ▼                  ▼                  ▼
┌──────────────┐  ┌──────────────────┐  ┌──────────────┐
│ Explainability│  │  Alert System    │  │   Database   │
│    Engine     │  │                  │  │  (Postgres)  │
└──────┬───────┘  └────────┬─────────┘  └──────────────┘
       │                   │
       └───────┬───────────┘
               ▼
       ┌──────────────────┐
       │  Analyst Dashboard│
       │   (Web UI)        │
       └──────────────────┘
```

### Data Flow

1. **Ingestion**: Transaction events arrive via Kafka stream or REST API
2. **Validation**: Parse and validate transaction schema
3. **Feature Engineering**: Compute velocity, deviation, frequency, and geo-time features
4. **Model Scoring**: Execute ensemble of models to generate fraud score
5. **Decision**: Classify transaction based on thresholds and cost matrix
6. **Explanation**: Generate SHAP-based explanation for flagged transactions
7. **Alert**: Notify analysts of high-risk transactions
8. **Storage**: Persist transaction, features, score, decision, and explanation

### Technology Stack

- **Ingestion**: Kafka (streaming), FastAPI (REST API)
- **Feature Store**: Redis (real-time), Postgres (historical)
- **Models**: scikit-learn (Isolation Forest), XGBoost/LightGBM
- **Explainability**: SHAP library
- **API**: FastAPI (Python)
- **Dashboard**: Streamlit or React
- **Database**: PostgreSQL
- **Deployment**: Docker, Kubernetes (optional for scaling)

## Components and Interfaces

### 1. Ingestion Pipeline

**Responsibilities:**
- Consume transaction events from Kafka or REST API
- Validate transaction schema and data types
- Enrich transactions with timestamps and request IDs
- Forward valid transactions to Feature Engineering Service

**Interface:**
```python
class Transaction:
    transaction_id: str
    user_id: str
    merchant_id: str
    amount: float
    currency: str
    timestamp: datetime
    device_id: str
    ip_address: str
    location: Optional[Location]
    
class Location:
    latitude: float
    longitude: float
    country: str

class IngestionPipeline:
    def validate_transaction(self, raw_data: dict) -> Transaction:
        """Validate and parse transaction data"""
        
    def process_stream(self) -> None:
        """Consume and process Kafka stream"""
```

**Performance Requirements:**
- Validation latency: < 10ms
- Throughput: 1000+ tx/sec
- Error handling: Log and reject invalid transactions

### 2. Feature Engineering Service

**Responsibilities:**
- Compute velocity features (transaction counts in 1m, 5m, 1h windows)
- Calculate amount deviation from user baseline
- Compute device and merchant frequency metrics
- Detect geo-time inconsistencies

**Interface:**
```python
class Features:
    # Velocity features
    tx_count_1m: int
    tx_count_5m: int
    tx_count_1h: int
    
    # Amount features
    amount_deviation_from_mean: float
    amount_deviation_from_median: float
    amount_percentile: float
    
    # Frequency features
    device_frequency: int
    merchant_frequency: int
    
    # Geo-time features
    geo_time_inconsistency_score: float
    distance_from_last_tx: float
    time_since_last_tx: float

class FeatureEngineeringService:
    def compute_features(self, transaction: Transaction) -> Features:
        """Compute all features for a transaction"""
        
    def get_user_baseline(self, user_id: str) -> UserBaseline:
        """Retrieve user's historical statistics"""
```

**Feature Computation Details:**

1. **Velocity Features**: Query Redis for recent transaction counts using sliding windows
2. **Amount Deviation**: Compare to user's mean/median from last 30 days (cached in Redis)
3. **Device/Merchant Frequency**: Count occurrences in last 24 hours
4. **Geo-Time Inconsistency**: Calculate if travel time between locations is physically impossible

**Performance Requirements:**
- Feature computation latency: < 20ms
- Cache hit rate: > 95% for user baselines

### 3. Model Service

**Responsibilities:**
- Execute unsupervised models (Isolation Forest or Autoencoder)
- Execute supervised models (XGBoost or LightGBM)
- Combine model scores using weighted ensemble
- Log predictions for monitoring

**Interface:**
```python
class ModelPrediction:
    fraud_score: float  # 0-1
    unsupervised_score: float
    supervised_score: float
    model_version: str
    
class ModelService:
    def predict(self, features: Features) -> ModelPrediction:
        """Generate fraud score from features"""
        
    def load_models(self, model_paths: dict) -> None:
        """Load trained models from disk"""
        
    def update_models(self, new_models: dict) -> None:
        """Hot-swap models without downtime"""
```

**Model Ensemble Strategy:**
- Unsupervised weight: 0.3 (catches novel patterns)
- Supervised weight: 0.7 (catches known patterns)
- Final score: `0.3 * unsupervised + 0.7 * supervised`

**Performance Requirements:**
- Prediction latency: < 50ms
- Model loading: < 5 seconds
- Support for A/B testing multiple model versions

### 4. Decision Engine

**Responsibilities:**
- Classify transactions as approve, review, or block
- Optimize thresholds based on cost matrix
- Support threshold experimentation and A/B testing

**Interface:**
```python
class Decision:
    action: str  # "approve", "review", "block"
    fraud_score: float
    threshold_used: float
    confidence: float

class CostMatrix:
    false_positive_cost: float  # Cost of blocking legitimate transaction
    false_negative_cost: float  # Cost of approving fraudulent transaction
    
class DecisionEngine:
    def classify(self, prediction: ModelPrediction) -> Decision:
        """Classify transaction based on fraud score"""
        
    def calibrate_thresholds(self, cost_matrix: CostMatrix, 
                            validation_data: list) -> dict:
        """Optimize thresholds to minimize expected cost"""
```

**Decision Thresholds:**
- **Block threshold**: fraud_score >= 0.85 (high confidence fraud)
- **Review threshold**: 0.50 <= fraud_score < 0.85 (uncertain, needs human review)
- **Approve threshold**: fraud_score < 0.50 (likely legitimate)

**Threshold Calibration:**
- Use validation set to compute precision/recall at different thresholds
- Calculate expected cost: `FP_rate * FP_cost + FN_rate * FN_cost`
- Select thresholds that minimize expected cost

**Performance Requirements:**
- Classification latency: < 5ms
- Support dynamic threshold updates

### 5. Explainability Engine

**Responsibilities:**
- Generate SHAP values for model predictions
- Identify top contributing features
- Create human-readable explanations

**Interface:**
```python
class Explanation:
    top_features: list[tuple[str, float]]  # (feature_name, shap_value)
    summary: str  # Human-readable explanation
    feature_values: dict  # Actual feature values
    
class ExplainabilityEngine:
    def explain(self, features: Features, 
                prediction: ModelPrediction) -> Explanation:
        """Generate explanation for prediction"""
        
    def format_explanation(self, explanation: Explanation) -> str:
        """Format explanation for display"""
```

**Explanation Generation:**
1. Compute SHAP values for the transaction
2. Select top 3-5 features by absolute SHAP value
3. Generate natural language summary:
   - "Flagged due to: high transaction velocity (15 tx in 5 min, normal is 2), unusual amount ($5000, 10x user average), new device"

**Performance Requirements:**
- Explanation generation: < 100ms
- Pre-compute SHAP explainer during model training

### 6. Alert System

**Responsibilities:**
- Generate alerts for review and block decisions
- Prioritize alerts by fraud score and business impact
- Support multiple notification channels
- Track alert status and analyst actions

**Interface:**
```python
class Alert:
    alert_id: str
    transaction: Transaction
    decision: Decision
    explanation: Explanation
    priority: str  # "high", "medium", "low"
    status: str  # "pending", "reviewed", "resolved"
    created_at: datetime
    
class AlertSystem:
    def create_alert(self, transaction: Transaction, 
                     decision: Decision, 
                     explanation: Explanation) -> Alert:
        """Create new alert"""
        
    def prioritize_alerts(self, alerts: list[Alert]) -> list[Alert]:
        """Sort alerts by priority"""
        
    def update_alert_status(self, alert_id: str, 
                           status: str, 
                           analyst_notes: str) -> None:
        """Update alert after analyst review"""
```

**Alert Prioritization:**
- High priority: fraud_score >= 0.85 OR amount > $10,000
- Medium priority: 0.70 <= fraud_score < 0.85
- Low priority: 0.50 <= fraud_score < 0.70

**Performance Requirements:**
- Alert creation: < 200ms
- Support 100+ concurrent alerts

### 7. Analyst Dashboard

**Responsibilities:**
- Display real-time feed of flagged transactions
- Provide search and filtering capabilities
- Show system metrics and performance
- Record analyst decisions for feedback loop

**Interface:**
```python
class DashboardService:
    def get_pending_alerts(self, filters: dict) -> list[Alert]:
        """Retrieve alerts for review"""
        
    def get_system_metrics(self, time_range: tuple) -> Metrics:
        """Retrieve system performance metrics"""
        
    def record_analyst_decision(self, alert_id: str, 
                               decision: str, 
                               notes: str) -> None:
        """Record analyst feedback"""
```

**Dashboard Features:**
- Real-time alert feed with auto-refresh
- Transaction detail view with explanation
- Filtering by user, merchant, amount, time range
- System metrics: throughput, latency, false positive rate
- Analyst action tracking for model retraining

### 8. API Service

**Responsibilities:**
- Expose REST API for transaction scoring
- Handle authentication and rate limiting
- Return fraud score, decision, and explanation

**Interface:**
```python
class ScoringRequest:
    transaction: Transaction
    
class ScoringResponse:
    transaction_id: str
    fraud_score: float
    decision: str
    explanation: str
    processing_time_ms: float

class APIService:
    @app.post("/score")
    async def score_transaction(self, request: ScoringRequest) -> ScoringResponse:
        """Score a transaction and return decision"""
```

**API Endpoints:**
- `POST /score`: Score a single transaction
- `GET /transaction/{id}`: Retrieve transaction details
- `GET /metrics`: System health and performance metrics

**Performance Requirements:**
- p95 latency: < 100ms
- p99 latency: < 200ms
- Throughput: 1000+ requests/sec
- Uptime: 99.9%

## Data Models

### Transaction Table
```sql
CREATE TABLE transactions (
    transaction_id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    merchant_id VARCHAR(255) NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    device_id VARCHAR(255),
    ip_address VARCHAR(45),
    latitude DECIMAL(9, 6),
    longitude DECIMAL(9, 6),
    country VARCHAR(2),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_user_timestamp (user_id, timestamp),
    INDEX idx_merchant_timestamp (merchant_id, timestamp)
);
```

### Features Table
```sql
CREATE TABLE features (
    transaction_id VARCHAR(255) PRIMARY KEY,
    tx_count_1m INT,
    tx_count_5m INT,
    tx_count_1h INT,
    amount_deviation_from_mean DECIMAL(10, 4),
    amount_deviation_from_median DECIMAL(10, 4),
    amount_percentile DECIMAL(5, 4),
    device_frequency INT,
    merchant_frequency INT,
    geo_time_inconsistency_score DECIMAL(5, 4),
    distance_from_last_tx DECIMAL(10, 2),
    time_since_last_tx INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (transaction_id) REFERENCES transactions(transaction_id)
);
```

### Predictions Table
```sql
CREATE TABLE predictions (
    transaction_id VARCHAR(255) PRIMARY KEY,
    fraud_score DECIMAL(5, 4) NOT NULL,
    unsupervised_score DECIMAL(5, 4),
    supervised_score DECIMAL(5, 4),
    model_version VARCHAR(50),
    decision VARCHAR(20) NOT NULL,
    threshold_used DECIMAL(5, 4),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (transaction_id) REFERENCES transactions(transaction_id),
    INDEX idx_decision_timestamp (decision, created_at),
    INDEX idx_fraud_score (fraud_score)
);
```

### Alerts Table
```sql
CREATE TABLE alerts (
    alert_id VARCHAR(255) PRIMARY KEY,
    transaction_id VARCHAR(255) NOT NULL,
    priority VARCHAR(20) NOT NULL,
    status VARCHAR(20) NOT NULL,
    explanation TEXT,
    analyst_id VARCHAR(255),
    analyst_decision VARCHAR(20),
    analyst_notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    reviewed_at TIMESTAMP,
    FOREIGN KEY (transaction_id) REFERENCES transactions(transaction_id),
    INDEX idx_status_priority (status, priority),
    INDEX idx_created_at (created_at)
);
```

### User Baselines Table (Cached in Redis)
```sql
CREATE TABLE user_baselines (
    user_id VARCHAR(255) PRIMARY KEY,
    mean_amount DECIMAL(10, 2),
    median_amount DECIMAL(10, 2),
    std_amount DECIMAL(10, 2),
    total_transactions INT,
    last_updated TIMESTAMP,
    INDEX idx_last_updated (last_updated)
);
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*


### Property 1: Invalid Transaction Rejection
*For any* transaction with invalid or incomplete data (missing required fields, invalid data types, out-of-range values), the Ingestion Pipeline should reject it and not persist it to the database.
**Validates: Requirements 1.2**

### Property 2: Transaction Ordering Preservation
*For any* sequence of transactions from the same user, the order in which they are processed should match the order in which they were received (based on timestamps).
**Validates: Requirements 1.5**

### Property 3: Velocity Feature Correctness
*For any* transaction and time window (1m, 5m, 1h), the computed velocity feature should equal the actual count of transactions from that user within the time window.
**Validates: Requirements 2.1**

### Property 4: Amount Deviation Calculation
*For any* transaction and user baseline, the computed amount deviation should correctly reflect the difference between the transaction amount and the user's historical mean/median.
**Validates: Requirements 2.2**

### Property 5: Frequency Metrics Accuracy
*For any* transaction, the device frequency and merchant frequency should equal the actual count of transactions with that device/merchant in the last 24 hours.
**Validates: Requirements 2.3**

### Property 6: Geo-Time Inconsistency Detection
*For any* transaction with location data, if the time and distance from the previous transaction make travel physically impossible (distance > speed_of_light * time), the geo-time inconsistency score should be high (> 0.8).
**Validates: Requirements 2.4**

### Property 7: Dual Model Execution
*For any* set of features, the Model Service should produce predictions that include both unsupervised and supervised scores (both should be non-null and in range [0, 1]).
**Validates: Requirements 3.1**

### Property 8: Ensemble Score Calculation
*For any* unsupervised score and supervised score, the final fraud score should equal the weighted combination: `0.3 * unsupervised + 0.7 * supervised`.
**Validates: Requirements 3.3**

### Property 9: Prediction Logging Completeness
*For any* prediction generated by the Model Service, a log entry should be created containing the transaction ID, fraud score, model version, and timestamp.
**Validates: Requirements 3.4, 10.3**

### Property 10: Decision Classification Correctness
*For any* fraud score, the Decision Engine should classify it correctly: approve if score < 0.50, review if 0.50 ≤ score < 0.85, block if score ≥ 0.85.
**Validates: Requirements 4.1**

### Property 11: Decision Routing Consistency
*For any* transaction decision, if the decision is "review" or "block", an alert should be created; if the decision is "approve", no alert should be created.
**Validates: Requirements 4.3, 4.4, 4.5**

### Property 12: Cost-Based Threshold Optimization
*For any* cost matrix and validation dataset, the calibrated thresholds should produce lower expected cost than using default thresholds (0.50, 0.85).
**Validates: Requirements 4.2, 12.2**

### Property 13: Explanation Completeness
*For any* transaction flagged for review or blocked, the explanation should include: (1) 3-5 top features with SHAP values, (2) actual feature values, (3) deviation from normal patterns, and (4) a human-readable summary.
**Validates: Requirements 5.1, 5.2, 5.3**

### Property 14: Alert Content Completeness
*For any* alert created, it should contain all required fields: transaction details (ID, user, merchant, amount), fraud score, decision, explanation, priority, and timestamp.
**Validates: Requirements 6.2**

### Property 15: Alert Prioritization Correctness
*For any* set of alerts, they should be ordered by priority where high priority (score ≥ 0.85 OR amount > $10,000) comes before medium priority (0.70 ≤ score < 0.85) which comes before low priority (0.50 ≤ score < 0.70).
**Validates: Requirements 6.3**

### Property 16: Alert Status Tracking
*For any* alert that is reviewed by an analyst, the alert status should be updated from "pending" to "reviewed" and the analyst's decision should be recorded.
**Validates: Requirements 6.5**

### Property 17: Dashboard Filtering Accuracy
*For any* filter criteria (user ID, merchant ID, amount range, time range), the Dashboard should return only transactions that match all specified criteria.
**Validates: Requirements 7.3**

### Property 18: System Metrics Calculation
*For any* set of transactions with ground truth labels, the calculated false positive rate should equal (false positives / total negatives) and false negative rate should equal (false negatives / total positives).
**Validates: Requirements 7.4, 10.4**

### Property 19: Analyst Feedback Persistence
*For any* analyst action on a transaction (approve, reject, add notes), the action should be persisted to the database and retrievable for model retraining.
**Validates: Requirements 7.5**

### Property 20: API Response Completeness
*For any* valid transaction submitted to the API, the response should contain: transaction ID, fraud score, decision (approve/review/block), explanation, and processing time.
**Validates: Requirements 8.2**

### Property 21: API Input Validation
*For any* invalid API request (missing fields, wrong types, malformed JSON), the API should return a 400 error with a descriptive error message and not process the transaction.
**Validates: Requirements 8.3**

### Property 22: API Authentication Enforcement
*For any* API request without valid authentication credentials, the API should return a 401 error and not process the request.
**Validates: Requirements 8.4**

### Property 23: High-Value Transaction Prioritization
*For any* set of pending transactions under resource constraints, transactions with amount > $10,000 should be processed before transactions with amount ≤ $10,000.
**Validates: Requirements 9.5**

### Property 24: Distribution Tracking
*For any* batch of predictions, the Model Service should record the distribution statistics (mean, std, percentiles) for both fraud scores and feature values.
**Validates: Requirements 10.1**

### Property 25: Performance Degradation Detection
*For any* model performance metric (precision, recall, ROC-AUC), if it degrades by more than 5% from baseline, an alert should be generated for retraining.
**Validates: Requirements 10.2**

### Property 26: Metric Update on Ground Truth
*For any* transaction that receives a ground truth label (confirmed fraud or legitimate), the system metrics (precision, recall, ROC-AUC) should be recalculated to include this new data point.
**Validates: Requirements 10.5**

### Property 27: Complete Data Persistence
*For any* transaction processed by the system, all associated data (transaction details, features, fraud score, decision, explanation) should be stored in the database and retrievable by transaction ID.
**Validates: Requirements 11.1**

### Property 28: Data Export Format Validity
*For any* data export request, the exported file should be valid CSV or JSON (parseable by standard libraries) and contain all requested fields with correct values.
**Validates: Requirements 11.4**

### Property 29: PII Encryption
*For any* transaction stored in the database, personally identifiable information fields (user_id, device_id, ip_address) should be encrypted and not readable in plaintext.
**Validates: Requirements 11.5**

### Property 30: Configurable Cost Impact
*For any* two different cost matrices, if false positive cost increases, the calibrated block threshold should increase (becoming more conservative to avoid false positives).
**Validates: Requirements 12.1**

### Property 31: Cost Metric Calculation
*For any* set of transactions with decisions and ground truth labels, the estimated fraud loss prevented should equal (true positives * average fraud amount) and false positive cost should equal (false positives * false positive cost per transaction).
**Validates: Requirements 12.3**

### Property 32: A/B Testing Isolation
*For any* two threshold configurations running in A/B test, transactions assigned to configuration A should only be evaluated using A's thresholds, and transactions assigned to B should only use B's thresholds.
**Validates: Requirements 12.4**

### Property 33: Threshold Recalibration Without Retraining
*For any* change to the cost matrix, the system should recalibrate thresholds and apply them to new transactions without requiring model retraining (model weights should remain unchanged).
**Validates: Requirements 12.5**

## Error Handling

### Ingestion Errors
- **Invalid schema**: Return 400 error with field-level validation messages
- **Database connection failure**: Retry up to 3 times with exponential backoff, then return 503
- **Kafka consumer lag**: Alert if lag exceeds 1000 messages

### Feature Engineering Errors
- **Missing user baseline**: Use global baseline as fallback
- **Redis cache miss**: Query Postgres for historical data, update cache
- **Geo-location parsing error**: Set geo-time features to null, continue processing

### Model Scoring Errors
- **Model loading failure**: Use last known good model, alert operations team
- **Feature dimension mismatch**: Log error, return default score (0.5), alert data science team
- **Prediction timeout**: Return default score (0.5), log timeout event

### Decision Engine Errors
- **Invalid fraud score**: Treat as high risk (score = 1.0), flag for review
- **Threshold configuration missing**: Use default thresholds (0.50, 0.85)

### Explainability Errors
- **SHAP computation failure**: Return feature importance from model instead
- **Explanation generation timeout**: Return simplified explanation with top features only

### Alert System Errors
- **Alert creation failure**: Retry up to 3 times, log error if all retries fail
- **Notification channel failure**: Try alternative channels, ensure at least one succeeds

### API Errors
- **Rate limit exceeded**: Return 429 with Retry-After header
- **Authentication failure**: Return 401 with WWW-Authenticate header
- **Internal server error**: Return 500, log full stack trace, alert on-call engineer

### Database Errors
- **Connection pool exhausted**: Queue requests, return 503 if queue full
- **Query timeout**: Cancel query, return partial results or error
- **Constraint violation**: Return 409 with conflict details

## Testing Strategy

The testing strategy employs both unit tests and property-based tests to ensure comprehensive coverage:

### Unit Testing
Unit tests verify specific examples, edge cases, and error conditions:
- **Ingestion**: Test parsing of valid/invalid transaction formats
- **Feature Engineering**: Test velocity calculation with known transaction sequences
- **Model Service**: Test ensemble scoring with mock model outputs
- **Decision Engine**: Test classification at boundary thresholds (0.49, 0.50, 0.84, 0.85)
- **Explainability**: Test SHAP value extraction and formatting
- **Alert System**: Test prioritization with specific fraud scores
- **API**: Test authentication, rate limiting, error responses
- **Database**: Test CRUD operations, constraint enforcement

### Property-Based Testing
Property-based tests verify universal properties across all inputs using randomized test data:

**Testing Framework**: Use **Hypothesis** (Python) for property-based testing

**Test Configuration**:
- Minimum 100 iterations per property test
- Each test tagged with: `# Feature: fraud-detection-system, Property N: [property text]`
- Generate random transactions, features, scores, and decisions
- Use shrinking to find minimal failing examples

**Key Property Tests**:
1. **Invalid Transaction Rejection** (Property 1): Generate transactions with random missing/invalid fields
2. **Transaction Ordering** (Property 2): Generate random transaction sequences, verify order preservation
3. **Velocity Features** (Property 3): Generate transaction histories, verify counts match
4. **Amount Deviation** (Property 4): Generate random amounts and baselines, verify calculation
5. **Ensemble Scoring** (Property 8): Generate random model scores, verify weighted combination
6. **Decision Classification** (Property 10): Generate random fraud scores, verify correct classification
7. **Alert Prioritization** (Property 15): Generate random alert sets, verify correct ordering
8. **API Validation** (Property 21): Generate random invalid requests, verify rejection
9. **PII Encryption** (Property 29): Generate random transactions, verify encrypted storage
10. **Cost-Based Optimization** (Property 12): Generate validation sets, verify cost minimization

**Test Data Generators**:
```python
@st.composite
def transaction_strategy(draw):
    """Generate random valid transactions"""
    return Transaction(
        transaction_id=draw(st.uuids()),
        user_id=draw(st.uuids()),
        merchant_id=draw(st.uuids()),
        amount=draw(st.floats(min_value=0.01, max_value=100000)),
        currency=draw(st.sampled_from(['USD', 'EUR', 'GBP'])),
        timestamp=draw(st.datetimes()),
        device_id=draw(st.uuids()),
        ip_address=draw(st.ip_addresses()),
        location=draw(st.none() | location_strategy())
    )

@st.composite
def invalid_transaction_strategy(draw):
    """Generate random invalid transactions"""
    # Randomly omit required fields or use invalid types
    ...
```

**Integration Testing**:
- End-to-end tests: Submit transaction → verify decision and alert creation
- Database integration: Test transaction persistence and retrieval
- API integration: Test full request/response cycle
- Model integration: Test with actual trained models (not mocks)

**Performance Testing**:
- Load testing: Verify 1000 tx/sec throughput
- Latency testing: Verify p95 < 100ms end-to-end
- Stress testing: Test behavior under 2x expected load
- Soak testing: Run at normal load for 24 hours, check for memory leaks

**Monitoring and Observability**:
- Metrics: Throughput, latency (p50, p95, p99), error rate, fraud score distribution
- Alerts: High error rate, high latency, model drift, system degradation
- Dashboards: Real-time system health, transaction feed, model performance
- Logging: Structured logs with transaction IDs for tracing

This dual testing approach ensures both concrete correctness (unit tests) and universal correctness (property tests), providing high confidence in system reliability.
