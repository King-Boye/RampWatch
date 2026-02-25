import dotenv from 'dotenv';
import { createApp } from './app';
import { initializeCurrencyConfig } from './config';

// Load environment variables
dotenv.config();

const PORT = process.env.PORT || 3000;

async function start() {
  try {
    // Initialize and validate currency configuration (fail fast)
    console.log('Initializing currency configuration...');
    initializeCurrencyConfig();

    // Create and start Express app
    const app = createApp();

    app.listen(PORT, () => {
      console.log(`✓ SwiftRemit API server running on port ${PORT}`);
      console.log(`✓ Environment: ${process.env.NODE_ENV || 'development'}`);
      console.log(`✓ Health check: http://localhost:${PORT}/health`);
      console.log(`✓ Currencies API: http://localhost:${PORT}/api/currencies`);
    });
  } catch (error) {
    console.error('✗ Failed to start server:', error);
    console.error('✗ Server startup aborted due to configuration error');
    process.exit(1); // Fail fast
  }
}

// Handle uncaught errors
process.on('uncaughtException', (error) => {
  console.error('Uncaught exception:', error);
  process.exit(1);
});

process.on('unhandledRejection', (reason, promise) => {
  console.error('Unhandled rejection at:', promise, 'reason:', reason);
  process.exit(1);
});

start();
