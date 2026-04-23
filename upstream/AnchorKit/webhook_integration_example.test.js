/**
 * Tests for sanitizeWebhookPayload — allowlist-based field filtering.
 *
 * Run with: node --test webhook_integration_example.test.js
 * (Node.js 18+ built-in test runner, no extra deps required)
 */

const { describe, it, beforeEach, afterEach } = require('node:test');
const assert = require('node:assert/strict');

// ---------------------------------------------------------------------------
// Inline the allowlist + function under test so this file is self-contained.
// If the project adds a module.exports to webhook_integration_example.js,
// replace the block below with:
//   const { sanitizeWebhookPayload, WEBHOOK_PAYLOAD_ALLOWLIST } =
//       require('./webhook_integration_example');
// ---------------------------------------------------------------------------

const WEBHOOK_PAYLOAD_ALLOWLIST = new Set([
    'id', 'type', 'timestamp', 'status', 'amount', 'asset',
    'user', 'email', 'memo', 'transaction_id', 'account',
    'network', 'fee', 'message',
]);

function sanitizeWebhookPayload(payload) {
    if (!payload || typeof payload !== 'object' || Array.isArray(payload)) {
        return {};
    }
    const sanitized = {};
    for (const key of Object.keys(payload)) {
        if (!WEBHOOK_PAYLOAD_ALLOWLIST.has(key)) {
            console.warn(`WARN: Unknown field detected in webhook payload: "${key}"`);
            continue;
        }
        let value = payload[key];
        if (key === 'email' && typeof value === 'string' && value.includes('@')) {
            const [local, domain] = value.split('@');
            value = `${local.substring(0, 2)}***@${domain}`;
        }
        if (key === 'user' && typeof value === 'string' && value.length > 11) {
            value = `${value.substring(0, 8)}...${value.substring(value.length - 3)}`;
        }
        sanitized[key] = value;
    }
    return sanitized;
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

let warnMessages = [];

function captureWarnings() {
    warnMessages = [];
    const original = console.warn;
    console.warn = (...args) => { warnMessages.push(args.join(' ')); };
    return () => { console.warn = original; };
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe('sanitizeWebhookPayload', () => {

    describe('payload with only known fields passes through cleanly', () => {
        it('returns all allowlisted fields unchanged (non-sensitive)', () => {
            const payload = {
                id: 'evt_001',
                type: 'deposit',
                status: 'completed',
                amount: '100.00',
                asset: 'USDC',
                network: 'stellar',
            };
            const result = sanitizeWebhookPayload(payload);
            assert.deepEqual(result, payload);
        });

        it('redacts email to partial form', () => {
            const payload = { id: 'evt_002', email: 'alice@example.com' };
            const result = sanitizeWebhookPayload(payload);
            assert.equal(result.email, 'al***@example.com');
            assert.equal(result.id, 'evt_002');
        });

        it('truncates long user/account strings', () => {
            const payload = { user: 'GABCDEFGHIJKLMNOP' };
            const result = sanitizeWebhookPayload(payload);
            assert.match(result.user, /^.{8}\.{3}.{3}$/);
        });
    });

    describe('payload with unknown fields — strips them and logs warnings', () => {
        let restore;
        beforeEach(() => { restore = captureWarnings(); });
        afterEach(() => restore());

        it('strips unknown fields from the result', () => {
            // Use Object.assign to add __proto__ as a real own key (prototype pollution test)
            const payload = Object.assign(Object.create(null), {
                id: 'evt_003',
                type: 'transfer',
                injectedField: '<script>alert(1)</script>',
                apiKey: 'sk-secret',
            });
            // Manually add __proto__ as an own enumerable key
            Object.defineProperty(payload, '__proto__', { value: 'polluted', enumerable: true });

            const result = sanitizeWebhookPayload(payload);
            assert.ok(!('injectedField' in result), 'injectedField must be stripped');
            assert.ok(!('apiKey' in result), 'apiKey must be stripped');
            assert.ok(!Object.prototype.hasOwnProperty.call(result, '__proto__'), '__proto__ own key must be stripped');
            assert.equal(result.id, 'evt_003');
            assert.equal(result.type, 'transfer');
        });

        it('logs a WARN for each unknown field (field name only, not value)', () => {
            sanitizeWebhookPayload({ id: 'evt_004', password: 'hunter2', injected: 'bad' });
            assert.equal(warnMessages.length, 2);
            assert.ok(warnMessages.some(m => m.includes('"password"')));
            assert.ok(warnMessages.some(m => m.includes('"injected"')));
            // Values must NOT appear in the log
            assert.ok(!warnMessages.some(m => m.includes('hunter2')));
            assert.ok(!warnMessages.some(m => m.includes('bad')));
        });
    });

    describe('edge cases', () => {
        let restore;
        beforeEach(() => { restore = captureWarnings(); });
        afterEach(() => restore());

        it('handles empty payload gracefully', () => {
            const result = sanitizeWebhookPayload({});
            assert.deepEqual(result, {});
            assert.equal(warnMessages.length, 0);
        });

        it('handles null gracefully', () => {
            assert.deepEqual(sanitizeWebhookPayload(null), {});
        });

        it('handles undefined gracefully', () => {
            assert.deepEqual(sanitizeWebhookPayload(undefined), {});
        });

        it('handles array input gracefully', () => {
            assert.deepEqual(sanitizeWebhookPayload([{ id: 'x' }]), {});
        });

        it('payload with ALL unknown fields returns empty object', () => {
            const payload = { foo: 1, bar: 2, baz: 3 };
            const result = sanitizeWebhookPayload(payload);
            assert.deepEqual(result, {});
            assert.equal(warnMessages.length, 3);
        });
    });
});

// ---------------------------------------------------------------------------
// WebhookMonitorWebSocket — reconnect limit tests
// ---------------------------------------------------------------------------

// Minimal WebSocket stub — records calls, exposes onclose/onopen triggers
class FakeWebSocket {
    constructor() {
        FakeWebSocket.instances.push(this);
        this.onopen = null;
        this.onclose = null;
        this.onerror = null;
        this.onmessage = null;
    }
    close() {}
    static reset() { FakeWebSocket.instances = []; }
}
FakeWebSocket.instances = [];

// Inline the updated class under test (mirrors webhook_integration_example.js)
class WebhookMonitorWebSocket {
    constructor(wsUrl, options = {}) {
        this.wsUrl = wsUrl;
        this.ws = null;
        this.reconnectCount = 0;
        this.maxReconnectAttempts = options.maxReconnectAttempts ?? 10;
        this._listeners = {};
    }
    on(event, listener) {
        if (!this._listeners[event]) this._listeners[event] = [];
        this._listeners[event].push(listener);
        return this;
    }
    emit(event, ...args) {
        (this._listeners[event] || []).forEach(fn => fn(...args));
    }
    connect() {
        this.ws = new FakeWebSocket();
        this.ws.onopen = () => { this.reconnectCount = 0; };
        this.ws.onclose = () => { this.attemptReconnect(); };
        this.ws.onerror = () => {};
    }
    attemptReconnect() {
        if (this.reconnectCount >= this.maxReconnectAttempts) {
            if (this.reconnectCount === this.maxReconnectAttempts) {
                this.emit('max_reconnects_exceeded', { attempts: this.reconnectCount });
                this.reconnectCount++; // sentinel: silence further onclose calls
            }
            return;
        }
        this.reconnectCount++;
        this._reconnectTimer = setTimeout(() => this.connect(), 0);
    }
}

// Fake clock helpers — replace setTimeout with synchronous flush
let pendingTimers = [];
const realSetTimeout = global.setTimeout;

function installFakeClock() {
    pendingTimers = [];
    global.setTimeout = (fn) => { pendingTimers.push(fn); };
}
function flushTimers() {
    while (pendingTimers.length) {
        const fn = pendingTimers.shift();
        fn();
    }
}
function uninstallFakeClock() {
    global.setTimeout = realSetTimeout;
    pendingTimers = [];
}

describe('WebhookMonitorWebSocket — reconnect limit', () => {

    beforeEach(() => {
        FakeWebSocket.reset();
        installFakeClock();
    });
    afterEach(() => {
        uninstallFakeClock();
    });

    it('stops reconnecting after hitting maxReconnectAttempts (default 10)', () => {
        const monitor = new WebhookMonitorWebSocket('ws://example.com');
        monitor.connect();

        // Simulate server permanently down: trigger onclose repeatedly
        for (let i = 0; i < 15; i++) {
            const latest = FakeWebSocket.instances[FakeWebSocket.instances.length - 1];
            if (latest && latest.onclose) latest.onclose();
            flushTimers();
        }

        // reconnectCount is maxReconnectAttempts+1 due to sentinel increment after event fires
        assert.ok(
            monitor.reconnectCount >= monitor.maxReconnectAttempts,
            'reconnectCount must reach the limit'
        );
        // Total WebSocket instances = 1 initial + 10 retries = 11
        assert.equal(FakeWebSocket.instances.length, 11);
    });

    it('emits max_reconnects_exceeded with correct attempt count', () => {
        const monitor = new WebhookMonitorWebSocket('ws://example.com', { maxReconnectAttempts: 3 });
        let eventPayload = null;
        monitor.on('max_reconnects_exceeded', (data) => { eventPayload = data; });

        monitor.connect();
        for (let i = 0; i < 5; i++) {
            const latest = FakeWebSocket.instances[FakeWebSocket.instances.length - 1];
            if (latest && latest.onclose) latest.onclose();
            flushTimers();
        }

        assert.ok(eventPayload !== null, 'max_reconnects_exceeded must have fired');
        assert.equal(eventPayload.attempts, 3);
    });

    it('emits max_reconnects_exceeded exactly once', () => {
        const monitor = new WebhookMonitorWebSocket('ws://example.com', { maxReconnectAttempts: 2 });
        let fireCount = 0;
        monitor.on('max_reconnects_exceeded', () => { fireCount++; });

        monitor.connect();
        for (let i = 0; i < 6; i++) {
            const latest = FakeWebSocket.instances[FakeWebSocket.instances.length - 1];
            if (latest && latest.onclose) latest.onclose();
            flushTimers();
        }

        assert.equal(fireCount, 1);
    });

    it('resets reconnectCount to 0 on successful reconnect', () => {
        const monitor = new WebhookMonitorWebSocket('ws://example.com', { maxReconnectAttempts: 5 });
        monitor.connect();

        // Simulate 3 failed attempts
        for (let i = 0; i < 3; i++) {
            const latest = FakeWebSocket.instances[FakeWebSocket.instances.length - 1];
            if (latest && latest.onclose) latest.onclose();
            flushTimers();
        }
        assert.equal(monitor.reconnectCount, 3);

        // Simulate successful reconnect (onopen fires)
        const latest = FakeWebSocket.instances[FakeWebSocket.instances.length - 1];
        if (latest && latest.onopen) latest.onopen();

        assert.equal(monitor.reconnectCount, 0, 'reconnectCount must reset to 0 after successful connection');
    });

    it('respects a custom maxReconnectAttempts value', () => {
        const monitor = new WebhookMonitorWebSocket('ws://example.com', { maxReconnectAttempts: 2 });
        assert.equal(monitor.maxReconnectAttempts, 2);

        monitor.connect();
        for (let i = 0; i < 10; i++) {
            const latest = FakeWebSocket.instances[FakeWebSocket.instances.length - 1];
            if (latest && latest.onclose) latest.onclose();
            flushTimers();
        }

        // 1 initial + 2 retries = 3 total WebSocket instances
        assert.equal(FakeWebSocket.instances.length, 3);
        // reconnectCount is maxReconnectAttempts+1 (sentinel) after limit is hit
        assert.ok(monitor.reconnectCount >= 2);
    });

    it('uses default maxReconnectAttempts of 10 when no options passed', () => {
        const monitor = new WebhookMonitorWebSocket('ws://example.com');
        assert.equal(monitor.maxReconnectAttempts, 10);
    });
});
