"""Model service for fraud detection predictions."""
import pickle
import numpy as np
from typing import Optional
from pathlib import Path
from datetime import datetime
from sqlalchemy.orm import Session

from src.models.schemas import Features, ModelPrediction
from src.models.db_models import Prediction as DBPrediction
from config.settings import settings


class ModelService:
    """Service for executing ML models and generating fraud scores."""
    
    def __init__(self, db: Session):
        """Initialize model service.
        
        Args:
            db: Database session
        """
        self.db = db
        self.unsupervised_model = None
        self.supervised_model = None
        self.model_version = "1.0.0"
        self.models_loaded = False
        
        # Try to load models
        self.load_models()
    
    def load_models(self) -> None:
        """Load trained models from disk."""
        try:
            # Try to load Isolation Forest
            iso_path = Path(settings.isolation_forest_path)
            if iso_path.exists():
                with open(iso_path, 'rb') as f:
                    self.unsupervised_model = pickle.load(f)
            
            # Try to load XGBoost
            xgb_path = Path(settings.xgboost_path)
            if xgb_path.exists():
                with open(xgb_path, 'rb') as f:
                    self.supervised_model = pickle.load(f)
            
            if self.unsupervised_model and self.supervised_model:
                self.models_loaded = True
                print(f"✓ Models loaded successfully (version {self.model_version})")
            else:
                print("⚠ Models not found, using mock predictions")
                self.models_loaded = False
                
        except Exception as e:
            print(f"⚠ Error loading models: {e}, using mock predictions")
            self.models_loaded = False
    
    def predict(self, features: Features) -> ModelPrediction:
        """Generate fraud score from features.
        
        Args:
            features: Computed features for transaction
            
        Returns:
            ModelPrediction with fraud scores
        """
        # Convert features to numpy array
        feature_array = self._features_to_array(features)
        
        if self.models_loaded:
            # Use real models
            unsupervised_score = self._predict_unsupervised(feature_array)
            supervised_score = self._predict_supervised(feature_array)
        else:
            # Use mock predictions based on feature heuristics
            unsupervised_score = self._mock_unsupervised_score(features)
            supervised_score = self._mock_supervised_score(features)
        
        # Ensemble: weighted combination
        fraud_score = (
            settings.unsupervised_weight * unsupervised_score +
            settings.supervised_weight * supervised_score
        )
        
        return ModelPrediction(
            fraud_score=fraud_score,
            unsupervised_score=unsupervised_score,
            supervised_score=supervised_score,
            model_version=self.model_version
        )
    
    def _features_to_array(self, features: Features) -> np.ndarray:
        """Convert Features object to numpy array.
        
        Args:
            features: Features object
            
        Returns:
            Numpy array of feature values
        """
        return np.array([
            features.tx_count_1m,
            features.tx_count_5m,
            features.tx_count_1h,
            features.amount_deviation_from_mean,
            features.amount_deviation_from_median,
            features.amount_percentile,
            features.device_frequency,
            features.merchant_frequency,
            features.geo_time_inconsistency_score,
            features.distance_from_last_tx,
            features.time_since_last_tx
        ]).reshape(1, -1)
    
    def _predict_unsupervised(self, feature_array: np.ndarray) -> float:
        """Predict using unsupervised model (Isolation Forest).
        
        Args:
            feature_array: Feature array
            
        Returns:
            Anomaly score (0-1, higher = more anomalous)
        """
        # Isolation Forest returns -1 for outliers, 1 for inliers
        # decision_function returns anomaly score (negative = anomalous)
        score = self.unsupervised_model.decision_function(feature_array)[0]
        
        # Normalize to 0-1 range (higher = more fraudulent)
        # Typical range is [-0.5, 0.5], normalize to [0, 1]
        normalized_score = max(0, min(1, 0.5 - score))
        
        return normalized_score
    
    def _predict_supervised(self, feature_array: np.ndarray) -> float:
        """Predict using supervised model (XGBoost).
        
        Args:
            feature_array: Feature array
            
        Returns:
            Fraud probability (0-1)
        """
        # XGBoost predict_proba returns [prob_legit, prob_fraud]
        proba = self.supervised_model.predict_proba(feature_array)[0]
        return proba[1]  # Probability of fraud class
    
    def _mock_unsupervised_score(self, features: Features) -> float:
        """Generate mock unsupervised score based on feature anomalies.
        
        Args:
            features: Features object
            
        Returns:
            Mock anomaly score (0-1)
        """
        score = 0.0
        
        # High velocity is suspicious
        if features.tx_count_1m > 5:
            score += 0.3
        elif features.tx_count_5m > 10:
            score += 0.2
        
        # Geo-time inconsistency is very suspicious
        score += features.geo_time_inconsistency_score * 0.4
        
        # High amount deviation is suspicious
        if abs(features.amount_deviation_from_mean) > 1000:
            score += 0.2
        
        # New device is slightly suspicious
        if features.device_frequency == 0:
            score += 0.1
        
        return min(score, 1.0)
    
    def _mock_supervised_score(self, features: Features) -> float:
        """Generate mock supervised score based on known fraud patterns.
        
        Args:
            features: Features object
            
        Returns:
            Mock fraud probability (0-1)
        """
        score = 0.1  # Base score
        
        # Velocity abuse pattern
        if features.tx_count_1m >= 3:
            score += 0.3
        if features.tx_count_5m >= 8:
            score += 0.2
        
        # Geo-time impossibility
        if features.geo_time_inconsistency_score > 0.8:
            score += 0.4
        
        # Amount anomaly
        if features.amount_percentile > 0.95:
            score += 0.2
        
        # New device + high amount
        if features.device_frequency == 0 and features.amount_percentile > 0.8:
            score += 0.3
        
        return min(score, 1.0)
    
    def log_prediction(self, transaction_id: str, prediction: ModelPrediction, 
                      decision: str, threshold: float) -> None:
        """Log prediction to database for audit and monitoring.
        
        Args:
            transaction_id: Transaction ID
            prediction: Model prediction
            decision: Decision made (approve/review/block)
            threshold: Threshold used for decision
        """
        try:
            db_prediction = DBPrediction(
                transaction_id=transaction_id,
                fraud_score=prediction.fraud_score,
                unsupervised_score=prediction.unsupervised_score,
                supervised_score=prediction.supervised_score,
                model_version=prediction.model_version,
                decision=decision,
                threshold_used=threshold
            )
            
            self.db.add(db_prediction)
            self.db.commit()
            
        except Exception as e:
            self.db.rollback()
            print(f"⚠ Error logging prediction: {e}")
    
    def update_models(self, isolation_forest_path: Optional[str] = None,
                     xgboost_path: Optional[str] = None) -> bool:
        """Hot-swap models without downtime.
        
        Args:
            isolation_forest_path: Path to new Isolation Forest model
            xgboost_path: Path to new XGBoost model
            
        Returns:
            True if successful, False otherwise
        """
        try:
            if isolation_forest_path:
                with open(isolation_forest_path, 'rb') as f:
                    new_model = pickle.load(f)
                self.unsupervised_model = new_model
            
            if xgboost_path:
                with open(xgboost_path, 'rb') as f:
                    new_model = pickle.load(f)
                self.supervised_model = new_model
            
            # Update version
            self.model_version = f"1.0.0-{datetime.now().strftime('%Y%m%d%H%M%S')}"
            self.models_loaded = True
            
            print(f"✓ Models updated successfully (version {self.model_version})")
            return True
            
        except Exception as e:
            print(f"✗ Error updating models: {e}")
            return False
