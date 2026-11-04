"""Decision engine for transaction classification and threshold optimization."""
from typing import List, Tuple, Dict
import numpy as np
from src.models.schemas import ModelPrediction, Decision, CostMatrix
from config.settings import settings


class DecisionEngine:
    """Engine for classifying transactions and optimizing thresholds."""
    
    def __init__(self):
        """Initialize decision engine with default thresholds."""
        self.approve_threshold = settings.approve_threshold
        self.block_threshold = settings.block_threshold
        self.cost_matrix = CostMatrix(
            false_positive_cost=settings.false_positive_cost,
            false_negative_cost=settings.false_negative_cost
        )
    
    def classify(self, prediction: ModelPrediction) -> Decision:
        """Classify transaction based on fraud score and thresholds.
        
        Args:
            prediction: Model prediction with fraud score
            
        Returns:
            Decision object with action and metadata
        """
        fraud_score = prediction.fraud_score
        
        # Determine action based on thresholds
        if fraud_score >= self.block_threshold:
            action = "block"
            threshold_used = self.block_threshold
            confidence = min((fraud_score - self.block_threshold) / (1 - self.block_threshold), 1.0)
        elif fraud_score >= self.approve_threshold:
            action = "review"
            threshold_used = self.approve_threshold
            confidence = (fraud_score - self.approve_threshold) / (self.block_threshold - self.approve_threshold)
        else:
            action = "approve"
            threshold_used = self.approve_threshold
            confidence = 1.0 - (fraud_score / self.approve_threshold)
        
        return Decision(
            action=action,
            fraud_score=fraud_score,
            threshold_used=threshold_used,
            confidence=confidence
        )
    
    def should_create_alert(self, decision: Decision) -> bool:
        """Determine if an alert should be created for this decision.
        
        Args:
            decision: Decision object
            
        Returns:
            True if alert should be created (review or block)
        """
        return decision.action in ["review", "block"]
    
    def calibrate_thresholds(self, 
                           validation_data: List[Tuple[float, bool]],
                           cost_matrix: CostMatrix) -> Dict[str, float]:
        """Optimize thresholds to minimize expected cost.
        
        Args:
            validation_data: List of (fraud_score, is_fraud) tuples
            cost_matrix: Cost matrix for false positives and false negatives
            
        Returns:
            Dict with optimized 'approve_threshold' and 'block_threshold'
        """
        if not validation_data:
            return {
                'approve_threshold': self.approve_threshold,
                'block_threshold': self.block_threshold
            }
        
        # Sort by fraud score
        sorted_data = sorted(validation_data, key=lambda x: x[0])
        scores = [x[0] for x in sorted_data]
        labels = [x[1] for x in sorted_data]
        
        # Try different threshold combinations
        best_cost = float('inf')
        best_approve = self.approve_threshold
        best_block = self.block_threshold
        
        # Sample threshold values to try
        threshold_candidates = np.linspace(0.1, 0.9, 20)
        
        for approve_th in threshold_candidates:
            for block_th in threshold_candidates:
                if block_th <= approve_th:
                    continue
                
                # Calculate cost for this threshold combination
                cost = self._calculate_cost(
                    scores, labels, approve_th, block_th, cost_matrix
                )
                
                if cost < best_cost:
                    best_cost = cost
                    best_approve = approve_th
                    best_block = block_th
        
        return {
            'approve_threshold': best_approve,
            'block_threshold': best_block,
            'expected_cost': best_cost
        }
    
    def _calculate_cost(self, 
                       scores: List[float], 
                       labels: List[bool],
                       approve_threshold: float,
                       block_threshold: float,
                       cost_matrix: CostMatrix) -> float:
        """Calculate expected cost for given thresholds.
        
        Args:
            scores: Fraud scores
            labels: True labels (True = fraud, False = legitimate)
            approve_threshold: Threshold for approve/review
            block_threshold: Threshold for review/block
            cost_matrix: Cost matrix
            
        Returns:
            Total expected cost
        """
        false_positives = 0
        false_negatives = 0
        
        for score, is_fraud in zip(scores, labels):
            # Classify based on thresholds
            if score >= block_threshold:
                predicted_fraud = True
            elif score >= approve_threshold:
                # Review cases - assume 50% are caught
                predicted_fraud = True if score > (approve_threshold + block_threshold) / 2 else False
            else:
                predicted_fraud = False
            
            # Count errors
            if predicted_fraud and not is_fraud:
                false_positives += 1
            elif not predicted_fraud and is_fraud:
                false_negatives += 1
        
        # Calculate total cost
        total_cost = (
            false_positives * cost_matrix.false_positive_cost +
            false_negatives * cost_matrix.false_negative_cost
        )
        
        return total_cost
    
    def update_thresholds(self, approve_threshold: float, block_threshold: float) -> None:
        """Update decision thresholds.
        
        Args:
            approve_threshold: New approve threshold
            block_threshold: New block threshold
            
        Raises:
            ValueError: If thresholds are invalid
        """
        if not (0 <= approve_threshold < block_threshold <= 1):
            raise ValueError(
                f"Invalid thresholds: approve={approve_threshold}, block={block_threshold}. "
                "Must satisfy: 0 <= approve < block <= 1"
            )
        
        self.approve_threshold = approve_threshold
        self.block_threshold = block_threshold
        
        print(f"✓ Thresholds updated: approve={approve_threshold:.3f}, block={block_threshold:.3f}")
    
    def update_cost_matrix(self, cost_matrix: CostMatrix) -> None:
        """Update cost matrix for threshold optimization.
        
        Args:
            cost_matrix: New cost matrix
        """
        self.cost_matrix = cost_matrix
        print(f"✓ Cost matrix updated: FP=${cost_matrix.false_positive_cost}, "
              f"FN=${cost_matrix.false_negative_cost}")
    
    def get_priority(self, decision: Decision, amount: float) -> str:
        """Determine alert priority based on decision and amount.
        
        Args:
            decision: Decision object
            amount: Transaction amount
            
        Returns:
            Priority level: "high", "medium", or "low"
        """
        # High priority: block decision OR high amount
        if decision.action == "block" or amount > settings.high_value_threshold:
            return "high"
        
        # Medium priority: high fraud score in review range
        if decision.fraud_score >= 0.70:
            return "medium"
        
        # Low priority: lower fraud score in review range
        return "low"
