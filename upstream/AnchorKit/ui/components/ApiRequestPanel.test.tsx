import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { ApiRequestPanel, HistoryEntry } from './ApiRequestPanel';

describe('ApiRequestPanel', () => {
  beforeEach(() => {
    localStorage.clear();
  });

  afterEach(() => {
    localStorage.clear();
  });

  describe('History Recording - Consecutive Requests Bug Fix', () => {
    it('should record consecutive requests with identical error objects', async () => {
      const sharedError = 'Network error';
      const { rerender } = render(
        <ApiRequestPanel
          endpoint="https://api.example.com/users"
          method="POST"
          persistHistory={true}
          error={sharedError}
        />
      );

      const toggleButton = screen.getAllByTitle(/show history|hide history/i)[0];
      fireEvent.click(toggleButton);

      await waitFor(() => {
        expect(screen.getByRole('list')).toBeInTheDocument();
      });

      const initialItems = screen.queryAllByRole('listitem');
      expect(initialItems.length).toBe(1);

      // Rerender with same error object - should still record
      rerender(
        <ApiRequestPanel
          endpoint="https://api.example.com/users"
          method="POST"
          persistHistory={true}
          error={sharedError}
        />
      );

      // Verify second request was recorded despite identical error reference
      const historyItems = screen.queryAllByRole('listitem');
      expect(historyItems.length).toBe(2);
    });

    it('should record consecutive requests with reference-identical responses', async () => {
      const sharedResponse = { status: 'ok' };
      const { rerender } = render(
        <ApiRequestPanel
          endpoint="https://api.example.com/status"
          method="GET"
          persistHistory={true}
          response={sharedResponse}
        />
      );

      const toggleButton = screen.getAllByTitle(/show history|hide history/i)[0];
      fireEvent.click(toggleButton);

      await waitFor(() => {
        expect(screen.getByRole('list')).toBeInTheDocument();
      });

      const initialItems = screen.queryAllByRole('listitem');
      expect(initialItems.length).toBe(1);

      // Rerender with same response object
      rerender(
        <ApiRequestPanel
          endpoint="https://api.example.com/status"
          method="GET"
          persistHistory={true}
          response={sharedResponse}
        />
      );

      // Should have recorded both requests
      const historyItems = screen.queryAllByRole('listitem');
      expect(historyItems.length).toBe(2);
    });

    it('should record requests when endpoint changes even with identical response', async () => {
      const sharedResponse = { data: 'cached' };
      const { rerender } = render(
        <ApiRequestPanel
          endpoint="https://api.example.com/endpoint1"
          method="GET"
          persistHistory={true}
          response={sharedResponse}
        />
      );

      const toggleButton = screen.getAllByTitle(/show history|hide history/i)[0];
      fireEvent.click(toggleButton);

      await waitFor(() => {
        expect(screen.getByText('https://api.example.com/endpoint1')).toBeInTheDocument();
      });

      const initialItems = screen.queryAllByRole('listitem');
      expect(initialItems.length).toBe(1);

      // Change endpoint with same response
      rerender(
        <ApiRequestPanel
          endpoint="https://api.example.com/endpoint2"
          method="GET"
          persistHistory={true}
          response={sharedResponse}
        />
      );

      // Should have recorded both requests
      const historyItems = screen.queryAllByRole('listitem');
      expect(historyItems.length).toBe(2);
    });

    it('should record requests when method changes with identical response', async () => {
      const sharedResponse = { success: true };
      const { rerender } = render(
        <ApiRequestPanel
          endpoint="https://api.example.com/resource"
          method="GET"
          persistHistory={true}
          response={sharedResponse}
        />
      );

      const toggleButton = screen.getAllByTitle(/show history|hide history/i)[0];
      fireEvent.click(toggleButton);

      await waitFor(() => {
        expect(screen.getByRole('list')).toBeInTheDocument();
      });

      const initialItems = screen.queryAllByRole('listitem');
      expect(initialItems.length).toBe(1);

      // Change method with same response
      rerender(
        <ApiRequestPanel
          endpoint="https://api.example.com/resource"
          method="POST"
          persistHistory={true}
          response={sharedResponse}
        />
      );

      // Should have recorded both requests
      const historyItems = screen.queryAllByRole('listitem');
      expect(historyItems.length).toBe(2);
    });
  });

  describe('History Persistence', () => {
    it('should persist history to localStorage', async () => {
      render(
        <ApiRequestPanel
          endpoint="https://api.example.com/test"
          method="POST"
          persistHistory={true}
          response={{ id: 1 }}
        />
      );

      const stored = localStorage.getItem('anchorkit_api_history');
      expect(stored).toBeTruthy();
      const history: HistoryEntry[] = JSON.parse(stored!);
      expect(history.length).toBeGreaterThan(0);
      expect(history[0].endpoint).toBe('https://api.example.com/test');
    });

    it('should limit history to 20 entries', () => {
      const { rerender } = render(
        <ApiRequestPanel
          endpoint="https://api.example.com/test"
          method="GET"
          persistHistory={true}
          response={{ count: 1 }}
        />
      );

      // Simulate 25 requests
      for (let i = 2; i <= 25; i++) {
        rerender(
          <ApiRequestPanel
            endpoint={`https://api.example.com/test${i}`}
            method="GET"
            persistHistory={true}
            response={{ count: i }}
          />
        );
      }

      const stored = localStorage.getItem('anchorkit_api_history');
      const history: HistoryEntry[] = JSON.parse(stored!);
      expect(history.length).toBeLessThanOrEqual(20);
    });

    it('should not persist history when persistHistory is false', () => {
      render(
        <ApiRequestPanel
          endpoint="https://api.example.com/test"
          method="GET"
          persistHistory={false}
          response={{ data: 'test' }}
        />
      );

      const stored = localStorage.getItem('anchorkit_api_history');
      expect(stored).toBeNull();
    });
  });

  describe('History Display and Interaction', () => {
    it('should toggle history visibility', async () => {
      render(
        <ApiRequestPanel
          endpoint="https://api.example.com/test"
          method="GET"
          persistHistory={true}
          response={{ data: 'test' }}
        />
      );

      // History should be hidden initially
      expect(screen.queryByRole('list')).not.toBeInTheDocument();

      // Click toggle to show
      const toggleButton = screen.getAllByTitle(/show history|hide history/i)[0];
      fireEvent.click(toggleButton);

      await waitFor(() => {
        expect(screen.getByRole('list')).toBeInTheDocument();
      });

      // Click toggle to hide
      fireEvent.click(toggleButton);

      await waitFor(() => {
        expect(screen.queryByRole('list')).not.toBeInTheDocument();
      });
    });

    it('should display history entries with method badge and endpoint', async () => {
      render(
        <ApiRequestPanel
          endpoint="https://api.example.com/users"
          method="POST"
          persistHistory={true}
          response={{ id: 1 }}
        />
      );

      // Expand history
      const toggleButton = screen.getAllByTitle(/show history|hide history/i)[0];
      fireEvent.click(toggleButton);

      await waitFor(() => {
        expect(screen.getByText('POST')).toBeInTheDocument();
        expect(screen.getByText('https://api.example.com/users')).toBeInTheDocument();
      });
    });

    it('should clear history when clear button is clicked', async () => {
      render(
        <ApiRequestPanel
          endpoint="https://api.example.com/test"
          method="GET"
          persistHistory={true}
          response={{ data: 'test' }}
        />
      );

      const clearButton = screen.getByTitle('Clear history');
      fireEvent.click(clearButton);

      const stored = localStorage.getItem('anchorkit_api_history');
      const history: HistoryEntry[] = JSON.parse(stored || '[]');
      expect(history.length).toBe(0);
    });

    it('should show error icon in history for failed requests', async () => {
      render(
        <ApiRequestPanel
          endpoint="https://api.example.com/test"
          method="GET"
          persistHistory={true}
          error="Connection timeout"
        />
      );

      // Expand history
      const toggleButton = screen.getAllByTitle(/show history|hide history/i)[0];
      fireEvent.click(toggleButton);

      await waitFor(() => {
        const errorIcons = screen.getAllByText('⚠️');
        expect(errorIcons.length).toBeGreaterThan(0);
      });
    });
  });

  describe('Sensitive Data Redaction', () => {
    it('should redact sensitive fields by default', async () => {
      const sensitiveData = {
        name: 'John',
        token: 'secret-token-123',
        password: 'my-password',
      };

      render(
        <ApiRequestPanel
          endpoint="https://api.example.com/login"
          method="POST"
          persistHistory={true}
          requestBody={sensitiveData}
          response={{ success: true }}
        />
      );

      const stored = localStorage.getItem('anchorkit_api_history');
      const history: HistoryEntry[] = JSON.parse(stored!);
      expect(history[0].requestBody).toEqual({
        name: 'John',
        token: '[REDACTED]',
        password: '[REDACTED]',
      });
    });

    it('should use custom sensitiveFields', async () => {
      const data = {
        name: 'John',
        customSecret: 'should-be-redacted',
        normalField: 'ok',
      };

      render(
        <ApiRequestPanel
          endpoint="https://api.example.com/test"
          method="POST"
          persistHistory={true}
          requestBody={data}
          response={{ success: true }}
          sensitiveFields={['customSecret']}
        />
      );

      const stored = localStorage.getItem('anchorkit_api_history');
      const history: HistoryEntry[] = JSON.parse(stored!);
      expect(history[0].requestBody).toEqual({
        name: 'John',
        customSecret: '[REDACTED]',
        normalField: 'ok',
      });
    });

    it('should use custom redactHistoryEntry when provided', async () => {
      const customRedact = (entry: HistoryEntry) => ({
        ...entry,
        endpoint: 'REDACTED',
      });

      render(
        <ApiRequestPanel
          endpoint="https://api.example.com/test"
          method="GET"
          persistHistory={true}
          response={{ data: 'test' }}
          redactHistoryEntry={customRedact}
        />
      );

      const stored = localStorage.getItem('anchorkit_api_history');
      const history: HistoryEntry[] = JSON.parse(stored!);
      expect(history[0].endpoint).toBe('REDACTED');
    });
  });
});
