"""Pydantic schemas for data validation and API."""
from pydantic import BaseModel, Field, field_validator
from typing import Optional
from datetime import datetime
from decimal import Decimal


class Location(BaseModel):
    """Geographic location data."""
    latitude: float = Field(..., ge=-90, le=90)
    longitude: float = Field(..., ge=-180, le=180)
    country: str = Field(..., min_length=2, max_length=2)


class Transaction(BaseModel):
    """Transaction data model."""
    transaction_id: str
    user_id: str
    merchant_id: str
    amount: float = Field(..., gt=0)
    currency: str = Field(..., min_length=3, max_length=3)
    timestamp: datetime
    device_id: Optional[str] = None
    ip_address: Optional[str] = None
    location: Optional[Location] = None
    
    @field_validator('currency')
    @classmethod
    def validate_currency(cls, v: str) -> str:
        """Validate currency code."""
        return v.upper()


class Features(BaseModel):
    """Computed features for a transaction."""
    # Velocity features
    tx_count_1m: int = Field(..., ge=0)
    tx_count_5m: int = Field(..., ge=0)
    tx_count_1h: int = Field(..., ge=0)
    
    # Amount features
    amount_deviation_from_mean: float
    amount_deviation_from_median: float
    amount_percentile: float = Field(..., ge=0, le=1)
    
    # Frequency features
    device_frequency: int = Field(..., ge=0)
    merchant_frequency: int = Field(..., ge=0)
    
    # Geo-time features
    geo_time_inconsistency_score: float = Field(..., ge=0, le=1)
    distance_from_last_tx: float = Field(..., ge=0)
    time_since_last_tx: int = Field(..., ge=0)


class ModelPrediction(BaseModel):
    """Model prediction result."""
    model_config = {"protected_namespaces": ()}
    
    fraud_score: float = Field(..., ge=0, le=1)
    unsupervised_score: float = Field(..., ge=0, le=1)
    supervised_score: float = Field(..., ge=0, le=1)
    model_version: str


class Decision(BaseModel):
    """Transaction decision."""
    action: str = Field(..., pattern="^(approve|review|block)$")
    fraud_score: float = Field(..., ge=0, le=1)
    threshold_used: float = Field(..., ge=0, le=1)
    confidence: float = Field(..., ge=0, le=1)


class Explanation(BaseModel):
    """Explanation for a fraud decision."""
    top_features: list[tuple[str, float]]
    summary: str
    feature_values: dict


class Alert(BaseModel):
    """Alert for flagged transaction."""
    alert_id: str
    transaction: Transaction
    decision: Decision
    explanation: Explanation
    priority: str = Field(..., pattern="^(high|medium|low)$")
    status: str = Field(..., pattern="^(pending|reviewed|resolved)$")
    created_at: datetime
    analyst_id: Optional[str] = None
    analyst_decision: Optional[str] = None
    analyst_notes: Optional[str] = None
    reviewed_at: Optional[datetime] = None


class UserBaseline(BaseModel):
    """User baseline statistics."""
    user_id: str
    mean_amount: float
    median_amount: float
    std_amount: float
    total_transactions: int
    last_updated: datetime


class CostMatrix(BaseModel):
    """Cost matrix for decision optimization."""
    false_positive_cost: float = Field(..., gt=0)
    false_negative_cost: float = Field(..., gt=0)


class ScoringRequest(BaseModel):
    """API request for transaction scoring."""
    transaction: Transaction


class ScoringResponse(BaseModel):
    """API response for transaction scoring."""
    transaction_id: str
    fraud_score: float = Field(..., ge=0, le=1)
    decision: str = Field(..., pattern="^(approve|review|block)$")
    explanation: str
    processing_time_ms: float
