# Unique Request ID Implementation Summary

This document summarizes the implementation of end-to-end request IDs for traceability in the SwiftRemit system.

## Implementation Overview
Every operation in the system is now associated with a unique Request ID (UUID v4). This ID is generated at the start of a request and propagated through all layers, including logging and contract responses.

### Key Components
1. **Dynamic Request ID Generation**:
   - Clients automatically generate a UUID v4 using the `uuid` library.
   - External IDs can be provided via the `REQUEST_ID` environment variable.

2. **Contract Responses ([response.rs](file:///c:/Users/user/SwiftRemit/src/response.rs))**:
   - The standardized `Response<T>` struct now includes a `request_id: String` field.
   - All query-style functions return this structured response.

3. **Traceability Points**:
   - New `query_remittance` function in `src/lib.rs` demonstrates ID propagation.
   - The centralized logger attaches the current `request_id` to every log entry.

## Traceability Flow
1. **Client** generates `requestId` (e.g., `b3d8...`).
2. **Client** logs the start of the operation with `requestId` in the JSON context.
3. **Client** calls the contract, passing the `requestId` as an argument.
4. **Contract** processes the request and returns a `Response` containing the `requestId`.
5. **Client** receives the response and logs the result, verifying the `requestId`.

## Verification
To verify end-to-end traceability, filter logs by the `request_id` field:
```bash
grep "b3d8..." app.log
```

You can also pass a custom ID for external tracking:
`$env:REQUEST_ID="EXTERNAL-123"; node examples/client-example.js`
