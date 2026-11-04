# 🚀 Quick Start Guide

This guide will help you start the complete fraud detection system in the correct order.

## Prerequisites

- Python 3.12+ installed
- Node.js 18+ and npm installed
- Virtual environment activated

## Step-by-Step Startup

### 1. Start the Backend API (Required)

Open a terminal and run:

```bash
# Activate virtual environment (if not already active)
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

# Start the API server
uvicorn src.api.main:app --reload
```

**Expected output:**
```
INFO:     Uvicorn running on http://127.0.0.1:8000 (Press CTRL+C to quit)
INFO:     Started reloader process
INFO:     Started server process
INFO:     Waiting for application startup.
INFO:     Application startup complete.
```

The API is now running at: **http://localhost:8000**

---

### 2. Start the Frontend (Choose One)

#### Option A: React Frontend (Recommended) 🚀

Open a **new terminal** and run:

```bash
cd frontend-react

# First time only: Install dependencies
npm install

# Start the development server
npm run dev

# Or use the startup script
./start.sh
```

**Expected output:**
```
VITE v5.x.x  ready in xxx ms

➜  Local:   http://localhost:5173/
➜  Network: use --host to expose
```

The React frontend is now running at: **http://localhost:5173**

#### Option B: Streamlit Dashboard 🐍

Open a **new terminal** and run:

```bash
# Activate virtual environment
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

# Start Streamlit
streamlit run src/dashboard/app.py
```

**Expected output:**
```
You can now view your Streamlit app in your browser.

Local URL: http://localhost:8501
Network URL: http://192.168.x.x:8501
```

The Streamlit dashboard is now running at: **http://localhost:8501**

---

## 3. Test the System

### Using React Frontend:

1. Open http://localhost:5173 in your browser
2. Click on quick scenario buttons or enter custom transaction details
3. Click "Analyze Transaction"
4. View the fraud score, decision, and explanation
5. Switch to "Metrics Dashboard" tab to view system metrics and alerts

### Using Streamlit Dashboard:

1. Open http://localhost:8501 in your browser
2. Navigate through the pages:
   - **Home**: System overview
   - **Test Transaction**: Submit test transactions
   - **Analytics**: View system analytics
   - **Alerts**: Manage fraud alerts
   - **Settings**: View configuration

### Using API Directly:

```bash
curl -X POST "http://localhost:8000/score" \
  -H "X-API-Key: dev-api-key-change-in-production" \
  -H "Content-Type: application/json" \
  -d '{
    "transaction": {
      "transaction_id": "tx_001",
      "user_id": "user_123",
      "merchant_id": "merchant_456",
      "amount": 50.00,
      "currency": "USD",
      "timestamp": "2026-01-15T13:00:00",
      "device_id": "device_789",
      "ip_address": "192.168.1.1"
    }
  }'
```

---

## Quick Test Scenarios

### Normal Transaction ✅
- Amount: $50
- User: user_normal
- Expected: APPROVE (low fraud score)

### High Amount ⚠️
- Amount: $15,000
- User: user_high_amount
- Expected: REVIEW (medium fraud score)

### Suspicious Pattern 🚨
- Amount: $500
- User: user_suspicious
- Device: device_new
- Expected: REVIEW or BLOCK (high fraud score)

---

## Troubleshooting

### "Connection refused" or "Failed to connect to API"

**Problem:** Frontend can't reach the backend API

**Solution:**
1. Make sure the API is running: `uvicorn src.api.main:app --reload`
2. Check that it's running on http://localhost:8000
3. Look for any error messages in the API terminal

### "Port already in use"

**Problem:** Port 8000, 5173, or 8501 is already in use

**Solution:**
```bash
# For API (use different port)
uvicorn src.api.main:app --reload --port 8001

# For React (use different port)
npm run dev -- --port 5174

# For Streamlit (use different port)
streamlit run src/dashboard/app.py --server.port 8502
```

Then update the `.env` file in frontend-react to match the new API port.

### "Module not found" errors

**Problem:** Missing dependencies

**Solution:**
```bash
# For Python
pip install -r requirements.txt

# For React
cd frontend-react
npm install
```

### Database errors

**Problem:** Database not initialized

**Solution:**
```bash
python3 src/models/init_db.py create
```

---

## Stopping the System

To stop all services:

1. Press `Ctrl+C` in each terminal window
2. Wait for graceful shutdown
3. All services will stop cleanly

---

## Next Steps

- Read [FRONTEND_GUIDE.md](FRONTEND_GUIDE.md) for detailed frontend documentation
- Check [README.md](README.md) for API documentation
- View [IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md) for system details

---

**Enjoy testing your fraud detection system! 🎉**
