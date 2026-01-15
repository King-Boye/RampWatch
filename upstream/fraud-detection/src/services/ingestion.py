"""Transaction ingestion and validation service."""
from typing import Optional, Dict, Any
from datetime import datetime
from pydantic import ValidationError
from sqlalchemy.orm import Session

from src.models.schemas import Transaction, Location
from src.models.db_models import Transaction as DBTransaction
from src.models.encryption import pii_encryption


class ValidationException(Exception):
    """Exception raised when transaction validation fails."""
    pass


class IngestionPipeline:
    """Pipeline for ingesting and validating transactions."""
    
    def __init__(self, db: Session):
        """Initialize ingestion pipeline.
        
        Args:
            db: Database session
        """
        self.db = db
    
    def validate_transaction(self, raw_data: Dict[str, Any]) -> Transaction:
        """Validate and parse transaction data.
        
        Args:
            raw_data: Raw transaction data dictionary
            
        Returns:
            Validated Transaction object
            
        Raises:
            ValidationException: If validation fails
        """
        try:
            # Parse location if present
            if 'location' in raw_data and raw_data['location']:
                location_data = raw_data['location']
                location = Location(**location_data)
                raw_data['location'] = location
            
            # Validate transaction
            transaction = Transaction(**raw_data)
            return transaction
            
        except ValidationError as e:
            error_msg = f"Transaction validation failed: {e}"
            raise ValidationException(error_msg)
        except Exception as e:
            error_msg = f"Unexpected error during validation: {e}"
            raise ValidationException(error_msg)
    
    def persist_transaction(self, transaction: Transaction) -> DBTransaction:
        """Persist validated transaction to database.
        
        Args:
            transaction: Validated transaction object
            
        Returns:
            Database transaction object
            
        Raises:
            Exception: If persistence fails
        """
        try:
            # Encrypt PII fields
            encrypted_user_id = pii_encryption.encrypt(transaction.user_id)
            encrypted_device_id = pii_encryption.encrypt(transaction.device_id) if transaction.device_id else None
            encrypted_ip = pii_encryption.encrypt(transaction.ip_address) if transaction.ip_address else None
            
            # Create database object
            db_transaction = DBTransaction(
                transaction_id=transaction.transaction_id,
                user_id=encrypted_user_id,
                merchant_id=transaction.merchant_id,
                amount=transaction.amount,
                currency=transaction.currency,
                timestamp=transaction.timestamp,
                device_id=encrypted_device_id,
                ip_address=encrypted_ip,
                latitude=transaction.location.latitude if transaction.location else None,
                longitude=transaction.location.longitude if transaction.location else None,
                country=transaction.location.country if transaction.location else None
            )
            
            # Add to database
            self.db.add(db_transaction)
            self.db.commit()
            self.db.refresh(db_transaction)
            
            return db_transaction
            
        except Exception as e:
            self.db.rollback()
            raise Exception(f"Failed to persist transaction: {e}")
    
    def process_transaction(self, raw_data: Dict[str, Any]) -> Transaction:
        """Process a transaction: validate and persist.
        
        Args:
            raw_data: Raw transaction data
            
        Returns:
            Validated transaction object
            
        Raises:
            ValidationException: If validation fails
            Exception: If persistence fails
        """
        # Validate
        transaction = self.validate_transaction(raw_data)
        
        # Persist
        self.persist_transaction(transaction)
        
        return transaction
    
    def get_transaction(self, transaction_id: str) -> Optional[Transaction]:
        """Retrieve a transaction by ID.
        
        Args:
            transaction_id: Transaction ID
            
        Returns:
            Transaction object or None if not found
        """
        db_tx = self.db.query(DBTransaction).filter(
            DBTransaction.transaction_id == transaction_id
        ).first()
        
        if not db_tx:
            return None
        
        # Decrypt PII fields
        user_id = pii_encryption.decrypt(db_tx.user_id)
        device_id = pii_encryption.decrypt(db_tx.device_id) if db_tx.device_id else None
        ip_address = pii_encryption.decrypt(db_tx.ip_address) if db_tx.ip_address else None
        
        # Build location if present
        location = None
        if db_tx.latitude and db_tx.longitude:
            location = Location(
                latitude=float(db_tx.latitude),
                longitude=float(db_tx.longitude),
                country=db_tx.country
            )
        
        # Build transaction object
        transaction = Transaction(
            transaction_id=db_tx.transaction_id,
            user_id=user_id,
            merchant_id=db_tx.merchant_id,
            amount=float(db_tx.amount),
            currency=db_tx.currency,
            timestamp=db_tx.timestamp,
            device_id=device_id,
            ip_address=ip_address,
            location=location
        )
        
        return transaction
