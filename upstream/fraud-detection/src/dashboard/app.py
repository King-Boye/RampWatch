"""Streamlit dashboard for fraud detection system."""
import streamlit as st
import requests
import json
from datetime import datetime
import pandas as pd
import plotly.express as px
import plotly.graph_objects as go

# Configuration
API_URL = "http://localhost:8000"
API_KEY = "dev-api-key-change-in-production"
HEADERS = {
    "X-API-Key": API_KEY,
    "Content-Type": "application/json"
}

# Page config
st.set_page_config(
    page_title="Fraud Detection Dashboard",
    page_icon="🛡️",
    layout="wide",
    initial_sidebar_state="expanded"
)

# Custom CSS
st.markdown("""
<style>
    .main-header {
        font-size: 3rem;
        font-weight: bold;
        color: #1f77b4;
        text-align: center;
        margin-bottom: 2rem;
    }
    .metric-card {
        background-color: #f0f2f6;
        padding: 1rem;
        border-radius: 0.5rem;
        margin: 0.5rem 0;
    }
    .fraud-high {
        color: #d62728;
        font-weight: bold;
    }
    .fraud-medium {
        color: #ff7f0e;
        font-weight: bold;
    }
    .fraud-low {
        color: #2ca02c;
        font-weight: bold;
    }
</style>
""", unsafe_allow_html=True)

# Title
st.markdown('<h1 class="main-header">🛡️ Fraud Detection System</h1>', unsafe_allow_html=True)

# Sidebar
with st.sidebar:
    st.header("⚙️ Navigation")
    page = st.radio(
        "Select Page",
        ["🏠 Home", "🔍 Test Transaction", "📊 Analytics", "🚨 Alerts", "⚙️ Settings"]
    )
    
    st.divider()
    
    # System status
    st.subheader("System Status")
    try:
        response = requests.get(f"{API_URL}/health", timeout=2)
        if response.status_code == 200:
            st.success("✅ API Online")
        else:
            st.error("❌ API Error")
    except:
        st.error("❌ API Offline")

# Home Page
if page == "🏠 Home":
    st.header("Welcome to the Fraud Detection Dashboard")
    
    col1, col2, col3 = st.columns(3)
    
    # Get metrics
    try:
        response = requests.get(f"{API_URL}/metrics", headers=HEADERS)
        if response.status_code == 200:
            metrics = response.json()
            
            with col1:
                st.metric(
                    "Total Alerts",
                    metrics['alerts']['total_alerts'],
                    delta=None
                )
            
            with col2:
                st.metric(
                    "Pending Alerts",
                    metrics['alerts']['pending_alerts'],
                    delta=None
                )
            
            with col3:
                st.metric(
                    "High Priority",
                    metrics['alerts']['high_priority_pending'],
                    delta=None
                )
    except:
        st.warning("Unable to fetch metrics. Make sure the API is running.")
    
    st.divider()
    
    # Quick info
    col1, col2 = st.columns(2)
    
    with col1:
        st.subheader("🎯 System Features")
        st.markdown("""
        - **Real-time Processing**: Sub-100ms latency
        - **Hybrid ML**: Isolation Forest + XGBoost
        - **Explainable AI**: SHAP-based explanations
        - **Smart Alerting**: Priority-based routing
        - **PII Protection**: Encrypted sensitive data
        """)
    
    with col2:
        st.subheader("📈 Detection Capabilities")
        st.markdown("""
        - **Velocity Abuse**: High transaction frequency
        - **Amount Anomalies**: Unusual transaction amounts
        - **Geo-Time Inconsistency**: Impossible travel
        - **Device Patterns**: New or suspicious devices
        - **Merchant Patterns**: Unusual merchant activity
        """)

# Test Transaction Page
elif page == "🔍 Test Transaction":
    st.header("Test Transaction Scoring")
    
    st.markdown("Enter transaction details to test the fraud detection system:")
    
    col1, col2 = st.columns(2)
    
    with col1:
        transaction_id = st.text_input("Transaction ID", value=f"tx_{datetime.now().strftime('%Y%m%d%H%M%S')}")
        user_id = st.text_input("User ID", value="user_123")
        merchant_id = st.text_input("Merchant ID", value="merchant_456")
        amount = st.number_input("Amount ($)", min_value=0.01, value=100.00, step=10.0)
        currency = st.selectbox("Currency", ["USD", "EUR", "GBP", "JPY"])
    
    with col2:
        device_id = st.text_input("Device ID", value="device_789")
        ip_address = st.text_input("IP Address", value="192.168.1.1")
        
        # Location (optional)
        use_location = st.checkbox("Include Location Data")
        if use_location:
            latitude = st.number_input("Latitude", value=40.7128, format="%.4f")
            longitude = st.number_input("Longitude", value=-74.0060, format="%.4f")
            country = st.text_input("Country Code", value="US", max_chars=2)
    
    st.divider()
    
    # Preset scenarios
    st.subheader("🎭 Quick Test Scenarios")
    col1, col2, col3, col4 = st.columns(4)
    
    with col1:
        if st.button("✅ Normal Transaction"):
            amount = 50.00
            user_id = "user_normal"
            st.rerun()
    
    with col2:
        if st.button("⚠️ High Amount"):
            amount = 15000.00
            user_id = "user_high_amount"
            st.rerun()
    
    with col3:
        if st.button("🚨 Suspicious Pattern"):
            amount = 500.00
            user_id = "user_suspicious"
            device_id = "device_new"
            st.rerun()
    
    with col4:
        if st.button("🌍 Geo-Time Issue"):
            use_location = True
            latitude = 51.5074  # London
            longitude = -0.1278
            st.rerun()
    
    st.divider()
    
    # Submit button
    if st.button("🔍 Score Transaction", type="primary", use_container_width=True):
        # Build transaction data
        transaction_data = {
            "transaction_id": transaction_id,
            "user_id": user_id,
            "merchant_id": merchant_id,
            "amount": amount,
            "currency": currency,
            "timestamp": datetime.now().isoformat(),
            "device_id": device_id,
            "ip_address": ip_address
        }
        
        if use_location:
            transaction_data["location"] = {
                "latitude": latitude,
                "longitude": longitude,
                "country": country
            }
        
        # Make API request
        with st.spinner("Analyzing transaction..."):
            try:
                response = requests.post(
                    f"{API_URL}/score",
                    headers=HEADERS,
                    json={"transaction": transaction_data}
                )
                
                if response.status_code == 200:
                    result = response.json()
                    
                    # Display results
                    st.success("✅ Transaction Analyzed Successfully!")
                    
                    # Fraud score gauge
                    fraud_score = result['fraud_score']
                    
                    fig = go.Figure(go.Indicator(
                        mode = "gauge+number+delta",
                        value = fraud_score * 100,
                        domain = {'x': [0, 1], 'y': [0, 1]},
                        title = {'text': "Fraud Score", 'font': {'size': 24}},
                        delta = {'reference': 50},
                        gauge = {
                            'axis': {'range': [None, 100], 'tickwidth': 1, 'tickcolor': "darkblue"},
                            'bar': {'color': "darkblue"},
                            'bgcolor': "white",
                            'borderwidth': 2,
                            'bordercolor': "gray",
                            'steps': [
                                {'range': [0, 50], 'color': '#2ca02c'},
                                {'range': [50, 85], 'color': '#ff7f0e'},
                                {'range': [85, 100], 'color': '#d62728'}
                            ],
                            'threshold': {
                                'line': {'color': "red", 'width': 4},
                                'thickness': 0.75,
                                'value': 85
                            }
                        }
                    ))
                    
                    fig.update_layout(height=300)
                    st.plotly_chart(fig, use_container_width=True)
                    
                    # Results
                    col1, col2, col3 = st.columns(3)
                    
                    with col1:
                        decision = result['decision']
                        if decision == "approve":
                            st.success(f"**Decision:** ✅ {decision.upper()}")
                        elif decision == "review":
                            st.warning(f"**Decision:** ⚠️ {decision.upper()}")
                        else:
                            st.error(f"**Decision:** 🚨 {decision.upper()}")
                    
                    with col2:
                        st.metric("Fraud Score", f"{fraud_score:.3f}")
                    
                    with col3:
                        st.metric("Processing Time", f"{result['processing_time_ms']:.2f}ms")
                    
                    # Explanation
                    st.subheader("📝 Explanation")
                    st.info(result['explanation'])
                    
                    # Transaction details
                    with st.expander("📋 Transaction Details"):
                        st.json(transaction_data)
                    
                else:
                    st.error(f"Error: {response.status_code} - {response.text}")
                    
            except Exception as e:
                st.error(f"Failed to connect to API: {str(e)}")
                st.info("Make sure the API is running: `uvicorn src.api.main:app --reload`")

# Analytics Page
elif page == "📊 Analytics":
    st.header("System Analytics")
    
    try:
        # Get metrics
        response = requests.get(f"{API_URL}/metrics", headers=HEADERS)
        if response.status_code == 200:
            metrics = response.json()
            
            # Alert statistics
            st.subheader("🚨 Alert Statistics")
            col1, col2, col3, col4 = st.columns(4)
            
            with col1:
                st.metric("Total Alerts", metrics['alerts']['total_alerts'])
            with col2:
                st.metric("Pending", metrics['alerts']['pending_alerts'])
            with col3:
                st.metric("Reviewed", metrics['alerts']['reviewed_alerts'])
            with col4:
                st.metric("High Priority", metrics['alerts']['high_priority_pending'])
            
            # Threshold settings
            st.subheader("⚙️ Current Thresholds")
            col1, col2 = st.columns(2)
            
            with col1:
                st.metric("Approve Threshold", f"{metrics['thresholds']['approve']:.2f}")
            with col2:
                st.metric("Block Threshold", f"{metrics['thresholds']['block']:.2f}")
            
            # Visualization
            st.subheader("📈 Alert Distribution")
            
            # Create sample data for visualization
            alert_data = {
                'Status': ['Pending', 'Reviewed', 'High Priority'],
                'Count': [
                    metrics['alerts']['pending_alerts'],
                    metrics['alerts']['reviewed_alerts'],
                    metrics['alerts']['high_priority_pending']
                ]
            }
            
            fig = px.bar(
                alert_data,
                x='Status',
                y='Count',
                color='Status',
                color_discrete_map={
                    'Pending': '#ff7f0e',
                    'Reviewed': '#2ca02c',
                    'High Priority': '#d62728'
                }
            )
            fig.update_layout(showlegend=False)
            st.plotly_chart(fig, use_container_width=True)
            
    except Exception as e:
        st.error(f"Unable to fetch analytics: {str(e)}")

# Alerts Page
elif page == "🚨 Alerts":
    st.header("Alert Management")
    
    # Filters
    col1, col2, col3 = st.columns(3)
    
    with col1:
        status_filter = st.selectbox("Status", ["All", "pending", "reviewed", "resolved"])
    with col2:
        priority_filter = st.selectbox("Priority", ["All", "high", "medium", "low"])
    with col3:
        limit = st.number_input("Limit", min_value=10, max_value=500, value=50)
    
    if st.button("🔄 Refresh Alerts"):
        st.rerun()
    
    st.divider()
    
    # Fetch alerts
    try:
        params = {"limit": limit}
        if status_filter != "All":
            params["status"] = status_filter
        if priority_filter != "All":
            params["priority"] = priority_filter
        
        response = requests.get(f"{API_URL}/alerts", headers=HEADERS, params=params)
        
        if response.status_code == 200:
            alerts = response.json()['alerts']
            
            if alerts:
                st.success(f"Found {len(alerts)} alerts")
                
                # Display alerts
                for alert in alerts:
                    with st.expander(
                        f"🚨 Alert {alert['alert_id'][:8]}... | "
                        f"Priority: {alert['priority'].upper()} | "
                        f"Status: {alert['status'].upper()}"
                    ):
                        col1, col2 = st.columns(2)
                        
                        with col1:
                            st.write(f"**Transaction ID:** {alert['transaction_id']}")
                            st.write(f"**Priority:** {alert['priority']}")
                            st.write(f"**Status:** {alert['status']}")
                        
                        with col2:
                            st.write(f"**Created:** {alert['created_at']}")
                            if alert['reviewed_at']:
                                st.write(f"**Reviewed:** {alert['reviewed_at']}")
                        
                        st.write(f"**Explanation:** {alert['explanation']}")
                        
                        # Review button
                        if alert['status'] == 'pending':
                            if st.button(f"Mark as Reviewed", key=f"review_{alert['alert_id']}"):
                                # Review alert
                                review_response = requests.post(
                                    f"{API_URL}/alerts/{alert['alert_id']}/review",
                                    headers=HEADERS,
                                    json={
                                        "analyst_id": "dashboard_user",
                                        "analyst_decision": "reviewed",
                                        "analyst_notes": "Reviewed via dashboard"
                                    }
                                )
                                if review_response.status_code == 200:
                                    st.success("Alert marked as reviewed!")
                                    st.rerun()
            else:
                st.info("No alerts found matching the filters.")
        else:
            st.error(f"Error fetching alerts: {response.status_code}")
            
    except Exception as e:
        st.error(f"Failed to fetch alerts: {str(e)}")

# Settings Page
elif page == "⚙️ Settings":
    st.header("System Settings")
    
    st.subheader("🔧 Configuration")
    
    st.info("Current settings are loaded from environment variables and configuration files.")
    
    col1, col2 = st.columns(2)
    
    with col1:
        st.markdown("**API Configuration**")
        st.code(f"API URL: {API_URL}")
        st.code(f"API Key: {API_KEY[:20]}...")
    
    with col2:
        st.markdown("**System Information**")
        st.code("Version: 1.0.0")
        st.code("Environment: Development")
    
    st.divider()
    
    st.subheader("📚 Documentation")
    st.markdown("""
    - **API Docs**: [http://localhost:8000/docs](http://localhost:8000/docs)
    - **GitHub**: [Repository](https://github.com/Ndifreke000/Real-Time-Fraud-Anomaly-Detection-System-for-Digital-Payments)
    - **Requirements**: `.kiro/specs/fraud-detection-system/requirements.md`
    - **Design**: `.kiro/specs/fraud-detection-system/design.md`
    """)
    
    st.divider()
    
    st.subheader("🚀 Quick Start")
    st.markdown("""
    **To start the API server:**
    ```bash
    uvicorn src.api.main:app --reload
    ```
    
    **To run this dashboard:**
    ```bash
    streamlit run src/dashboard/app.py
    ```
    """)

# Footer
st.divider()
st.markdown(
    "<p style='text-align: center; color: gray;'>🛡️ Fraud Detection System v1.0.0 | "
    "Built with FastAPI, XGBoost, and Streamlit</p>",
    unsafe_allow_html=True
)
