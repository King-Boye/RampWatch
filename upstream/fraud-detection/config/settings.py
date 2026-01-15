"""Application configuration settings."""
from pydantic_settings import BaseSettings
from typing import Optional


class Settings(BaseSettings):
    """Application settings loaded from environment variables."""
    
    # Database (use SQLite for development, PostgreSQL for production)
    database_url: str = "sqlite:///./fraud_detection.db"
    # For PostgreSQL: "postgresql://postgres:postgres@localhost:5432/fraud_detection"
    
    # Redis
    redis_host: str = "localhost"
    redis_port: int = 6379
    redis_db: int = 0
    redis_ttl: int = 3600  # 1 hour cache TTL
    
    # Model paths
    isolation_forest_path: str = "models/isolation_forest.pkl"
    xgboost_path: str = "models/xgboost.pkl"
    
    # Model ensemble weights
    unsupervised_weight: float = 0.3
    supervised_weight: float = 0.7
    
    # Decision thresholds
    approve_threshold: float = 0.50
    block_threshold: float = 0.85
    
    # Cost matrix (default values)
    false_positive_cost: float = 50.0  # Cost of blocking legitimate transaction
    false_negative_cost: float = 1000.0  # Cost of approving fraudulent transaction
    
    # API settings
    api_key: str = "dev-api-key-change-in-production"
    rate_limit_per_minute: int = 100
    
    # Performance settings
    high_value_threshold: float = 10000.0  # Transactions above this are high priority
    
    # Monitoring
    performance_degradation_threshold: float = 0.05  # 5% degradation triggers alert
    
    class Config:
        env_file = ".env"
        env_file_encoding = "utf-8"


# Global settings instance
settings = Settings()
