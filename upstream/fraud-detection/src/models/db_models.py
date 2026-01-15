"""SQLAlchemy database models."""
from sqlalchemy import Column, String, Float, Integer, DateTime, Text, ForeignKey, Index, DECIMAL
from sqlalchemy.sql import func
from src.models.database import Base


class Transaction(Base):
    """Transaction table - stores all payment transactions."""
    __tablename__ = "transactions"
    
    transaction_id = Column(String(255), primary_key=True)
    user_id = Column(String(255), nullable=False)
    merchant_id = Column(String(255), nullable=False)
    amount = Column(DECIMAL(10, 2), nullable=False)
    currency = Column(String(3), nullable=False)
    timestamp = Column(DateTime, nullable=False)
    device_id = Column(String(255))
    ip_address = Column(String(45))
    latitude = Column(DECIMAL(9, 6))
    longitude = Column(DECIMAL(9, 6))
    country = Column(String(2))
    created_at = Column(DateTime, server_default=func.now())
    
    # Indexes for performance
    __table_args__ = (
        Index('idx_user_timestamp', 'user_id', 'timestamp'),
        Index('idx_merchant_timestamp', 'merchant_id', 'timestamp'),
        Index('idx_timestamp', 'timestamp'),
    )


class Features(Base):
    """Features table - stores computed features for each transaction."""
    __tablename__ = "features"
    
    transaction_id = Column(String(255), ForeignKey('transactions.transaction_id'), primary_key=True)
    
    # Velocity features
    tx_count_1m = Column(Integer)
    tx_count_5m = Column(Integer)
    tx_count_1h = Column(Integer)
    
    # Amount features
    amount_deviation_from_mean = Column(DECIMAL(10, 4))
    amount_deviation_from_median = Column(DECIMAL(10, 4))
    amount_percentile = Column(DECIMAL(5, 4))
    
    # Frequency features
    device_frequency = Column(Integer)
    merchant_frequency = Column(Integer)
    
    # Geo-time features
    geo_time_inconsistency_score = Column(DECIMAL(5, 4))
    distance_from_last_tx = Column(DECIMAL(10, 2))
    time_since_last_tx = Column(Integer)
    
    created_at = Column(DateTime, server_default=func.now())


class Prediction(Base):
    """Predictions table - stores model predictions and scores."""
    __tablename__ = "predictions"
    
    transaction_id = Column(String(255), ForeignKey('transactions.transaction_id'), primary_key=True)
    fraud_score = Column(DECIMAL(5, 4), nullable=False)
    unsupervised_score = Column(DECIMAL(5, 4))
    supervised_score = Column(DECIMAL(5, 4))
    model_version = Column(String(50))
    decision = Column(String(20), nullable=False)
    threshold_used = Column(DECIMAL(5, 4))
    created_at = Column(DateTime, server_default=func.now())
    
    # Indexes for querying
    __table_args__ = (
        Index('idx_decision_timestamp', 'decision', 'created_at'),
        Index('idx_fraud_score', 'fraud_score'),
    )


class Alert(Base):
    """Alerts table - stores alerts for flagged transactions."""
    __tablename__ = "alerts"
    
    alert_id = Column(String(255), primary_key=True)
    transaction_id = Column(String(255), ForeignKey('transactions.transaction_id'), nullable=False)
    priority = Column(String(20), nullable=False)
    status = Column(String(20), nullable=False)
    explanation = Column(Text)
    analyst_id = Column(String(255))
    analyst_decision = Column(String(20))
    analyst_notes = Column(Text)
    created_at = Column(DateTime, server_default=func.now())
    reviewed_at = Column(DateTime)
    
    # Indexes for querying
    __table_args__ = (
        Index('idx_status_priority', 'status', 'priority'),
        Index('idx_created_at', 'created_at'),
        Index('idx_transaction_id', 'transaction_id'),
    )


class UserBaseline(Base):
    """User baselines table - stores historical statistics for each user."""
    __tablename__ = "user_baselines"
    
    user_id = Column(String(255), primary_key=True)
    mean_amount = Column(DECIMAL(10, 2))
    median_amount = Column(DECIMAL(10, 2))
    std_amount = Column(DECIMAL(10, 2))
    total_transactions = Column(Integer)
    last_updated = Column(DateTime)
    
    # Index for cache refresh
    __table_args__ = (
        Index('idx_last_updated', 'last_updated'),
    )
