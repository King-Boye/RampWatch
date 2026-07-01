#!/usr/bin/env python3
"""Minimal mock server for integration testing.

Implements a small subset of SEP-6 and SEP-10 endpoints using the Python
standard library (no external deps).

Supported endpoints:
- POST /deposit
- POST /withdraw
- POST /transaction
- POST /auth

Also supports:
- OPTIONS preflight for CORS
- GET /health

The mock is intentionally lenient about input format (JSON body + query
params) to accommodate slightly different client implementations.
"""

from __future__ import annotations

import json
import threading
from http.server import BaseHTTPRequestHandler, HTTPServer
from urllib.parse import parse_qs, urlparse


class _State:
    def __init__(self):
        self._lock = threading.Lock()
        self._next_tx = 1
        # tx_id -> dict (transaction_id, kind?, status, amounts, message)
        self._tx = {}

    def new_tx_id(self) -> str:
        with self._lock:
            tx_id = f"txn-{self._next_tx:06d}"
            self._next_tx += 1
            return tx_id

    def put_tx(self, tx_id: str, record: dict) -> None:
        with self._lock:
            self._tx[tx_id] = record

    def get_tx(self, tx_id: str) -> dict | None:
        with self._lock:
            return self._tx.get(tx_id)


STATE = _State()


def _json_body(handler: BaseHTTPRequestHandler) -> dict:
    length = int(handler.headers.get("Content-Length") or 0)
    if length <= 0:
        return {}
    raw = handler.rfile.read(length)
    if not raw:
        return {}
    try:
        return json.loads(raw.decode("utf-8"))
    except Exception:
        return {}


def _send_json(handler: BaseHTTPRequestHandler, status_code: int, payload: dict) -> None:
    body = json.dumps(payload).encode("utf-8")
    handler.send_response(status_code)
    handler.send_header("Access-Control-Allow-Origin", "*")
    handler.send_header("Access-Control-Allow-Headers", "Content-Type")
    handler.send_header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
    handler.send_header("Content-Type", "application/json")
    handler.send_header("Content-Length", str(len(body)))
    handler.end_headers()
    handler.wfile.write(body)


def _parse_query(handler: BaseHTTPRequestHandler) -> dict:
    parsed = urlparse(handler.path)
    q = parse_qs(parsed.query)
    # flatten single values
    out = {}
    for k, v in q.items():
        out[k] = v[0] if isinstance(v, list) and v else ""
    return out


class MockAnchorHandler(BaseHTTPRequestHandler):
    server_version = "MockAnchor/1.0"

    def log_message(self, fmt, *args):
        # Keep tests quiet
        return

    def _handle_options(self):
        self.send_response(200)
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Headers", "Content-Type")
        self.send_header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        self.end_headers()

    def do_OPTIONS(self):
        self._handle_options()

    def do_GET(self):
        parsed = urlparse(self.path)
        if parsed.path == "/health":
            _send_json(self, 200, {"status": "ok"})
            return
        _send_json(self, 404, {"error": "not_found", "path": parsed.path})

    def do_POST(self):
        parsed = urlparse(self.path)
        path = parsed.path
        query = _parse_query(self)
        body = _json_body(self)

        # Convenience: allow setting tx status through either body or query.
        force_status = (
            body.get("status")
            or body.get("transaction_status")
            or query.get("status")
            or query.get("transaction_status")
        )

        if path == "/deposit":
            self._post_deposit(body, query, force_status)
            return
        if path == "/withdraw":
            self._post_withdraw(body, query, force_status)
            return
        if path == "/transaction":
            self._post_transaction(body, query, force_status)
            return
        if path == "/auth":
            self._post_auth(body, query)
            return

        _send_json(self, 404, {"error": "not_found", "path": path})

    def _post_deposit(self, body: dict, query: dict, force_status: str | None) -> None:
        tx_id = body.get("transaction_id") or query.get("transaction_id") or STATE.new_tx_id()
        how = body.get("how") or query.get("how") or "Send to mock deposit address"

        status = force_status or body.get("status") or "pending_external"

        record = {
            "transaction_id": tx_id,
            "kind": "deposit",
            "status": status,
            "amount_in": body.get("amount") or body.get("amount_in"),
            "amount_out": body.get("amount_out"),
            "amount_fee": body.get("fee") or body.get("amount_fee"),
            "message": body.get("message") or None,
        }
        STATE.put_tx(tx_id, record)

        resp = {
            "transaction_id": tx_id,
            "how": how,
            "extra_info": body.get("extra_info") or query.get("extra_info"),
            "min_amount": body.get("min_amount") or query.get("min_amount"),
            "max_amount": body.get("max_amount") or query.get("max_amount"),
            "fee_fixed": body.get("fee_fixed") or query.get("fee_fixed"),
            "fee_percent": body.get("fee_percent") or query.get("fee_percent"),
            "status": status,
        }

        # Strip null-like values to keep responses closer to real anchors
        resp = {k: v for k, v in resp.items() if v is not None}
        _send_json(self, 200, resp)

    def _post_withdraw(self, body: dict, query: dict, force_status: str | None) -> None:
        tx_id = body.get("transaction_id") or query.get("transaction_id") or STATE.new_tx_id()
        account_id = body.get("account_id") or query.get("account_id") or "GABC1234567890MOCKMOCKMOCKMOCKMOCKMOCKMOCKMOCKMOCK"[:56]

        status = force_status or body.get("status") or "pending_user"

        record = {
            "transaction_id": tx_id,
            "kind": "withdraw",
            "status": status,
            "amount_in": body.get("amount") or body.get("amount_in"),
            "amount_out": body.get("amount_out"),
            "amount_fee": body.get("fee") or body.get("amount_fee"),
            "message": body.get("message") or None,
        }
        STATE.put_tx(tx_id, record)

        resp = {
            "transaction_id": tx_id,
            "account_id": account_id,
            "dest_account_id": body.get("dest_account_id") or query.get("dest_account_id"),
            "memo": body.get("memo") or query.get("memo"),
            "memo_type": body.get("memo_type") or query.get("memo_type"),
            "min_amount": body.get("min_amount") or query.get("min_amount"),
            "max_amount": body.get("max_amount") or query.get("max_amount"),
            "fee_fixed": body.get("fee_fixed") or query.get("fee_fixed"),
            "fee_percent": body.get("fee_percent") or query.get("fee_percent"),
            "status": status,
        }
        resp = {k: v for k, v in resp.items() if v is not None}
        _send_json(self, 200, resp)

    def _post_transaction(self, body: dict, query: dict, force_status: str | None) -> None:
        tx_id = (
            body.get("transaction_id")
            or body.get("id")
            or query.get("transaction_id")
            or query.get("id")
        )
        if not tx_id:
            _send_json(self, 400, {"error": "missing_transaction_id"})
            return

        record = STATE.get_tx(tx_id)
        if record is None:
            # Mimic a typical anchor behavior: not found
            _send_json(self, 404, {"error": "not_found", "transaction_id": tx_id})
            return

        if force_status is not None:
            record = dict(record)
            record["status"] = force_status
            STATE.put_tx(tx_id, record)

        resp = {
            "transaction_id": tx_id,
            "kind": record.get("kind"),
            "status": record.get("status", "pending"),
            "amount_in": record.get("amount_in"),
            "amount_out": record.get("amount_out"),
            "amount_fee": record.get("amount_fee"),
            "message": record.get("message"),
        }
        resp = {k: v for k, v in resp.items() if v is not None}
        _send_json(self, 200, resp)

    def _post_auth(self, body: dict, query: dict) -> None:
        # Extremely forgiving mock for SEP-10 /auth.
        # We return a plausible challenge/envelope structure.
        # Some clients may just expect 200 + a JWT-like string.

        # If client provides a challenge/anchor info, reflect it.
        envelope = body.get("envelope") or query.get("envelope")
        client_domain = (
            body.get("clientDomain")
            or body.get("homeDomain")
            or query.get("clientDomain")
            or query.get("homeDomain")
            or "https://client.example.com"
        )

        # Some flows request the anchor to produce a token/challenge; keep it deterministic.
        challenge = body.get("challenge") or query.get("challenge") or "mock-challenge"
        sep10_token = body.get("sep10_token") or query.get("sep10_token") or "mock-sep10-jwt"

        resp = {
            "envelope": envelope if envelope is not None else {
                "clientDomain": client_domain,
                "challenge": challenge,
            },
            "challenge": challenge,
            "jwt": sep10_token,
            # Include commonly used top-level fields defensively
            "token": sep10_token,
            "success": True,
        }

        _send_json(self, 200, resp)


def main() -> None:
    host = "0.0.0.0"
    port = 8080
    print(f"Mock anchor server running on http://localhost:{port}")
    HTTPServer((host, port), MockAnchorHandler).serve_forever()


if __name__ == "__main__":
    main()

