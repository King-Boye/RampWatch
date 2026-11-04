you a# Implementation Plan: Real-Time Fraud & Anomaly Detection System

## Overview

This implementation plan breaks down the fraud detection system into incremental, testable components. We'll build the system in layers: data models and storage, feature engineering, model service, decision engine, explainability, alerting, API, and dashboard. Each major component includes property-based tests to validate correctness properties from the design document.

The implementation follows a bottom-up approach, building core functionality first and integrating components progressively. Testing is integrated throughout to catch errors early.

## Tasks

- [x] 1. Set up project structure and dependencies
  - Create Python project with virtual environment
  - Install dependencies: FastAPI, SQLAlchemy, Redis, scikit-learn, XGBoost, SHAP, Hypothesis, pytest
  - Set up directory structure: `src/`, `tests/`, `models/`, `config/`
  - Create configuration files for database, Redis, and application settings
  - _Requirements: All_

- [x] 2. Implement data models and database schema
  - [x] 2.1 Create SQLAlchemy models for transactions, features, predictions, alerts, user_baselines tables
    - Define Transaction, Features, Prediction, Alert, UserBaseline models
    - Add indexes for performance (user_id + timestamp, fraud_score, etc.)
    - _Requirements: 11.1_

  - [ ]* 2.2 Write property test for data persistence
    - **Property 27: Complete Data Persistence**
    - **Validates: Requirements 11.1**

  - [x] 2.3 Implement database initialization and migration scripts
    - Create tables with proper constraints and indexes
    - Add database connection pooling
    - _Requirements: 11.1_

  - [ ]* 2.4 Write property test for PII encryption
    - **Property 29: PII Encryption**
    - **Validates: Requirements 11.5**

- [x] 3. Implement Ingestion Pipeline
  - [x] 3.1 Create Transaction data class and validation logic
    - Define Transaction schema with required fields
    - Implement validation for data types, ranges, required fields
    - _Requirements: 1.2_

  - [ ]* 3.2 Write property test for invalid transaction rejection
    - **Property 1: Invalid Transaction Rejection**
    - **Validates: Requirements 1.2**

  - [x] 3.3 Implement transaction parsing and persistence
    - Parse incoming transaction data
    - Persist valid transactions to database
    - Log and reject invalid transactions
    - _Requirements: 1.1, 1.3_

  - [ ]* 3.4 Write property test for transaction ordering preservation
    - **Property 2: Transaction Ordering Preservation**
    - **Validates: Requirements 1.5**

- [x] 4. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 5. Implement Feature Engineering Service
  - [x] 5.1 Create Redis client for caching user baselines
    - Set up Redis connection
    - Implement cache get/set operations with TTL
    - _Requirements: 2.2_

  - [x] 5.2 Implement velocity feature calculation
    - Query transactions in 1m, 5m, 1h windows
    - Count transactions per user per window
    - _Requirements: 2.1_

  - [ ]* 5.3 Write property test for velocity feature correctness
    - **Property 3: Velocity Feature Correctness**
    - **Validates: Requirements 2.1**

  - [x] 5.4 Implement amount deviation calculation
    - Retrieve user baseline (mean, median, std) from cache/database
    - Calculate deviation from baseline
    - Handle missing baselines with global fallback
    - _Requirements: 2.2_

  - [ ]* 5.5 Write property test for amount deviation calculation
    - **Property 4: Amount Deviation Calculation**
    - **Validates: Requirements 2.2**

  - [x] 5.6 Implement device and merchant frequency metrics
    - Count device occurrences in last 24 hours
    - Count merchant occurrences in last 24 hours
    - _Requirements: 2.3_

  - [ ]* 5.7 Write property test for frequency metrics accuracy
    - **Property 5: Frequency Metrics Accuracy**
    - **Validates: Requirements 2.3**

  - [x] 5.8 Implement geo-time inconsistency detection
    - Calculate distance between consecutive transactions
    - Calculate time difference
    - Compute inconsistency score based on physical impossibility
    - _Requirements: 2.4_

  - [ ]* 5.9 Write property test for geo-time inconsistency detection
    - **Property 6: Geo-Time Inconsistency Detection**
    - **Validates: Requirements 2.4**

  - [x] 5.10 Create FeatureEngineeringService class that orchestrates all feature computation
    - Combine all feature calculators
    - Return Features object with all computed features
    - _Requirements: 2.1, 2.2, 2.3, 2.4_

- [x] 6. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 7. Implement Model Service
  - [x] 7.1 Create model loading and management infrastructure
    - Load Isolation Forest and XGBoost models from disk
    - Support model versioning
    - Implement hot-swapping for model updates
    - _Requirements: 3.1, 3.5_

  - [x] 7.2 Implement prediction generation
    - Execute unsupervised model (Isolation Forest)
    - Execute supervised model (XGBoost)
    - Return ModelPrediction with both scores
    - _Requirements: 3.1, 3.2_

  - [ ]* 7.3 Write property test for dual model execution
    - **Property 7: Dual Model Execution**
    - **Validates: Requirements 3.1**

  - [x] 7.4 Implement ensemble scoring
    - Combine unsupervised and supervised scores with weights (0.3, 0.7)
    - Calculate final fraud score
    - _Requirements: 3.3_

  - [ ]* 7.5 Write property test for ensemble score calculation
    - **Property 8: Ensemble Score Calculation**
    - **Validates: Requirements 3.3**

  - [x] 7.6 Implement prediction logging
    - Log all predictions with transaction ID, scores, model version, timestamp
    - Store logs in database for audit
    - _Requirements: 3.4, 10.3_

  - [ ]* 7.7 Write property test for prediction logging completeness
    - **Property 9: Prediction Logging Completeness**
    - **Validates: Requirements 3.4, 10.3**

- [x] 8. Implement Decision Engine
  - [x] 8.1 Create threshold-based classifier
    - Classify fraud scores: approve (< 0.50), review (0.50-0.85), block (≥ 0.85)
    - Return Decision object with action and metadata
    - _Requirements: 4.1_

  - [ ]* 8.2 Write property test for decision classification correctness
    - **Property 10: Decision Classification Correctness**
    - **Validates: Requirements 4.1**

  - [x] 8.3 Implement decision routing logic
    - Route review/block decisions to Alert System
    - Allow approve decisions to proceed
    - _Requirements: 4.3, 4.4, 4.5_

  - [ ]* 8.4 Write property test for decision routing consistency
    - **Property 11: Decision Routing Consistency**
    - **Validates: Requirements 4.3, 4.4, 4.5**

  - [x] 8.5 Implement cost-based threshold calibration
    - Accept CostMatrix configuration
    - Optimize thresholds to minimize expected cost on validation data
    - Support threshold updates without model retraining
    - _Requirements: 4.2, 12.1, 12.2, 12.5_

  - [ ]* 8.6 Write property test for cost-based threshold optimization
    - **Property 12: Cost-Based Threshold Optimization**
    - **Validates: Requirements 4.2, 12.2**

  - [ ]* 8.7 Write property test for configurable cost impact
    - **Property 30: Configurable Cost Impact**
    - **Validates: Requirements 12.1**

  - [ ]* 8.8 Write property test for threshold recalibration without retraining
    - **Property 33: Threshold Recalibration Without Retraining**
    - **Validates: Requirements 12.5**

- [x] 9. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 10. Implement Explainability Engine
  - [x] 10.1 Set up SHAP explainer for models
    - Initialize SHAP TreeExplainer for XGBoost
    - Pre-compute background data for SHAP
    - _Requirements: 5.2_

  - [x] 10.2 Implement SHAP value computation
    - Compute SHAP values for transaction features
    - Identify top 3-5 features by absolute SHAP value
    - _Requirements: 5.1, 5.2_

  - [x] 10.3 Implement explanation formatting
    - Include feature names, SHAP values, actual values, deviations
    - Generate human-readable summary text
    - _Requirements: 5.1, 5.3_

  - [ ]* 10.4 Write property test for explanation completeness
    - **Property 13: Explanation Completeness**
    - **Validates: Requirements 5.1, 5.2, 5.3**

- [x] 11. Implement Alert System
  - [x] 11.1 Create Alert data model and creation logic
    - Generate unique alert IDs
    - Include transaction, decision, explanation, priority, timestamp
    - Persist alerts to database
    - _Requirements: 6.1, 6.2_

  - [ ]* 11.2 Write property test for alert content completeness
    - **Property 14: Alert Content Completeness**
    - **Validates: Requirements 6.2**

  - [x] 11.3 Implement alert prioritization
    - Classify alerts as high/medium/low priority based on fraud score and amount
    - Sort alerts by priority
    - _Requirements: 6.3_

  - [ ]* 11.4 Write property test for alert prioritization correctness
    - **Property 15: Alert Prioritization Correctness**
    - **Validates: Requirements 6.3**

  - [x] 11.5 Implement alert status tracking
    - Update alert status when analyst reviews
    - Record analyst decision and notes
    - _Requirements: 6.5, 7.5_

  - [ ]* 11.6 Write property test for alert status tracking
    - **Property 16: Alert Status Tracking**
    - **Validates: Requirements 6.5**

  - [ ]* 11.7 Write property test for analyst feedback persistence
    - **Property 19: Analyst Feedback Persistence**
    - **Validates: Requirements 7.5**

- [ ] 12. Implement Model Monitoring
  - [ ] 12.1 Implement distribution tracking
    - Track fraud score distributions (mean, std, percentiles)
    - Track feature distributions over time
    - Store distribution statistics
    - _Requirements: 10.1_

  - [ ]* 12.2 Write property test for distribution tracking
    - **Property 24: Distribution Tracking**
    - **Validates: Requirements 10.1**

  - [ ] 12.3 Implement performance metric calculation
    - Calculate precision, recall, ROC-AUC from predictions and ground truth
    - Calculate false positive rate and false negative rate
    - _Requirements: 7.4, 10.4_

  - [ ]* 12.4 Write property test for system metrics calculation
    - **Property 18: System Metrics Calculation**
    - **Validates: Requirements 7.4, 10.4**

  - [ ] 12.5 Implement performance degradation detection
    - Compare current metrics to baseline
    - Generate alert if degradation > 5%
    - _Requirements: 10.2_

  - [ ]* 12.6 Write property test for performance degradation detection
    - **Property 25: Performance Degradation Detection**
    - **Validates: Requirements 10.2**

  - [ ] 12.7 Implement metric updates on ground truth
    - Accept ground truth labels for transactions
    - Recalculate metrics with new labels
    - _Requirements: 10.5_

  - [ ]* 12.8 Write property test for metric update on ground truth
    - **Property 26: Metric Update on Ground Truth**
    - **Validates: Requirements 10.5**

  - [ ] 12.9 Implement cost metric calculation
    - Calculate estimated fraud loss prevented
    - Calculate false positive costs
    - _Requirements: 12.3_

  - [ ]* 12.10 Write property test for cost metric calculation
    - **Property 31: Cost Metric Calculation**
    - **Validates: Requirements 12.3**

- [ ] 13. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 14. Implement REST API Service
  - [x] 14.1 Create FastAPI application and endpoints
    - Set up FastAPI app with CORS, logging
    - Create POST /score endpoint
    - Create GET /transaction/{id} endpoint
    - Create GET /metrics endpoint
    - _Requirements: 8.1, 8.2_

  - [x] 14.2 Implement transaction scoring endpoint
    - Accept ScoringRequest with transaction data
    - Orchestrate: ingestion → features → model → decision → explanation
    - Return ScoringResponse with fraud score, decision, explanation, processing time
    - _Requirements: 8.2_

  - [ ]* 14.3 Write property test for API response completeness
    - **Property 20: API Response Completeness**
    - **Validates: Requirements 8.2**

  - [x] 14.4 Implement input validation and error handling
    - Validate request schema
    - Return 400 with descriptive errors for invalid requests
    - _Requirements: 8.3_

  - [ ]* 14.5 Write property test for API input validation
    - **Property 21: API Input Validation**
    - **Validates: Requirements 8.3**

  - [x] 14.6 Implement authentication and rate limiting
    - Add API key authentication
    - Implement rate limiting (e.g., 100 requests/minute per key)
    - Return 401 for unauthenticated requests
    - Return 429 for rate limit exceeded
    - _Requirements: 8.4_

  - [ ]* 14.7 Write property test for API authentication enforcement
    - **Property 22: API Authentication Enforcement**
    - **Validates: Requirements 8.4**

- [ ] 15. Implement Dashboard Service
  - [ ] 15.1 Create dashboard backend API endpoints
    - GET /alerts - retrieve pending alerts with filters
    - POST /alerts/{id}/review - record analyst decision
    - GET /dashboard/metrics - retrieve system metrics
    - _Requirements: 7.1, 7.2, 7.3, 7.4_

  - [ ] 15.2 Implement alert filtering and search
    - Filter by user_id, merchant_id, amount range, time range
    - Return only matching alerts
    - _Requirements: 7.3_

  - [ ]* 15.3 Write property test for dashboard filtering accuracy
    - **Property 17: Dashboard Filtering Accuracy**
    - **Validates: Requirements 7.3**

  - [ ] 15.3 Create simple Streamlit dashboard UI
    - Display real-time alert feed
    - Show transaction details, fraud score, explanation for each alert
    - Provide filter controls
    - Display system metrics (throughput, latency, FP rate, FN rate)
    - Allow analyst to review and take action on alerts
    - _Requirements: 7.1, 7.2, 7.4, 7.5_

- [ ] 16. Implement Data Export and Utilities
  - [ ] 16.1 Implement CSV and JSON export functionality
    - Query transactions by filters
    - Export to CSV format
    - Export to JSON format
    - _Requirements: 11.4_

  - [ ]* 16.2 Write property test for data export format validity
    - **Property 28: Data Export Format Validity**
    - **Validates: Requirements 11.4**

- [ ] 17. Implement A/B Testing Infrastructure
  - [ ] 17.1 Create A/B test configuration and assignment logic
    - Assign transactions to test groups (A or B)
    - Apply group-specific thresholds
    - Track which group each transaction belongs to
    - _Requirements: 12.4_

  - [ ]* 17.2 Write property test for A/B testing isolation
    - **Property 32: A/B Testing Isolation**
    - **Validates: Requirements 12.4**

- [ ] 18. Implement Resource Prioritization
  - [ ] 18.1 Create transaction prioritization logic
    - Prioritize high-value transactions (amount > $10,000)
    - Implement priority queue for processing
    - _Requirements: 9.5_

  - [ ]* 18.2 Write property test for high-value transaction prioritization
    - **Property 23: High-Value Transaction Prioritization**
    - **Validates: Requirements 9.5**

- [ ] 19. Integration and End-to-End Testing
  - [ ]* 19.1 Write end-to-end integration tests
    - Test full pipeline: transaction → features → model → decision → alert
    - Test API request/response cycle
    - Test dashboard alert retrieval and review

  - [ ]* 19.2 Write performance tests
    - Test throughput (1000 tx/sec)
    - Test latency (p95 < 100ms)
    - Test under load (2x expected volume)

- [ ] 20. Final Checkpoint - Ensure all tests pass
  - Run full test suite
  - Verify all property tests pass with 100+ iterations
  - Ensure all integration tests pass
  - Ask the user if questions arise

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- Property tests validate universal correctness properties from the design document
- Unit tests validate specific examples and edge cases
- Integration tests validate end-to-end system behavior
- The implementation builds incrementally with checkpoints to validate progress
- All property tests should run with minimum 100 iterations using Hypothesis framework
- Each property test must be tagged with: `# Feature: fraud-detection-system, Property N: [property text]`
