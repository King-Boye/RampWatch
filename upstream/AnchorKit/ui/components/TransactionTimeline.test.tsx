import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { TransactionTimeline, TxStatus, TxType, TxEvent } from './TransactionTimeline';
import '@testing-library/jest-dom';

const user = userEvent.setup();

const baseProps = {
  type: 'deposit' as TxType,
  amount: '250.00',
  asset: 'USDC',
  events: [] as TxEvent[],
  currentStatus: 'initiated' as TxStatus,
  onRetry: jest.fn(),
  onClose: jest.fn(),
};

describe('TransactionTimeline', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });
  test('renders deposit header correctly', () => {
    render(<TransactionTimeline {...baseProps} />);
    expect(screen.getByText('↓ Deposit')).toBeInTheDocument();
    expect(screen.getByText('250.00')).toHaveTextContent('250.00');
    expect(screen.getByText('USDC')).toBeInTheDocument();
    expect(screen.getAllByText('Initiated').length).toBeGreaterThan(0);
  });

  test('renders withdrawal header correctly', () => {
    render(<TransactionTimeline {...baseProps} type="withdrawal" />);
    expect(screen.getByText('↑ Withdrawal')).toBeInTheDocument();
  });

  test('renders status badge with correct color class', () => {
    render(<TransactionTimeline {...baseProps} currentStatus="processing" />);
    const badges = screen.getAllByText('Processing');
    expect(badges[0]).toBeInTheDocument();
    expect(badges[0]).toHaveStyle({ color: '#0284c7' });
  });

  test('renders all TxStatus icons and labels', () => {
    const statuses: TxStatus[] = ['initiated', 'pending', 'processing', 'completed', 'failed'];
    statuses.forEach(status => {
      render(<TransactionTimeline {...baseProps} currentStatus={status} />);
      const label = screen.getAllByText(new RegExp(status.charAt(0).toUpperCase() + status.slice(1), 'i'))[0];
      expect(label).toBeInTheDocument();
    });
  });

  test('shows txHash link when completed with txHash', () => {
    const events: TxEvent[] = [{
      status: 'completed' as TxStatus,
      txHash: 'abc123def456',
    }];
    render(<TransactionTimeline {...baseProps} currentStatus="completed" events={events} />);
    // truncateHash: 8 chars + … + 8 chars; 'abc123def456' is 12 chars so shown as-is (≤16)
    expect(screen.getByText(/abc123def456/)).toBeInTheDocument();
    const link = screen.getByRole('link');
    expect(link).toHaveAttribute('href', expect.stringContaining('abc123def456'));
  });

  test('shows Retry button on failed status and calls onRetry on click', async () => {
    render(<TransactionTimeline {...baseProps} currentStatus="failed" />);
    const retryButton = screen.getByRole('button', { name: /retry/i });
    expect(retryButton).toBeInTheDocument();
    await user.click(retryButton);
    expect(baseProps.onRetry).toHaveBeenCalledTimes(1);
  });

  test('shows Close button and calls onClose on click', async () => {
    render(<TransactionTimeline {...baseProps} currentStatus="pending" />);
    const closeButton = screen.getByRole('button', { name: /close/i });
    expect(closeButton).toBeInTheDocument();
    await user.click(closeButton);
    expect(baseProps.onClose).toHaveBeenCalledTimes(1);
  });

  test('shows Done button on completed and calls onClose', async () => {
    render(<TransactionTimeline {...baseProps} currentStatus="completed" />);
    const doneButton = screen.getByRole('button', { name: /done/i });
    expect(doneButton).toBeInTheDocument();
    await user.click(doneButton);
    expect(baseProps.onClose).toHaveBeenCalledTimes(1);
  });

  test('renders event timestamps and details', () => {
    const events: TxEvent[] = [{
      status: 'pending' as TxStatus,
      timestamp: '2024-01-15T10:30:00Z',
      detail: 'via ACH',
    }];
    render(<TransactionTimeline {...baseProps} events={events} currentStatus="pending" />);
    // formatTs uses toLocaleString — find any element containing "Jan" and "15"
    const tsEls = screen.getAllByText((_, el) =>
      !!el?.textContent?.includes('Jan') && !!el?.textContent?.includes('15')
    );
    expect(tsEls.length).toBeGreaterThan(0);
    expect(screen.getByText('via ACH')).toBeInTheDocument();
  });

  test('renders failed state with custom label and description', () => {
    const events: TxEvent[] = [{
      status: 'failed' as TxStatus,
      label: 'Bank Error',
      description: 'Account details mismatch',
    }];
    render(<TransactionTimeline {...baseProps} events={events} currentStatus="failed" />);
    expect(screen.getByText('Bank Error')).toBeInTheDocument();
    expect(screen.getByText('Account details mismatch')).toBeInTheDocument();
    expect(screen.getByText('✕')).toBeInTheDocument();
  });

  test('does not show Retry button when not failed', () => {
    render(<TransactionTimeline {...baseProps} currentStatus="completed" />);
    expect(screen.queryByRole('button', { name: /retry/i })).not.toBeInTheDocument();
  });

  test('renders countdown when estimatedCompletionAt is provided', () => {
    const future = Date.now() + 120000; // 2 minutes from now
    render(<TransactionTimeline {...baseProps} estimatedCompletionAt={future} />);
    expect(screen.getByText(/Est. Completion/i)).toBeInTheDocument();
    expect(screen.getByText(/2:00/)).toBeInTheDocument();
  });

  describe('Error State Rendering', () => {
    test('renders failed transaction with error message', () => {
      const events: TxEvent[] = [
        { status: 'initiated', timestamp: '2024-01-15T10:00:00Z' },
        { status: 'pending', timestamp: '2024-01-15T10:05:00Z' },
        {
          status: 'failed',
          timestamp: '2024-01-15T10:10:00Z',
          label: 'Payment Failed',
          description: 'Insufficient funds in source account',
        },
      ];
      render(<TransactionTimeline {...baseProps} currentStatus="failed" events={events} />);
      
      expect(screen.getByText('Payment Failed')).toBeInTheDocument();
      expect(screen.getByText('Insufficient funds in source account')).toBeInTheDocument();
      expect(screen.getByText('✕')).toBeInTheDocument();
    });

    test('renders failed state with default description when not provided', () => {
      render(<TransactionTimeline {...baseProps} type="deposit" currentStatus="failed" events={[]} />);
      
      expect(screen.getByText(/Deposit could not be completed/i)).toBeInTheDocument();
    });

    test('renders failed state for withdrawal with default description', () => {
      render(<TransactionTimeline {...baseProps} type="withdrawal" currentStatus="failed" events={[]} />);
      
      expect(screen.getByText(/Withdrawal could not be completed/i)).toBeInTheDocument();
    });

    test('renders very long error message without breaking layout', () => {
      const longError = 'A'.repeat(500);
      const events: TxEvent[] = [{
        status: 'failed',
        description: longError,
      }];
      render(<TransactionTimeline {...baseProps} currentStatus="failed" events={events} />);
      
      expect(screen.getByText(longError)).toBeInTheDocument();
    });

    test('shows retry button only on failed status', () => {
      const { rerender } = render(<TransactionTimeline {...baseProps} currentStatus="failed" />);
      expect(screen.getByRole('button', { name: /retry/i })).toBeInTheDocument();
      
      rerender(<TransactionTimeline {...baseProps} currentStatus="completed" />);
      expect(screen.queryByRole('button', { name: /retry/i })).not.toBeInTheDocument();
    });
  });

  describe('Unknown and Edge States', () => {
    test('renders with empty history array', () => {
      render(<TransactionTimeline {...baseProps} events={[]} currentStatus="initiated" />);
      
      expect(screen.getByText('Initiated')).toBeInTheDocument();
    });

    test('handles missing optional fields gracefully', () => {
      const minimalProps = {
        type: 'deposit' as TxType,
        amount: '100',
        asset: 'USDC',
        events: [] as TxEvent[],
        currentStatus: 'pending' as TxStatus,
      };
      render(<TransactionTimeline {...minimalProps} />);
      
      expect(screen.getByText('100')).toBeInTheDocument();
      expect(screen.getByText('USDC')).toBeInTheDocument();
    });

    test('renders transaction with no ID', () => {
      const propsWithoutId = { ...baseProps };
      delete (propsWithoutId as any).id;
      render(<TransactionTimeline {...propsWithoutId} />);
      
      expect(screen.getByText('250.00')).toBeInTheDocument();
    });

    test('renders with multiple events for same status', () => {
      const events: TxEvent[] = [
        { status: 'initiated', timestamp: '2024-01-15T10:00:00Z', detail: 'First attempt' },
        { status: 'initiated', timestamp: '2024-01-15T10:01:00Z', detail: 'Retry' },
      ];
      render(<TransactionTimeline {...baseProps} events={events} currentStatus="initiated" />);
      
      expect(screen.getByText('First attempt')).toBeInTheDocument();
    });
  });

  describe('Timestamp and Detail Rendering', () => {
    test('formats ISO timestamps correctly', () => {
      const events: TxEvent[] = [{
        status: 'completed',
        timestamp: '2024-01-15T14:30:00Z',
      }];
      render(<TransactionTimeline {...baseProps} currentStatus="completed" events={events} />);
      
      const tsElements = screen.getAllByText((_, el) =>
        !!el?.textContent?.includes('Jan') && !!el?.textContent?.includes('15')
      );
      expect(tsElements.length).toBeGreaterThan(0);
    });

    test('handles invalid timestamp gracefully', () => {
      const events: TxEvent[] = [{
        status: 'pending',
        timestamp: 'invalid-date',
      }];
      render(<TransactionTimeline {...baseProps} events={events} currentStatus="pending" />);
      
      expect(screen.getByText('invalid-date')).toBeInTheDocument();
    });

    test('renders event detail field', () => {
      const events: TxEvent[] = [{
        status: 'processing',
        detail: 'Processing via SEPA',
      }];
      render(<TransactionTimeline {...baseProps} events={events} currentStatus="processing" />);
      
      expect(screen.getByText('Processing via SEPA')).toBeInTheDocument();
    });
  });

  describe('Transaction Hash Links', () => {
    test('truncates long transaction hashes', () => {
      const longHash = 'a'.repeat(100);
      const events: TxEvent[] = [{
        status: 'completed',
        txHash: longHash,
      }];
      render(<TransactionTimeline {...baseProps} currentStatus="completed" events={events} />);
      
      const link = screen.getByRole('link');
      expect(link).toHaveAttribute('href', expect.stringContaining(longHash));
      // Should show truncated version (8 chars + … + 8 chars)
      expect(link.textContent).toContain('…');
    });

    test('does not truncate short hashes', () => {
      const shortHash = 'abc123';
      const events: TxEvent[] = [{
        status: 'completed',
        txHash: shortHash,
      }];
      render(<TransactionTimeline {...baseProps} currentStatus="completed" events={events} />);
      
      expect(screen.getByText(shortHash)).toBeInTheDocument();
    });
  });

  describe('Countdown Timer', () => {
    test('does not show countdown when transaction is completed', () => {
      const future = Date.now() + 60000;
      render(<TransactionTimeline {...baseProps} currentStatus="completed" estimatedCompletionAt={future} />);
      
      expect(screen.queryByText(/Est. Completion/i)).not.toBeInTheDocument();
    });

    test('does not show countdown when transaction is failed', () => {
      const future = Date.now() + 60000;
      render(<TransactionTimeline {...baseProps} currentStatus="failed" estimatedCompletionAt={future} />);
      
      expect(screen.queryByText(/Est. Completion/i)).not.toBeInTheDocument();
    });

    test('updates countdown timer', async () => {
      jest.useFakeTimers();
      const future = Date.now() + 65000; // 1:05
      render(<TransactionTimeline {...baseProps} currentStatus="processing" estimatedCompletionAt={future} />);
      
      expect(screen.getByText(/1:0[45]/)).toBeInTheDocument();
      
      jest.advanceTimersByTime(10000); // Advance 10 seconds
      
      await waitFor(() => {
        expect(screen.getByText(/0:5[0-9]/)).toBeInTheDocument();
      });
      
      jest.useRealTimers();
    });
  });
});
