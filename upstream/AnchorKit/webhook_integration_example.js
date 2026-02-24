/**
 * Webhook Monitor Integration Example
 * 
 * This file shows how to integrate the webhook monitor with your backend.
 * Choose one of the three methods below based on your infrastructure.
 */

// ============================================================================
// METHOD 1: WebSocket (Recommended for Real-Time)
// ============================================================================

class WebhookMonitorWebSocket {
    constructor(wsUrl) {
        this.wsUrl = wsUrl;
        this.ws = null;
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 5;
    }

    connect() {
        this.ws = new WebSocket(this.wsUrl);

        this.ws.onopen = () => {
            console.log('✅ WebSocket connected');
            this.reconnectAttempts = 0;
        };

        this.ws.onmessage = (event) => {
            try {
                const webhookData = JSON.parse(event.data);
                this.handleWebhookEvent(webhookData);
            } catch (error) {
                console.error('Failed to parse webhook event:', error);
            }
        };

        this.ws.onerror = (error) => {
            console.error('WebSocket error:', error);
        };

        this.ws.onclose = () => {
            console.log('WebSocket closed');
            this.attemptReconnect();
        };
    }

    attemptReconnect() {
        if (this.reconnectAttempts < this.maxReconnectAttempts) {
            this.reconnectAttempts++;
            const delay = Math.min(1000 * Math.pow(2, this.reconnectAttempts), 30000);
            console.log(`Reconnecting in ${delay}ms... (attempt ${this.reconnectAttempts})`);
            setTimeout(() => this.connect(), delay);
        }
    }

    handleWebhookEvent(webhookData) {
        // Add to monitor (assumes addEvent function exists in webhook_monitor.html)
        if (typeof addEvent === 'function') {
            addEvent({
                id: ++eventCounter,
                type: webhookData.type,
                timestamp: webhookData.timestamp || new Date().toISOString(),
                payload: webhookData.payload
            });
        }
    }

    disconnect() {
        if (this.ws) {
            this.ws.close();
            this.ws = null;
        }
    }
}

// Usage:
// const monitor = new WebhookMonitorWebSocket('ws://localhost:8080/webhooks');
// monitor.connect();

// ============================================================================
// METHOD 2: Server-Sent Events (SSE)
// ============================================================================

class WebhookMonitorSSE {
    constructor(sseUrl) {
        this.sseUrl = sseUrl;
        this.eventSource = null;
    }

    connect() {
        this.eventSource = new EventSource(this.sseUrl);

        this.eventSource.onopen = () => {
            console.log('✅ SSE connected');
        };

        this.eventSource.onmessage = (event) => {
            try {
                const webhookData = JSON.parse(event.data);
                this.handleWebhookEvent(webhookData);
            } catch (error) {
                console.error('Failed to parse SSE event:', error);
            }
        };

        this.eventSource.onerror = (error) => {
            console.error('SSE error:', error);
            // SSE automatically reconnects
        };
    }

    handleWebhookEvent(webhookData) {
        if (typeof addEvent === 'function') {
            addEvent({
                id: ++eventCounter,
                type: webhookData.type,
                timestamp: webhookData.timestamp || new Date().toISOString(),
                payload: webhookData.payload
            });
        }
    }

    disconnect() {
        if (this.eventSource) {
            this.eventSource.close();
            this.eventSource = null;
        }
    }
}

// Usage:
// const monitor = new WebhookMonitorSSE('/api/webhook-stream');
// monitor.connect();

// ============================================================================
// METHOD 3: HTTP Polling (Fallback)
// ============================================================================

class WebhookMonitorPolling {
    constructor(apiUrl, intervalMs = 2000) {
        this.apiUrl = apiUrl;
        this.intervalMs = intervalMs;
        this.intervalId = null;
        this.lastEventId = 0;
    }

    start() {
        this.intervalId = setInterval(() => this.poll(), this.intervalMs);
        this.poll(); // Initial poll
    }

    async poll() {
        try {
            const response = await fetch(`${this.apiUrl}?since=${this.lastEventId}`);
            
            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }

            const events = await response.json();
            
            events.forEach(webhookData => {
                this.handleWebhookEvent(webhookData);
                if (webhookData.id > this.lastEventId) {
                    this.lastEventId = webhookData.id;
                }
            });
        } catch (error) {
            console.error('Polling error:', error);
        }
    }

    handleWebhookEvent(webhookData) {
        if (typeof addEvent === 'function') {
            addEvent({
                id: ++eventCounter,
                type: webhookData.type,
                timestamp: webhookData.timestamp || new Date().toISOString(),
                payload: webhookData.payload
            });
        }
    }

    stop() {
        if (this.intervalId) {
            clearInterval(this.intervalId);
            this.intervalId = null;
        }
    }
}

// Usage:
// const monitor = new WebhookMonitorPolling('/api/webhooks/recent', 3000);
// monitor.start();

// ============================================================================
// BACKEND EXAMPLES
// ============================================================================

// Express.js + WebSocket Example
/*
const express = require('express');
const WebSocket = require('ws');
const app = express();

const wss = new WebSocket.Server({ port: 8080 });

// Webhook endpoint
app.post('/webhook', express.json(), (req, res) => {
    const event = {
        id: Date.now(),
        type: req.body.type,
        timestamp: new Date().toISOString(),
        payload: req.body.data
    };
    
    // Broadcast to all connected clients
    wss.clients.forEach(client => {
        if (client.readyState === WebSocket.OPEN) {
            client.send(JSON.stringify(event));
        }
    });
    
    res.status(200).json({ success: true });
});

app.listen(3000, () => console.log('Server running on port 3000'));
*/

// Express.js + SSE Example
/*
const express = require('express');
const app = express();

const clients = [];

// SSE endpoint
app.get('/api/webhook-stream', (req, res) => {
    res.setHeader('Content-Type', 'text/event-stream');
    res.setHeader('Cache-Control', 'no-cache');
    res.setHeader('Connection', 'keep-alive');
    
    clients.push(res);
    
    req.on('close', () => {
        clients.splice(clients.indexOf(res), 1);
    });
});

// Webhook endpoint
app.post('/webhook', express.json(), (req, res) => {
    const event = {
        id: Date.now(),
        type: req.body.type,
        timestamp: new Date().toISOString(),
        payload: req.body.data
    };
    
    // Send to all SSE clients
    clients.forEach(client => {
        client.write(`data: ${JSON.stringify(event)}\n\n`);
    });
    
    res.status(200).json({ success: true });
});

app.listen(3000, () => console.log('Server running on port 3000'));
*/

// ============================================================================
// SECURITY MIDDLEWARE EXAMPLE
// ============================================================================

function sanitizeWebhookPayload(payload) {
    const sanitized = { ...payload };
    
    // Redact email addresses
    if (sanitized.email) {
        const [local, domain] = sanitized.email.split('@');
        sanitized.email = `${local.substring(0, 2)}***@${domain}`;
    }
    
    // Truncate addresses
    if (sanitized.user) {
        sanitized.user = `${sanitized.user.substring(0, 8)}...${sanitized.user.substring(sanitized.user.length - 3)}`;
    }
    
    // Remove sensitive fields
    delete sanitized.password;
    delete sanitized.apiKey;
    delete sanitized.secret;
    
    return sanitized;
}

// ============================================================================
// STELLAR HORIZON INTEGRATION EXAMPLE
// ============================================================================

/*
const StellarSdk = require('stellar-sdk');

function monitorStellarAccount(accountId) {
    const server = new StellarSdk.Server('https://horizon-testnet.stellar.org');
    
    server.operations()
        .forAccount(accountId)
        .cursor('now')
        .stream({
            onmessage: (operation) => {
                const event = {
                    id: operation.id,
                    type: mapOperationType(operation.type),
                    timestamp: operation.created_at,
                    payload: {
                        operation_type: operation.type,
                        from: operation.from,
                        to: operation.to,
                        amount: operation.amount,
                        asset: operation.asset_type
                    }
                };
                
                // Send to webhook monitor
                broadcastEvent(event);
            },
            onerror: (error) => {
                console.error('Stellar stream error:', error);
            }
        });
}

function mapOperationType(stellarType) {
    const typeMap = {
        'payment': 'transfer',
        'create_account': 'deposit',
        'path_payment_strict_receive': 'transfer',
        'path_payment_strict_send': 'transfer'
    };
    return typeMap[stellarType] || 'other';
}
*/

// ============================================================================
// USAGE IN webhook_monitor.html
// ============================================================================

/*
Replace the simulation code in webhook_monitor.html with:

<script>
    // Choose your integration method
    const monitor = new WebhookMonitorWebSocket('ws://localhost:8080/webhooks');
    // OR
    // const monitor = new WebhookMonitorSSE('/api/webhook-stream');
    // OR
    // const monitor = new WebhookMonitorPolling('/api/webhooks/recent', 3000);
    
    // Start monitoring
    monitor.connect(); // or monitor.start() for polling
    
    // Clean up on page unload
    window.addEventListener('beforeunload', () => {
        monitor.disconnect(); // or monitor.stop() for polling
    });
</script>
*/
