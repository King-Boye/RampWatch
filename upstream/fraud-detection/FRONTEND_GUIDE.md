# 🎨 Frontend Testing Guide

You now have **TWO** beautiful frontends to visually test your fraud detection system!

## Option 1: Streamlit Dashboard (Recommended) 🚀

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

## Option 2: HTML/JavaScript Frontend 🌐

### Features:
- ✅ Simple, fast, no dependencies
- ✅ Beautiful gradient design
- ✅ Quick test scenarios
- ✅ Real-time fraud scoring
- ✅ Visual fraud score gauge

### How to Run:

1. **Make sure the API is running:**
```bash
uvicorn src.api.main:app --reload
```

2. **Open the HTML file:**
```bash
# On Linux/Mac
open frontend/index.html

# Or just double-click the file in your file manager
```

3. **The page will open in your default browser**

### What You Can Do:
- Enter transaction details manually
- Use quick scenario buttons (Normal, High Amount, Suspicious, Geo-Time)
- See instant fraud analysis with visual score
- View explanations for decisions

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

## 📸 Screenshots

### Streamlit Dashboard:
- Multi-page application
- Interactive charts and graphs
- Real-time metrics
- Alert management system

### HTML Frontend:
- Single-page application
- Gradient design
- Circular fraud score gauge
- Quick scenario buttons

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
# For Streamlit
streamlit run src/dashboard/app.py --server.port 8502

# For API
uvicorn src.api.main:app --reload --port 8001
```

### CORS Errors (HTML Frontend)
**Solution:** The API already has CORS enabled. If you still see errors:
1. Make sure you're accessing via `file://` or a local server
2. Check browser console for specific errors

---

## 🎨 Customization

### Streamlit Dashboard:
Edit `src/dashboard/app.py` to:
- Change colors and themes
- Add new pages
- Modify visualizations
- Add more features

### HTML Frontend:
Edit `frontend/index.html` to:
- Change colors (CSS section)
- Modify layout
- Add new fields
- Customize styling

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

---

## 🚀 Next Steps

1. **Add More Features:**
   - Historical transaction view
   - User profiles
   - Merchant analytics
   - Custom reports

2. **Enhance Visualizations:**
   - Time-series charts
   - Heatmaps
   - Network graphs

3. **Deploy:**
   - Host on Streamlit Cloud (free)
   - Deploy HTML to GitHub Pages
   - Use Docker for full stack

---

## 💡 Tips

- **Streamlit** is better for comprehensive testing and demos
- **HTML** is better for quick tests and embedding
- Both work with the same API
- You can run both simultaneously!

---

**Enjoy testing your fraud detection system! 🎉**
