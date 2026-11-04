"""Feature engineering service for fraud detection."""
from datetime import datetime, timedelta
from typing import Optional
from sqlalchemy.orm import Session
from sqlalchemy import func
import math

from src.models.schemas import Transaction, Features, UserBaseline
from src.models.db_models import Transaction as DBTransaction, UserBaseline as DBUserBaseline
from src.services.cache import cache_service
from src.models.encryption import pii_encryption


class FeatureEngineeringService:
    """Service for computing fraud detection features."""
    
    def __init__(self, db: Session):
        """Initialize feature engineering service.
        
        Args:
            db: Database session
        """
        self.db = db
    
    def compute_features(self, transaction: Transaction) -> Features:
        """Compute all features for a transaction.
        
        Args:
            transaction: Transaction to compute features for
            
        Returns:
            Features object with all computed features
        """
        # Compute velocity features
        tx_count_1m = self._compute_velocity(transaction, minutes=1)
        tx_count_5m = self._compute_velocity(transaction, minutes=5)
        tx_count_1h = self._compute_velocity(transaction, minutes=60)
        
        # Get user baseline
        baseline = self.get_user_baseline(transaction.user_id)
        
        # Compute amount features
        if baseline:
            amount_dev_mean = transaction.amount - baseline.mean_amount
            amount_dev_median = transaction.amount - baseline.median_amount
            # Calculate percentile (simplified)
            if baseline.std_amount > 0:
                z_score = (transaction.amount - baseline.mean_amount) / baseline.std_amount
                amount_percentile = min(max((z_score + 3) / 6, 0), 1)  # Normalize to 0-1
            else:
                amount_percentile = 0.5
        else:
            amount_dev_mean = 0.0
            amount_dev_median = 0.0
            amount_percentile = 0.5
        
        # Compute frequency features
        device_freq = self._compute_device_frequency(transaction)
        merchant_freq = self._compute_merchant_frequency(transaction)
        
        # Compute geo-time features
        geo_score, distance, time_since = self._compute_geo_time_features(transaction)
        
        return Features(
            tx_count_1m=tx_count_1m,
            tx_count_5m=tx_count_5m,
            tx_count_1h=tx_count_1h,
            amount_deviation_from_mean=amount_dev_mean,
            amount_deviation_from_median=amount_dev_median,
            amount_percentile=amount_percentile,
            device_frequency=device_freq,
            merchant_frequency=merchant_freq,
            geo_time_inconsistency_score=geo_score,
            distance_from_last_tx=distance,
            time_since_last_tx=time_since
        )
    
    def _compute_velocity(self, transaction: Transaction, minutes: int) -> int:
        """Compute transaction velocity (count in time window).
        
        Args:
            transaction: Current transaction
            minutes: Time window in minutes
            
        Returns:
            Count of transactions in the time window
        """
        encrypted_user_id = pii_encryption.encrypt(transaction.user_id)
        time_threshold = transaction.timestamp - timedelta(minutes=minutes)
        
        count = self.db.query(func.count(DBTransaction.transaction_id)).filter(
            DBTransaction.user_id == encrypted_user_id,
            DBTransaction.timestamp >= time_threshold,
            DBTransaction.timestamp < transaction.timestamp
        ).scalar()
        
        return count or 0
    
    def _compute_device_frequency(self, transaction: Transaction) -> int:
        """Compute device frequency (count in last 24 hours).
        
        Args:
            transaction: Current transaction
            
        Returns:
            Count of transactions with this device in last 24 hours
        """
        if not transaction.device_id:
            return 0
        
        encrypted_device_id = pii_encryption.encrypt(transaction.device_id)
        time_threshold = transaction.timestamp - timedelta(hours=24)
        
        count = self.db.query(func.count(DBTransaction.transaction_id)).filter(
            DBTransaction.device_id == encrypted_device_id,
            DBTransaction.timestamp >= time_threshold,
            DBTransaction.timestamp < transaction.timestamp
        ).scalar()
        
        return count or 0
    
    def _compute_merchant_frequency(self, transaction: Transaction) -> int:
        """Compute merchant frequency (count in last 24 hours).
        
        Args:
            transaction: Current transaction
            
        Returns:
            Count of transactions with this merchant in last 24 hours
        """
        encrypted_user_id = pii_encryption.encrypt(transaction.user_id)
        time_threshold = transaction.timestamp - timedelta(hours=24)
        
        count = self.db.query(func.count(DBTransaction.transaction_id)).filter(
            DBTransaction.user_id == encrypted_user_id,
            DBTransaction.merchant_id == transaction.merchant_id,
            DBTransaction.timestamp >= time_threshold,
            DBTransaction.timestamp < transaction.timestamp
        ).scalar()
        
        return count or 0
    
    def _compute_geo_time_features(self, transaction: Transaction) -> tuple[float, float, int]:
        """Compute geo-time inconsistency features.
        
        Args:
            transaction: Current transaction
            
        Returns:
            Tuple of (inconsistency_score, distance_km, time_since_seconds)
        """
        if not transaction.location:
            return 0.0, 0.0, 0
        
        # Get last transaction with location
        encrypted_user_id = pii_encryption.encrypt(transaction.user_id)
        last_tx = self.db.query(DBTransaction).filter(
            DBTransaction.user_id == encrypted_user_id,
            DBTransaction.timestamp < transaction.timestamp,
            DBTransaction.latitude.isnot(None),
            DBTransaction.longitude.isnot(None)
        ).order_by(DBTransaction.timestamp.desc()).first()
        
        if not last_tx:
            return 0.0, 0.0, 0
        
        # Calculate distance using Haversine formula
        distance_km = self._haversine_distance(
            float(last_tx.latitude), float(last_tx.longitude),
            transaction.location.latitude, transaction.location.longitude
        )
        
        # Calculate time difference
        time_diff = transaction.timestamp - last_tx.timestamp
        time_seconds = int(time_diff.total_seconds())
        
        if time_seconds == 0:
            return 1.0, distance_km, 0
        
        # Calculate required speed (km/h)
        required_speed = (distance_km / time_seconds) * 3600
        
        # Speed of commercial aircraft ~900 km/h
        # If required speed > 900 km/h, it's physically impossible
        if required_speed > 900:
            inconsistency_score = min(required_speed / 900, 1.0)
        else:
            inconsistency_score = 0.0
        
        return inconsistency_score, distance_km, time_seconds
    
    def _haversine_distance(self, lat1: float, lon1: float, lat2: float, lon2: float) -> float:
        """Calculate distance between two points using Haversine formula.
        
        Args:
            lat1, lon1: First point coordinates
            lat2, lon2: Second point coordinates
            
        Returns:
            Distance in kilometers
        """
        R = 6371  # Earth's radius in kilometers
        
        lat1_rad = math.radians(lat1)
        lat2_rad = math.radians(lat2)
        delta_lat = math.radians(lat2 - lat1)
        delta_lon = math.radians(lon2 - lon1)
        
        a = (math.sin(delta_lat / 2) ** 2 +
             math.cos(lat1_rad) * math.cos(lat2_rad) * math.sin(delta_lon / 2) ** 2)
        c = 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))
        
        return R * c
    
    def get_user_baseline(self, user_id: str) -> Optional[UserBaseline]:
        """Get user baseline statistics from cache or database.
        
        Args:
            user_id: User ID
            
        Returns:
            UserBaseline object or None if not found
        """
        # Try cache first
        cache_key = f"baseline:{user_id}"
        cached = cache_service.get_json(cache_key)
        
        if cached:
            return UserBaseline(**cached)
        
        # Query database
        encrypted_user_id = pii_encryption.encrypt(user_id)
        db_baseline = self.db.query(DBUserBaseline).filter(
            DBUserBaseline.user_id == encrypted_user_id
        ).first()
        
        if db_baseline:
            baseline = UserBaseline(
                user_id=user_id,
                mean_amount=float(db_baseline.mean_amount),
                median_amount=float(db_baseline.median_amount),
                std_amount=float(db_baseline.std_amount),
                total_transactions=db_baseline.total_transactions,
                last_updated=db_baseline.last_updated
            )
            
            # Cache for future use
            cache_service.set_json(cache_key, baseline.model_dump())
            
            return baseline
        
        return None
    
    def update_user_baseline(self, user_id: str) -> UserBaseline:
        """Update user baseline statistics from historical transactions.
        
        Args:
            user_id: User ID
            
        Returns:
            Updated UserBaseline object
        """
        encrypted_user_id = pii_encryption.encrypt(user_id)
        
        # Calculate statistics from last 30 days
        time_threshold = datetime.now() - timedelta(days=30)
        
        transactions = self.db.query(DBTransaction).filter(
            DBTransaction.user_id == encrypted_user_id,
            DBTransaction.timestamp >= time_threshold
        ).all()
        
        if not transactions:
            # Return default baseline
            return UserBaseline(
                user_id=user_id,
                mean_amount=0.0,
                median_amount=0.0,
                std_amount=0.0,
                total_transactions=0,
                last_updated=datetime.now()
            )
        
        amounts = [float(tx.amount) for tx in transactions]
        mean_amount = sum(amounts) / len(amounts)
        median_amount = sorted(amounts)[len(amounts) // 2]
        
        # Calculate standard deviation
        variance = sum((x - mean_amount) ** 2 for x in amounts) / len(amounts)
        std_amount = math.sqrt(variance)
        
        # Update or create baseline
        db_baseline = self.db.query(DBUserBaseline).filter(
            DBUserBaseline.user_id == encrypted_user_id
        ).first()
        
        if db_baseline:
            db_baseline.mean_amount = mean_amount
            db_baseline.median_amount = median_amount
            db_baseline.std_amount = std_amount
            db_baseline.total_transactions = len(transactions)
            db_baseline.last_updated = datetime.now()
        else:
            db_baseline = DBUserBaseline(
                user_id=encrypted_user_id,
                mean_amount=mean_amount,
                median_amount=median_amount,
                std_amount=std_amount,
                total_transactions=len(transactions),
                last_updated=datetime.now()
            )
            self.db.add(db_baseline)
        
        self.db.commit()
        
        baseline = UserBaseline(
            user_id=user_id,
            mean_amount=mean_amount,
            median_amount=median_amount,
            std_amount=std_amount,
            total_transactions=len(transactions),
            last_updated=datetime.now()
        )
        
        # Update cache
        cache_key = f"baseline:{user_id}"
        cache_service.set_json(cache_key, baseline.model_dump())
        
        return baseline
