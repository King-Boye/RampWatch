import dotenv from 'dotenv';
import app from './api';
import { initDatabase } from './database';
import { startBackgroundJobs } from './scheduler';

dotenv.config();

const PORT = process.env.PORT || 3000;

async function start() {
  try {
    // Initialize database
    await initDatabase();
    console.log('Database initialized');

    // Start background jobs
    startBackgroundJobs();

    // Start API server
    app.listen(PORT, () => {
      console.log(`SwiftRemit Verification Service running on port ${PORT}`);
      console.log(`Environment: ${process.env.NODE_ENV || 'development'}`);
    });
  } catch (error) {
    console.error('Failed to start server:', error);
    process.exit(1);
  }
}

start();
