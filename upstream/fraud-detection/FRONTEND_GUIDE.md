# 🎨 Frontend Testing Guide

You now have **TWO** beautiful frontends to visually test your fraud detection system!

## Option 1: React Frontend (Modern SPA) 🚀

### Features:
- ✅ Modern React 19 with Vite
- ✅ Component-based architecture
- ✅ Real-time transaction testing
- ✅ Metrics dashboard with alerts
- ✅ Responsive design
- ✅ Beautiful gradient UI
- ✅ Quick test scenarios

### How to Run:

1. **Make sure the API is running:**
```bash
uvicorn src.api.main:app --reload
```

2. **In a new terminal, start the React frontend:**
```bash
cd frontend-react
npm install  # First time only
npm run dev
```

3. **Open your browser:**
   - Visit: http://localhost:5173

### What You Can Do:
- 🔍 **Test Transaction Tab**: Submit transactions and see fraud scores
  - Use quick scenario buttons (Normal, High Amount, Suspicious, Geo-Time)
  - Enter custom transaction details
  - View real-time fraud analysis with visual score gauge
  - See detailed explanations
- 📊 **Metrics Dashboard Tab**: View system analytics
  - Total alerts and status breakdown
  - Filter alerts by status and priority
  - View decision thresholds
  - Browse recent alerts

---

## Option 2: Streamlit Dashboard (Python-based) 🐍

### Features:
- ✅ Interactive web interface
- ✅ Real-time transaction testing
- ✅ Analytics and metrics
- ✅ Alert management
- ✅ System settings
- ✅ Beautiful visualizations with Plotly

### How to Run:

1. **Make sure the API is running:**
```bash
uvicorn src.api.main:app --reload
```

2. **In a new terminal, start the Streamlit dashboard:**
```bash
streamlit run src/dashboard/app.py
```

3. **Open your browser:**
   - The dashboard will automatically open at `http://localhost:8501`
   - If not, manually visit: http://localhost:8501

### What You Can Do:
- 🏠 **Home**: View system status and metrics
- 🔍 **Test Transaction**: Submit transactions and see fraud scores
- 📊 **Analytics**: View system analytics and statistics
- 🚨 **Alerts**: Manage and review fraud alerts
- ⚙️ **Settings**: View system configuration

---

## 🎯 Testing Scenarios

### Scenario 1: Normal Transaction ✅
- Amount: $50
- User: user_normal
- Expected: **APPROVE** (low fraud score)

### Scenario 2: High Amount ⚠️
- Amount: $15,000
- User: user_high_amount
- Expected: **REVIEW** (medium fraud score)

### Scenario 3: Suspicious Pattern 🚨
- Amount: $500
- User: user_suspicious
- Device: device_new
- Expected: **REVIEW or BLOCK** (high fraud score)

### Scenario 4: Velocity Abuse 🚨
- Submit 3-5 transactions quickly with same user
- Expected: **BLOCK** (high velocity detected)

---

## 📸 Comparison

### React Frontend:
- Single-page application with tabs
- Fast and responsive
- Modern component architecture
- Great for production deployment
- Mobile-friendly design

### Streamlit Dashboard:
- Multi-page application
- Python-based, easy to extend
- Interactive charts and graphs
- Great for data science demos
- Rapid prototyping

---

## 🔧 Troubleshooting

### "Failed to connect to API"
**Solution:** Make sure the API is running:
```bash
uvicorn src.api.main:app --reload
```

### "Port already in use"
**Solution:** Kill the process or use a different port:
```bash
# For React
cd frontend-react
npm run dev -- --port 5174

# For Streamlit
streamlit run src/dashboard/app.py --server.port 8502

# For API
uvicorn src.api.main:app --reload --port 8001
```

### React: "Module not found"
**Solution:** Install dependencies:
```bash
cd frontend-react
npm install
```

### CORS Errors
**Solution:** The API already has CORS enabled. If you still see errors:
1. Check that API is running on http://localhost:8000
2. Update `.env` file in frontend-react if using different URL
3. Check browser console for specific errors

---

## 🎨 Customization

### React Frontend:
Edit files in `frontend-react/src/`:
- `App.jsx` - Main application logic
- `App.css` - Global styles and colors
- `components/TransactionForm.jsx` - Transaction input form
- `components/ResultDisplay.jsx` - Fraud score display
- `components/MetricsDashboard.jsx` - Metrics and alerts

Configuration:
- Create `.env` file from `.env.example`
- Set `VITE_API_URL` and `VITE_API_KEY`

### Streamlit Dashboard:
Edit `src/dashboard/app.py` to:
- Change colors and themes
- Add new pages
- Modify visualizations
- Add more features

---

## 📊 What to Show in Demos

1. **Start with Normal Transaction**
   - Show it gets approved quickly
   - Point out the low fraud score

2. **Test High Amount**
   - Show it gets flagged for review
   - Explain the reasoning

3. **Demonstrate Velocity Abuse**
   - Submit multiple transactions quickly
   - Show how the system detects patterns

4. **Show Explainability**
   - Point out the human-readable explanations
   - Highlight top contributing features

5. **View Analytics**
   - Show system metrics
   - Demonstrate alert management
   - Filter alerts by priority

---

## 🚀 Production Deployment

### React Frontend:
```bash
cd frontend-react
npm run build
# Deploy the 'dist' folder to:
# - Netlify
# - Vercel
# - GitHub Pages
# - AWS S3 + CloudFront
```

### Streamlit Dashboard:
```bash
# Deploy to Streamlit Cloud (free)
# Or use Docker for full stack deployment
```

---

## 💡 Tips

- **React** is better for production applications and mobile users
- **Streamlit** is better for data science demos and rapid prototyping
- Both work with the same API
- You can run both simultaneously!
- React frontend is more performant and scalable

---

**Enjoy testing your fraud detection system! 🎉**
