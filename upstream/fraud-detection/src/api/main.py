"""FastAPI application for fraud detection API."""
import time
from fastapi import FastAPI, Depends, HTTPException, Header
from fastapi.middleware.cors import CORSMiddleware
from sqlalchemy.orm import Session
from typing import Optional

from src.models.database import get_db, init_db
from src.models.schemas import ScoringRequest, ScoringResponse, Transaction
from src.services.ingestion import IngestionPipeline, ValidationException
from src.services.features import FeatureEngineeringService
from src.services.model_service import ModelService
from src.services.decision_engine import DecisionEngine
from src.services.explainability import ExplainabilityEngine
from src.services.alert_system import AlertSystem
from config.settings import settings

# Create FastAPI app
app = FastAPI(
    title="Fraud Detection API",
    description="Real-time fraud detection system for digital payments",
    version="1.0.0"
)

# Add CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Initialize services (will be created per request via dependency injection)
decision_engine = DecisionEngine()
explainability_engine = ExplainabilityEngine()


def verify_api_key(x_api_key: str = Header(...)) -> str:
    """Verify API key from header.
    
    Args:
        x_api_key: API key from X-API-Key header
        
    Returns:
        API key if valid
        
    Raises:
        HTTPException: If API key is invalid
    """
    if x_api_key != settings.api_key:
        raise HTTPException(status_code=401, detail="Invalid API key")
    return x_api_key


@app.on_event("startup")
async def startup_event():
    """Initialize database on startup."""
    print("🚀 Starting Fraud Detection API...")
    init_db()
    print("✓ Database initialized")


@app.get("/")
async def root():
    """Root endpoint."""
    return {
        "service": "Fraud Detection API",
        "version": "1.0.0",
        "status": "running"
    }


@app.get("/health")
async def health_check():
    """Health check endpoint."""
    return {"status": "healthy"}


@app.post("/score", response_model=ScoringResponse)
async def score_transaction(
    request: ScoringRequest,
    db: Session = Depends(get_db),
    api_key: str = Depends(verify_api_key)
) -> ScoringResponse:
    """Score a transaction for fraud.
    
    This endpoint processes a transaction through the complete fraud detection pipeline:
    1. Validate and ingest transaction
    2. Compute features
    3. Generate fraud score
    4. Make decision (approve/review/block)
    5. Generate explanation
    6. Create alert if needed
    
    Args:
        request: Scoring request with transaction data
        db: Database session
        api_key: Verified API key
        
    Returns:
        Scoring response with fraud score, decision, and explanation
        
    Raises:
        HTTPException: If validation or processing fails
    """
    start_time = time.time()
    
    try:
        transaction = request.transaction
        
        # 1. Ingest and validate transaction
        ingestion = IngestionPipeline(db)
        try:
            ingestion.persist_transaction(transaction)
        except ValidationException as e:
            raise HTTPException(status_code=400, detail=str(e))
        
        # 2. Compute features
        feature_service = FeatureEngineeringService(db)
        features = feature_service.compute_features(transaction)
        
        # 3. Generate fraud score
        model_service = ModelService(db)
        prediction = model_service.predict(features)
        
        # 4. Make decision
        decision = decision_engine.classify(prediction)
        
        # 5. Generate explanation (only for flagged transactions)
        if decision.action in ["review", "block"]:
            explanation = explainability_engine.explain(features, prediction)
            explanation_text = explanation.summary
            
            # 6. Create alert
            priority = decision_engine.get_priority(decision, transaction.amount)
            alert_system = AlertSystem(db)
            alert_system.create_alert(transaction, decision, explanation, priority)
        else:
            explanation_text = "Transaction approved - no anomalies detected"
        
        # Log prediction
        model_service.log_prediction(
            transaction.transaction_id,
            prediction,
            decision.action,
            decision.threshold_used
        )
        
        # Calculate processing time
        processing_time_ms = (time.time() - start_time) * 1000
        
        return ScoringResponse(
            transaction_id=transaction.transaction_id,
            fraud_score=prediction.fraud_score,
            decision=decision.action,
            explanation=explanation_text,
            processing_time_ms=processing_time_ms
        )
        
    except ValidationException as e:
        raise HTTPException(status_code=400, detail=f"Validation error: {str(e)}")
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Internal error: {str(e)}")


@app.get("/transaction/{transaction_id}")
async def get_transaction(
    transaction_id: str,
    db: Session = Depends(get_db),
    api_key: str = Depends(verify_api_key)
):
    """Retrieve transaction details by ID.
    
    Args:
        transaction_id: Transaction ID
        db: Database session
        api_key: Verified API key
        
    Returns:
        Transaction details
        
    Raises:
        HTTPException: If transaction not found
    """
    ingestion = IngestionPipeline(db)
    transaction = ingestion.get_transaction(transaction_id)
    
    if not transaction:
        raise HTTPException(status_code=404, detail="Transaction not found")
    
    return transaction


@app.get("/metrics")
async def get_metrics(
    db: Session = Depends(get_db),
    api_key: str = Depends(verify_api_key)
):
    """Get system metrics.
    
    Args:
        db: Database session
        api_key: Verified API key
        
    Returns:
        System metrics including alert statistics
    """
    alert_system = AlertSystem(db)
    alert_stats = alert_system.get_alert_statistics()
    
    return {
        "alerts": alert_stats,
        "thresholds": {
            "approve": decision_engine.approve_threshold,
            "block": decision_engine.block_threshold
        },
        "model_version": "1.0.0"
    }


@app.get("/alerts")
async def get_alerts(
    status: Optional[str] = None,
    priority: Optional[str] = None,
    limit: int = 100,
    db: Session = Depends(get_db),
    api_key: str = Depends(verify_api_key)
):
    """Get alerts with optional filtering.
    
    Args:
        status: Filter by status (pending/reviewed/resolved)
        priority: Filter by priority (high/medium/low)
        limit: Maximum number of results
        db: Database session
        api_key: Verified API key
        
    Returns:
        List of alerts
    """
    alert_system = AlertSystem(db)
    alerts = alert_system.get_alerts_by_filters(
        status=status,
        priority=priority,
        limit=limit
    )
    
    return {"alerts": [
        {
            "alert_id": alert.alert_id,
            "transaction_id": alert.transaction_id,
            "priority": alert.priority,
            "status": alert.status,
            "explanation": alert.explanation,
            "created_at": alert.created_at,
            "reviewed_at": alert.reviewed_at
        }
        for alert in alerts
    ]}


@app.post("/alerts/{alert_id}/review")
async def review_alert(
    alert_id: str,
    analyst_id: str,
    analyst_decision: str,
    analyst_notes: Optional[str] = None,
    db: Session = Depends(get_db),
    api_key: str = Depends(verify_api_key)
):
    """Record analyst review of an alert.
    
    Args:
        alert_id: Alert ID
        analyst_id: ID of analyst reviewing
        analyst_decision: Analyst's decision (approve/reject)
        analyst_notes: Optional notes
        db: Database session
        api_key: Verified API key
        
    Returns:
        Success message
        
    Raises:
        HTTPException: If alert not found
    """
    alert_system = AlertSystem(db)
    success = alert_system.update_alert_status(
        alert_id=alert_id,
        status="reviewed",
        analyst_id=analyst_id,
        analyst_decision=analyst_decision,
        analyst_notes=analyst_notes
    )
    
    if not success:
        raise HTTPException(status_code=404, detail="Alert not found")
    
    return {"message": "Alert reviewed successfully"}


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
