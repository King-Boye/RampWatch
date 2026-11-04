"""Test script for fraud detection API."""
import requests
import json
from datetime import datetime

# API configuration
API_URL = "http://localhost:8000"
API_KEY = "dev-api-key-change-in-production"
HEADERS = {
    "X-API-Key": API_KEY,
    "Content-Type": "application/json"
}


def test_health():
    """Test health endpoint."""
    print("\n🔍 Testing health endpoint...")
    response = requests.get(f"{API_URL}/health")
    print(f"Status: {response.status_code}")
    print(f"Response: {response.json()}")


def test_score_transaction(transaction_data):
    """Test transaction scoring."""
    print("\n🔍 Testing transaction scoring...")
    print(f"Transaction: {json.dumps(transaction_data, indent=2)}")
    
    response = requests.post(
        f"{API_URL}/score",
        headers=HEADERS,
        json={"transaction": transaction_data}
    )
    
    print(f"Status: {response.status_code}")
    if response.status_code == 200:
        result = response.json()
        print(f"✓ Fraud Score: {result['fraud_score']:.3f}")
        print(f"✓ Decision: {result['decision']}")
        print(f"✓ Explanation: {result['explanation']}")
        print(f"✓ Processing Time: {result['processing_time_ms']:.2f}ms")
    else:
        print(f"✗ Error: {response.text}")
    
    return response


def test_metrics():
    """Test metrics endpoint."""
    print("\n🔍 Testing metrics endpoint...")
    response = requests.get(f"{API_URL}/metrics", headers=HEADERS)
    print(f"Status: {response.status_code}")
    if response.status_code == 200:
        print(f"Metrics: {json.dumps(response.json(), indent=2)}")


def test_alerts():
    """Test alerts endpoint."""
    print("\n🔍 Testing alerts endpoint...")
    response = requests.get(f"{API_URL}/alerts", headers=HEADERS)
    print(f"Status: {response.status_code}")
    if response.status_code == 200:
        alerts = response.json()['alerts']
        print(f"Found {len(alerts)} alerts")
        for alert in alerts[:3]:  # Show first 3
            print(f"  - Alert {alert['alert_id'][:8]}... Priority: {alert['priority']}, Status: {alert['status']}")


if __name__ == "__main__":
    print("=" * 60)
    print("Fraud Detection API Test Suite")
    print("=" * 60)
    
    # Test 1: Health check
    test_health()
    
    # Test 2: Normal transaction (should be approved)
    normal_transaction = {
        "transaction_id": "tx_001",
        "user_id": "user_123",
        "merchant_id": "merchant_456",
        "amount": 50.00,
        "currency": "USD",
        "timestamp": datetime.now().isoformat(),
        "device_id": "device_789",
        "ip_address": "192.168.1.1"
    }
    test_score_transaction(normal_transaction)
    
    # Test 3: Suspicious transaction (high velocity)
    # Simulate by sending multiple transactions quickly
    print("\n🔍 Testing high-velocity transactions...")
    for i in range(3):
        suspicious_tx = {
            "transaction_id": f"tx_velocity_{i}",
            "user_id": "user_suspicious",
            "merchant_id": "merchant_456",
            "amount": 100.00 + i * 50,
            "currency": "USD",
            "timestamp": datetime.now().isoformat(),
            "device_id": "device_new",
            "ip_address": "192.168.1.100"
        }
        test_score_transaction(suspicious_tx)
    
    # Test 4: High amount transaction (should be flagged)
    high_amount_tx = {
        "transaction_id": "tx_high_amount",
        "user_id": "user_123",
        "merchant_id": "merchant_456",
        "amount": 15000.00,
        "currency": "USD",
        "timestamp": datetime.now().isoformat(),
        "device_id": "device_789",
        "ip_address": "192.168.1.1"
    }
    test_score_transaction(high_amount_tx)
    
    # Test 5: Check metrics
    test_metrics()
    
    # Test 6: Check alerts
    test_alerts()
    
    print("\n" + "=" * 60)
    print("✓ Test suite completed!")
    print("=" * 60)
