# Fraud Detection System - React Frontend

Modern React frontend for the Real-Time Fraud & Anomaly Detection System.

## Features

- **Transaction Testing**: Test fraud detection with various scenarios
- **Real-time Analysis**: Instant fraud scoring and decision making
- **Metrics Dashboard**: View system metrics and alert statistics
- **Alert Management**: Browse and filter fraud alerts
- **Responsive Design**: Works on desktop and mobile devices

## Quick Start

### Prerequisites

- Node.js 18+ and npm
- Backend API running on http://localhost:8000

### Installation

```bash
cd frontend-react
npm install
```

### Configuration

Create a `.env` file (copy from `.env.example`):

```bash
cp .env.example .env
```

Edit `.env` to configure:
- `VITE_API_URL`: Backend API URL (default: http://localhost:8000)
- `VITE_API_KEY`: API key for authentication

### Development

```bash
npm run dev
```

Open http://localhost:5173 in your browser.

### Build for Production

```bash
npm run build
```

The built files will be in the `dist` directory.

### Preview Production Build

```bash
npm run preview
```

## Project Structure

```
frontend-react/
├── src/
│   ├── components/
│   │   ├── TransactionForm.jsx    # Transaction input form
│   │   ├── ResultDisplay.jsx      # Fraud score results
│   │   └── MetricsDashboard.jsx   # System metrics & alerts
│   ├── App.jsx                    # Main application
│   ├── App.css                    # Application styles
│   ├── main.jsx                   # Entry point
│   └── index.css                  # Global styles
├── public/                        # Static assets
├── index.html                     # HTML template
├── package.json                   # Dependencies
└── vite.config.js                 # Vite configuration
```

## Usage

### Testing Transactions

1. Click on quick scenario buttons (Normal, High Amount, Suspicious, Geo-Time)
2. Or manually enter transaction details
3. Click "Analyze Transaction"
4. View fraud score, decision, and explanation

### Viewing Metrics

1. Click the "Metrics Dashboard" tab
2. View total alerts and their status
3. Filter alerts by status or priority
4. See decision thresholds

## API Integration

The frontend communicates with the backend API:

- `POST /score` - Score a transaction
- `GET /metrics` - Get system metrics
- `GET /alerts` - Get alerts with filters

All requests include the `X-API-Key` header for authentication.

## Technologies

- **React 19** - UI framework
- **Vite** - Build tool and dev server
- **CSS3** - Styling with gradients and animations
- **Fetch API** - HTTP requests

## Troubleshooting

### "Cannot find native binding" error

If you see an error about rolldown native bindings, the dependencies need to be reinstalled:

```bash
rm -rf node_modules package-lock.json
npm install
```

This issue was fixed by switching from `rolldown-vite` to standard Vite.

### Other Issues

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for more solutions.

- Chrome/Edge (latest)
- Firefox (latest)
- Safari (latest)

## License

See LICENSE file in the root directory.
