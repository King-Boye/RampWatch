"""Explainability engine for fraud detection decisions."""
from typing import List, Tuple, Dict, Optional
import numpy as np

from src.models.schemas import Features, ModelPrediction, Explanation


class ExplainabilityEngine:
    """Engine for generating explanations for fraud decisions."""
    
    def __init__(self):
        """Initialize explainability engine."""
        self.feature_names = [
            "tx_count_1m", "tx_count_5m", "tx_count_1h",
            "amount_deviation_from_mean", "amount_deviation_from_median",
            "amount_percentile", "device_frequency", "merchant_frequency",
            "geo_time_inconsistency_score", "distance_from_last_tx",
            "time_since_last_tx"
        ]
        
        # Feature importance weights (mock SHAP values)
        # In production, these would come from actual SHAP computation
        self.feature_weights = {
            "tx_count_1m": 0.15,
            "tx_count_5m": 0.12,
            "tx_count_1h": 0.08,
            "amount_deviation_from_mean": 0.10,
            "amount_deviation_from_median": 0.08,
            "amount_percentile": 0.09,
            "device_frequency": 0.07,
            "merchant_frequency": 0.06,
            "geo_time_inconsistency_score": 0.18,
            "distance_from_last_tx": 0.04,
            "time_since_last_tx": 0.03
        }
    
    def explain(self, features: Features, prediction: ModelPrediction) -> Explanation:
        """Generate explanation for a fraud prediction.
        
        Args:
            features: Transaction features
            prediction: Model prediction
            
        Returns:
            Explanation object with top features and summary
        """
        # Calculate SHAP-like values (contribution to fraud score)
        shap_values = self._calculate_shap_values(features, prediction)
        
        # Get top 5 features by absolute contribution
        top_features = sorted(
            shap_values.items(),
            key=lambda x: abs(x[1]),
            reverse=True
        )[:5]
        
        # Get feature values
        feature_values = self._get_feature_values(features)
        
        # Generate human-readable summary
        summary = self._generate_summary(top_features, feature_values, prediction)
        
        return Explanation(
            top_features=top_features,
            summary=summary,
            feature_values=feature_values
        )
    
    def _calculate_shap_values(self, features: Features, 
                               prediction: ModelPrediction) -> Dict[str, float]:
        """Calculate SHAP-like values for features.
        
        In production, this would use actual SHAP library.
        For now, we use a simplified heuristic approach.
        
        Args:
            features: Transaction features
            prediction: Model prediction
            
        Returns:
            Dict mapping feature names to SHAP values
        """
        feature_dict = features.model_dump()
        shap_values = {}
        
        for feature_name, value in feature_dict.items():
            # Base contribution from feature weight
            base_contribution = self.feature_weights.get(feature_name, 0.05)
            
            # Scale by feature value (normalized)
            if feature_name in ["tx_count_1m", "tx_count_5m", "tx_count_1h"]:
                # Velocity features: higher = more suspicious
                normalized_value = min(value / 10, 1.0)
            elif feature_name in ["amount_deviation_from_mean", "amount_deviation_from_median"]:
                # Deviation: larger absolute value = more suspicious
                normalized_value = min(abs(value) / 1000, 1.0)
            elif feature_name == "amount_percentile":
                # Percentile: extreme values (very high or very low) are suspicious
                normalized_value = abs(value - 0.5) * 2
            elif feature_name in ["device_frequency", "merchant_frequency"]:
                # Frequency: 0 (new) is suspicious, very high is also suspicious
                if value == 0:
                    normalized_value = 0.8
                elif value > 20:
                    normalized_value = 0.6
                else:
                    normalized_value = 0.2
            elif feature_name == "geo_time_inconsistency_score":
                # Direct score
                normalized_value = value
            elif feature_name == "distance_from_last_tx":
                # Large distance is suspicious
                normalized_value = min(value / 5000, 1.0)
            elif feature_name == "time_since_last_tx":
                # Very short time is suspicious
                if value < 60:
                    normalized_value = 0.8
                else:
                    normalized_value = 0.2
            else:
                normalized_value = 0.5
            
            # Calculate contribution (can be positive or negative)
            contribution = base_contribution * normalized_value * prediction.fraud_score
            
            # Add some randomness to simulate SHAP variance
            shap_values[feature_name] = contribution
        
        return shap_values
    
    def _get_feature_values(self, features: Features) -> Dict[str, float]:
        """Get feature values as dictionary.
        
        Args:
            features: Features object
            
        Returns:
            Dict of feature names to values
        """
        return features.model_dump()
    
    def _generate_summary(self, top_features: List[Tuple[str, float]],
                         feature_values: Dict[str, float],
                         prediction: ModelPrediction) -> str:
        """Generate human-readable explanation summary.
        
        Args:
            top_features: List of (feature_name, shap_value) tuples
            feature_values: Dict of feature values
            prediction: Model prediction
            
        Returns:
            Human-readable summary string
        """
        reasons = []
        
        for feature_name, shap_value in top_features[:3]:  # Top 3 features
            value = feature_values[feature_name]
            reason = self._explain_feature(feature_name, value, shap_value)
            if reason:
                reasons.append(reason)
        
        if not reasons:
            return f"Fraud score: {prediction.fraud_score:.2f}. No significant anomalies detected."
        
        summary = f"Flagged due to: {', '.join(reasons)}"
        return summary
    
    def _explain_feature(self, feature_name: str, value: float, 
                        shap_value: float) -> Optional[str]:
        """Generate explanation for a single feature.
        
        Args:
            feature_name: Name of the feature
            value: Feature value
            shap_value: SHAP contribution value
            
        Returns:
            Human-readable explanation or None
        """
        if abs(shap_value) < 0.01:  # Ignore very small contributions
            return None
        
        explanations = {
            "tx_count_1m": f"high transaction velocity ({int(value)} transactions in 1 minute)",
            "tx_count_5m": f"unusual transaction frequency ({int(value)} transactions in 5 minutes)",
            "tx_count_1h": f"elevated transaction rate ({int(value)} transactions in 1 hour)",
            "amount_deviation_from_mean": f"unusual amount (${abs(value):.2f} from user average)",
            "amount_deviation_from_median": f"atypical transaction amount (${abs(value):.2f} deviation)",
            "amount_percentile": f"extreme amount ({value*100:.0f}th percentile for user)",
            "device_frequency": "new device" if value == 0 else f"device used {int(value)} times recently",
            "merchant_frequency": "new merchant" if value == 0 else f"merchant used {int(value)} times recently",
            "geo_time_inconsistency_score": f"impossible travel detected (score: {value:.2f})",
            "distance_from_last_tx": f"large distance from last transaction ({value:.0f} km)",
            "time_since_last_tx": f"very short time since last transaction ({int(value)} seconds)"
        }
        
        return explanations.get(feature_name)
    
    def format_explanation(self, explanation: Explanation) -> str:
        """Format explanation for display.
        
        Args:
            explanation: Explanation object
            
        Returns:
            Formatted explanation string
        """
        lines = [explanation.summary, ""]
        lines.append("Top Contributing Factors:")
        
        for i, (feature_name, shap_value) in enumerate(explanation.top_features, 1):
            value = explanation.feature_values[feature_name]
            feature_display = feature_name.replace("_", " ").title()
            lines.append(f"  {i}. {feature_display}: {value:.2f} (contribution: {shap_value:.3f})")
        
        return "\n".join(lines)
