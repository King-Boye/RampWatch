import { Router, Request, Response } from 'express';
import { getCurrencyConfigLoader } from '../config';
import { CurrencyResponse, ErrorResponse } from '../types';

const router = Router();

/**
 * GET /api/currencies
 * Returns all supported currencies with their formatting rules
 */
router.get('/', (req: Request, res: Response) => {
  try {
    const configLoader = getCurrencyConfigLoader();
    const currencies = configLoader.getCurrencies();

    const response: CurrencyResponse = {
      success: true,
      data: currencies,
      count: currencies.length,
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  } catch (error) {
    const errorResponse: ErrorResponse = {
      success: false,
      error: {
        message: error instanceof Error ? error.message : 'Failed to retrieve currencies',
        code: 'CURRENCY_RETRIEVAL_ERROR',
      },
      timestamp: new Date().toISOString(),
    };

    res.status(500).json(errorResponse);
  }
});

/**
 * GET /api/currencies/:code
 * Returns a specific currency by code
 */
router.get('/:code', (req: Request, res: Response) => {
  try {
    const { code } = req.params;

    if (!code || typeof code !== 'string') {
      const errorResponse: ErrorResponse = {
        success: false,
        error: {
          message: 'Currency code is required',
          code: 'INVALID_CURRENCY_CODE',
        },
        timestamp: new Date().toISOString(),
      };
      return res.status(400).json(errorResponse);
    }

    const configLoader = getCurrencyConfigLoader();
    const currency = configLoader.getCurrencyByCode(code);

    if (!currency) {
      const errorResponse: ErrorResponse = {
        success: false,
        error: {
          message: `Currency not found: ${code.toUpperCase()}`,
          code: 'CURRENCY_NOT_FOUND',
        },
        timestamp: new Date().toISOString(),
      };
      return res.status(404).json(errorResponse);
    }

    const response: CurrencyResponse = {
      success: true,
      data: [currency],
      count: 1,
      timestamp: new Date().toISOString(),
    };

    res.json(response);
  } catch (error) {
    const errorResponse: ErrorResponse = {
      success: false,
      error: {
        message: error instanceof Error ? error.message : 'Failed to retrieve currency',
        code: 'CURRENCY_RETRIEVAL_ERROR',
      },
      timestamp: new Date().toISOString(),
    };

    res.status(500).json(errorResponse);
  }
});

export default router;
