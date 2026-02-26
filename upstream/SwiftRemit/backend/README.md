# SwiftRemit Verification Service

Backend service for asset verification in the SwiftRemit remittance platform.

## Quick Start

```bash
# Install dependencies
npm install

# Setup environment
cp .env.example .env
# Edit .env with your configuration

# Start PostgreSQL
# Ensure PostgreSQL is running on localhost:5432

# Run development server
npm run dev

# Build for production
npm run build
npm start
```

## API Documentation

See [ASSET_VERIFICATION.md](../ASSET_VERIFICATION.md) for complete API documentation.

## Testing

```bash
npm test
```

## Project Structure

```
backend/
├── src/
│   ├── api.ts          # Express API routes
│   ├── database.ts     # PostgreSQL operations
│   ├── index.ts        # Entry point
│   ├── scheduler.ts    # Background jobs
│   ├── stellar.ts      # Soroban contract integration
│   ├── types.ts        # TypeScript types
│   └── verifier.ts     # Asset verification logic
├── .env.example        # Environment template
├── package.json
├── tsconfig.json
└── README.md
```

## Environment Variables

See `.env.example` for all configuration options.

## License

MIT
