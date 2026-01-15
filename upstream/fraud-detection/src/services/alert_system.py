"""Alert system for flagged transactions."""
import uuid
from datetime import datetime
from typing import List, Optional
from sqlalchemy.orm import Session
from sqlalchemy import and_

from src.models.schemas import Transaction, Decision, Explanation, Alert
from src.models.db_models import Alert as DBAlert


class AlertSystem:
    """System for creating and managing alerts for flagged transactions."""
    
    def __init__(self, db: Session):
        """Initialize alert system.
        
        Args:
            db: Database session
        """
        self.db = db
    
    def create_alert(self, transaction: Transaction, decision: Decision,
                    explanation: Explanation, priority: str) -> Alert:
        """Create a new alert for a flagged transaction.
        
        Args:
            transaction: Transaction that was flagged
            decision: Decision made for the transaction
            explanation: Explanation for the decision
            priority: Alert priority (high/medium/low)
            
        Returns:
            Created Alert object
        """
        alert_id = str(uuid.uuid4())
        
        # Create database alert
        db_alert = DBAlert(
            alert_id=alert_id,
            transaction_id=transaction.transaction_id,
            priority=priority,
            status="pending",
            explanation=explanation.summary
        )
        
        self.db.add(db_alert)
        self.db.commit()
        self.db.refresh(db_alert)
        
        # Create Alert object
        alert = Alert(
            alert_id=alert_id,
            transaction=transaction,
            decision=decision,
            explanation=explanation,
            priority=priority,
            status="pending",
            created_at=db_alert.created_at
        )
        
        return alert
    
    def get_alert(self, alert_id: str) -> Optional[Alert]:
        """Retrieve an alert by ID.
        
        Args:
            alert_id: Alert ID
            
        Returns:
            Alert object or None if not found
        """
        db_alert = self.db.query(DBAlert).filter(
            DBAlert.alert_id == alert_id
        ).first()
        
        if not db_alert:
            return None
        
        # Note: This is a simplified version
        # In production, you'd join with transactions table and reconstruct full Alert
        return None  # Would need full reconstruction
    
    def get_pending_alerts(self, limit: int = 100) -> List[DBAlert]:
        """Get pending alerts ordered by priority and creation time.
        
        Args:
            limit: Maximum number of alerts to return
            
        Returns:
            List of pending alerts
        """
        alerts = self.db.query(DBAlert).filter(
            DBAlert.status == "pending"
        ).order_by(
            # Order by priority (high > medium > low) then by creation time
            DBAlert.priority.desc(),
            DBAlert.created_at.desc()
        ).limit(limit).all()
        
        return alerts
    
    def prioritize_alerts(self, alerts: List[Alert]) -> List[Alert]:
        """Sort alerts by priority.
        
        Priority order: high > medium > low
        Within same priority, sort by fraud score (descending)
        
        Args:
            alerts: List of alerts to prioritize
            
        Returns:
            Sorted list of alerts
        """
        priority_order = {"high": 3, "medium": 2, "low": 1}
        
        return sorted(
            alerts,
            key=lambda a: (
                priority_order.get(a.priority, 0),
                a.decision.fraud_score
            ),
            reverse=True
        )
    
    def update_alert_status(self, alert_id: str, status: str,
                           analyst_id: Optional[str] = None,
                           analyst_decision: Optional[str] = None,
                           analyst_notes: Optional[str] = None) -> bool:
        """Update alert status after analyst review.
        
        Args:
            alert_id: Alert ID
            status: New status (reviewed/resolved)
            analyst_id: ID of analyst who reviewed
            analyst_decision: Analyst's decision (approve/reject)
            analyst_notes: Analyst's notes
            
        Returns:
            True if successful, False otherwise
        """
        try:
            db_alert = self.db.query(DBAlert).filter(
                DBAlert.alert_id == alert_id
            ).first()
            
            if not db_alert:
                return False
            
            db_alert.status = status
            db_alert.reviewed_at = datetime.now()
            
            if analyst_id:
                db_alert.analyst_id = analyst_id
            if analyst_decision:
                db_alert.analyst_decision = analyst_decision
            if analyst_notes:
                db_alert.analyst_notes = analyst_notes
            
            self.db.commit()
            return True
            
        except Exception as e:
            self.db.rollback()
            print(f"⚠ Error updating alert status: {e}")
            return False
    
    def get_alerts_by_filters(self, 
                             status: Optional[str] = None,
                             priority: Optional[str] = None,
                             start_date: Optional[datetime] = None,
                             end_date: Optional[datetime] = None,
                             limit: int = 100) -> List[DBAlert]:
        """Get alerts filtered by various criteria.
        
        Args:
            status: Filter by status
            priority: Filter by priority
            start_date: Filter by creation date (start)
            end_date: Filter by creation date (end)
            limit: Maximum number of results
            
        Returns:
            List of matching alerts
        """
        query = self.db.query(DBAlert)
        
        if status:
            query = query.filter(DBAlert.status == status)
        if priority:
            query = query.filter(DBAlert.priority == priority)
        if start_date:
            query = query.filter(DBAlert.created_at >= start_date)
        if end_date:
            query = query.filter(DBAlert.created_at <= end_date)
        
        alerts = query.order_by(DBAlert.created_at.desc()).limit(limit).all()
        return alerts
    
    def get_alert_statistics(self) -> dict:
        """Get statistics about alerts.
        
        Returns:
            Dict with alert statistics
        """
        total_alerts = self.db.query(DBAlert).count()
        pending_alerts = self.db.query(DBAlert).filter(
            DBAlert.status == "pending"
        ).count()
        reviewed_alerts = self.db.query(DBAlert).filter(
            DBAlert.status == "reviewed"
        ).count()
        
        high_priority = self.db.query(DBAlert).filter(
            and_(DBAlert.status == "pending", DBAlert.priority == "high")
        ).count()
        
        return {
            "total_alerts": total_alerts,
            "pending_alerts": pending_alerts,
            "reviewed_alerts": reviewed_alerts,
            "high_priority_pending": high_priority
        }
