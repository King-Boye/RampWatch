import React, { useMemo, useState } from 'react';
import type { TransactionProgressStatus } from './TransactionStatusTracker';
import './TransactionHistory.css';

type HistoryViewMode = 'table' | 'card';

export interface TransactionHistoryItem {
  id: string;
  amount: number;
  asset: string;
  recipient: string;
  status: TransactionProgressStatus;
  timestamp: string;
  memo?: string;
  details?: Record<string, string | number>;
}

interface TransactionHistoryProps {
  transactions: TransactionHistoryItem[];
  defaultView?: HistoryViewMode;
  title?: string;
  pageSize?: number;
  currentPage?: number;
  onPageChange?: (page: number) => void;
  onLoadMore?: () => void;
  isLoading?: boolean;
}

function formatAmount(amount: number, asset: string): string {
  return `${amount.toLocaleString(undefined, { maximumFractionDigits: 6 })} ${asset}`;
}

function formatTimestamp(value: string): string {
  const parsed = new Date(value);
  if (Number.isNaN(parsed.getTime())) return value;
  return parsed.toLocaleString();
}

function downloadJSON(tx: TransactionHistoryItem): void {
  const receipt = {
    transactionId: tx.id,
    amount: tx.amount,
    asset: tx.asset,
    recipient: tx.recipient,
    status: tx.status,
    timestamp: tx.timestamp,
    ...(tx.memo ? { memo: tx.memo } : {}),
    ...(tx.details ? { details: tx.details } : {}),
    downloadedAt: new Date().toISOString(),
  };
  const blob = new Blob([JSON.stringify(receipt, null, 2)], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `receipt-${tx.id}.json`;
  a.click();
  URL.revokeObjectURL(url);
}

function downloadPDF(tx: TransactionHistoryItem): void {
  const rows = [
    ['Transaction ID', tx.id],
    ['Amount', formatAmount(tx.amount, tx.asset)],
    ['Asset', tx.asset],
    ['Recipient', tx.recipient],
    ['Status', tx.status],
    ['Timestamp', formatTimestamp(tx.timestamp)],
    ...(tx.memo ? [['Memo', tx.memo]] : []),
    ...Object.entries(tx.details || {}).map(([k, v]) => [k, String(v)]),
    ['Downloaded At', new Date().toLocaleString()],
  ];

  const tableRows = rows
    .map(([k, v]) => `<tr><th>${k}</th><td>${v}</td></tr>`)
    .join('');

  const html = `<!DOCTYPE html><html><head><meta charset="utf-8">
<title>Receipt ${tx.id}</title>
<style>
  body { font-family: sans-serif; padding: 32px; color: #111; }
  h1 { font-size: 1.4em; margin-bottom: 16px; }
  table { border-collapse: collapse; width: 100%; }
  th, td { text-align: left; padding: 8px 12px; border-bottom: 1px solid #ddd; }
  th { width: 40%; color: #555; font-weight: 600; }
</style>
</head><body>
<h1>Transaction Receipt</h1>
<table>${tableRows}</table>
</body></html>`;

  const win = window.open('', '_blank');
  if (!win) return;
  win.document.write(html);
  win.document.close();
  win.focus();
  win.print();
  win.close();
}

function ReceiptDownloadButtons({ tx }: { tx: TransactionHistoryItem }) {
  return (
    <span className="receipt-buttons">
      <button
        type="button"
        className="receipt-btn"
        onClick={() => downloadJSON(tx)}
        aria-label={`Download JSON receipt for transaction ${tx.id}`}
        title="Download JSON"
      >
        JSON
      </button>
      <button
        type="button"
        className="receipt-btn"
        onClick={() => downloadPDF(tx)}
        aria-label={`Download PDF receipt for transaction ${tx.id}`}
        title="Download PDF"
      >
        PDF
      </button>
    </span>
  );
}

export const TransactionHistory: React.FC<TransactionHistoryProps> = ({
  transactions,
  defaultView = 'table',
  title = 'Transaction History',
  pageSize = 20,
  currentPage: controlledPage,
  onPageChange,
  onLoadMore,
  isLoading = false,
}) => {
  const [view, setView] = useState<HistoryViewMode>(defaultView);
  const [expandedId, setExpandedId] = useState<string | null>(null);
  const [uncontrolledPage, setUncontrolledPage] = useState(1);

  const isControlled = controlledPage !== undefined;
  const currentPage = isControlled ? controlledPage : uncontrolledPage;

  const hasTransactions = useMemo(() => transactions.length > 0, [transactions]);

  const paginationData = useMemo(() => {
    const total = transactions.length;
    const totalPages = Math.ceil(total / pageSize);
    const startIdx = (currentPage - 1) * pageSize;
    const endIdx = startIdx + pageSize;
    const paginatedItems = transactions.slice(startIdx, endIdx);

    return {
      items: paginatedItems,
      totalPages,
      totalRecords: total,
      startRecord: total === 0 ? 0 : startIdx + 1,
      endRecord: Math.min(endIdx, total),
    };
  }, [transactions, pageSize, currentPage]);

  const handlePageChange = (newPage: number) => {
    if (isControlled && onPageChange) {
      onPageChange(newPage);
    } else {
      setUncontrolledPage(newPage);
    }
  };

  const handlePrevPage = () => {
    if (currentPage > 1) {
      handlePageChange(currentPage - 1);
    }
  };

  const handleNextPage = () => {
    if (currentPage < paginationData.totalPages) {
      handlePageChange(currentPage + 1);
    } else if (onLoadMore) {
      onLoadMore();
    }
  };

  const toggleExpanded = (id: string) => {
    setExpandedId((current) => (current === id ? null : id));
  };

  // Reset to page 1 when transactions change
  React.useEffect(() => {
    if (!isControlled) {
      setUncontrolledPage(1);
    }
  }, [transactions, isControlled]);

  return (
    <section className="transaction-history" aria-label="Transaction history">
      <header className="transaction-history-header">
        <h2>{title}</h2>
        <div className="history-view-controls" role="tablist" aria-label="History view mode">
          <button
            type="button"
            className={view === 'table' ? 'active' : ''}
            onClick={() => setView('table')}
            role="tab"
            aria-selected={view === 'table'}
          >
            Table
          </button>
          <button
            type="button"
            className={view === 'card' ? 'active' : ''}
            onClick={() => setView('card')}
            role="tab"
            aria-selected={view === 'card'}
          >
            Cards
          </button>
        </div>
      </header>

      {isLoading && (
        <div className="history-loading" aria-live="polite">
          <div className="history-loading-spinner" />
          <span>Loading more transactions...</span>
        </div>
      )}

      {!hasTransactions && <p className="history-empty">No transactions yet.</p>}

      {hasTransactions && (
        <>
          <div className="history-pagination-info" aria-live="polite" aria-atomic="true">
            Showing {paginationData.startRecord}–{paginationData.endRecord} of{' '}
            {paginationData.totalRecords} transactions
          </div>

          {view === 'table' && (
            <div className="history-table-wrap">
              <table className="history-table">
                <thead>
                  <tr>
                    <th>Amount</th>
                    <th>Asset</th>
                    <th>Recipient</th>
                    <th>Status</th>
                    <th>Timestamp</th>
                    <th aria-label="Expand details column" />
                    <th>Receipt</th>
                  </tr>
                </thead>
                <tbody>
                  {paginationData.items.map((transaction) => {
                    const isExpanded = expandedId === transaction.id;
                    return (
                      <React.Fragment key={transaction.id}>
                        <tr>
                          <td>{formatAmount(transaction.amount, transaction.asset)}</td>
                          <td>{transaction.asset}</td>
                          <td className="history-recipient">{transaction.recipient}</td>
                          <td>
                            <span className={`history-status status-${transaction.status}`}>
                              {transaction.status}
                            </span>
                          </td>
                          <td>{formatTimestamp(transaction.timestamp)}</td>
                          <td>
                            <button
                              type="button"
                              className="history-expand"
                              onClick={() => toggleExpanded(transaction.id)}
                              aria-expanded={isExpanded}
                            >
                              {isExpanded ? 'Hide' : 'Expand'}
                            </button>
                          </td>
                          <td><ReceiptDownloadButtons tx={transaction} /></td>
                        </tr>
                        {isExpanded && (
                          <tr className="history-details-row">
                            <td colSpan={7}>
                              <dl className="history-details">
                                {transaction.memo && (
                                  <div key={`${transaction.id}-memo`}>
                                    <dt>Memo</dt>
                                    <dd>{transaction.memo}</dd>
                                  </div>
                                )}
                                {Object.entries(transaction.details || {}).map(([key, value]) => (
                                  <div key={`${transaction.id}-${key}`}>
                                    <dt>{key}</dt>
                                    <dd>{value}</dd>
                                  </div>
                                ))}
                              </dl>
                            </td>
                          </tr>
                        )}
                      </React.Fragment>
                    );
                  })}
                </tbody>
              </table>
            </div>
          )}

          {view === 'card' && (
            <div className="history-cards">
              {paginationData.items.map((transaction) => {
                const isExpanded = expandedId === transaction.id;
                return (
                  <article key={transaction.id} className="history-card">
                    <div className="history-card-top">
                      <p>{formatAmount(transaction.amount, transaction.asset)}</p>
                      <span className={`history-status status-${transaction.status}`}>
                        {transaction.status}
                      </span>
                    </div>
                    <dl className="history-card-grid">
                      <div>
                        <dt>Asset</dt>
                        <dd>{transaction.asset}</dd>
                      </div>
                      <div>
                        <dt>Recipient</dt>
                        <dd className="history-recipient">{transaction.recipient}</dd>
                      </div>
                      <div>
                        <dt>Timestamp</dt>
                        <dd>{formatTimestamp(transaction.timestamp)}</dd>
                      </div>
                    </dl>
                    <button
                      type="button"
                      className="history-expand"
                      onClick={() => toggleExpanded(transaction.id)}
                      aria-expanded={isExpanded}
                    >
                      {isExpanded ? 'Hide details' : 'Expand details'}
                    </button>
                    <ReceiptDownloadButtons tx={transaction} />
                    {isExpanded && (
                      <dl className="history-details">
                        {transaction.memo && (
                          <div key={`${transaction.id}-memo`}>
                            <dt>Memo</dt>
                            <dd>{transaction.memo}</dd>
                          </div>
                        )}
                        {Object.entries(transaction.details || {}).map(([key, value]) => (
                          <div key={`${transaction.id}-${key}`}>
                            <dt>{key}</dt>
                            <dd>{value}</dd>
                          </div>
                        ))}
                      </dl>
                    )}
                  </article>
                );
              })}
            </div>
          )}

          <nav className="history-pagination" aria-label="Pagination">
            <button
              type="button"
              onClick={handlePrevPage}
              disabled={currentPage === 1 || isLoading}
              aria-label="Previous page"
            >
              Previous
            </button>
            <span className="pagination-info" aria-live="polite">
              Page {currentPage} of {paginationData.totalPages}
            </span>
            <button
              type="button"
              onClick={handleNextPage}
              disabled={currentPage === paginationData.totalPages || isLoading}
              aria-label="Next page"
            >
              {onLoadMore && currentPage === paginationData.totalPages ? 'Load More' : 'Next'}
            </button>
          </nav>
        </>
      )}
    </section>
  );
};
