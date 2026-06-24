import React, { useState, useEffect } from 'react';
import './ApiRequestPanel.css';

export interface ApiRequestPanelProps {
  endpoint: string;
  method?: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH';
  requestBody?: Record<string, any> | string;
  response?: Record<string, any> | string;
  headers?: Record<string, string>;
  isLoading?: boolean;
  error?: string;
  editable?: boolean;
  onSubmit?: (body: Record<string, any> | string) => void;
}

export const ApiRequestPanel: React.FC<ApiRequestPanelProps> = ({
  endpoint,
  method = 'POST',
  requestBody,
  response,
  headers = {},
  isLoading = false,
  error,
  editable = false,
  onSubmit,
}) => {
  const formatJson = (data: any): string => {
    if (typeof data === 'string') return data;
    return JSON.stringify(data, null, 2);
  };

  const [copiedSection, setCopiedSection] = useState<string | null>(null);
  const [editableBody, setEditableBody] = useState<string>(
    requestBody ? formatJson(requestBody) : ''
  );
  const [jsonError, setJsonError] = useState<string | null>(null);

  useEffect(() => {
    setEditableBody(requestBody ? formatJson(requestBody) : '');
    setJsonError(null);
  }, [requestBody]);

  const validateJson = (value: string): boolean => {
    if (!value.trim()) {
      setJsonError(null);
      return true;
    }

    try {
      JSON.parse(value);
      setJsonError(null);
      return true;
    } catch (err) {
      const error = err as Error;
      setJsonError(error.message);
      return false;
    }
  };

  const handleBodyChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    const value = e.target.value;
    setEditableBody(value);
    validateJson(value);
  };

  const handleSubmit = () => {
    if (!onSubmit || jsonError) return;

    try {
      const parsedBody = editableBody.trim() ? JSON.parse(editableBody) : {};
      onSubmit(parsedBody);
    } catch (err) {
      // Should not happen as we validate before enabling submit
      console.error('Failed to parse JSON:', err);
    }
  };

  const isSubmitDisabled = !!jsonError || isLoading;

  const generateCurl = (): string => {
    const headerFlags = Object.entries(headers)
      .map(([key, value]) => `-H "${key}: ${value.replace(/"/g, '\\"')}"`)

      .join(' \\\n  ');

    let curl = `curl -X ${method} \\\n  "${endpoint}"`;
    
    if (headerFlags) {
      curl += ` \\\n  ${headerFlags}`;
    }

    if (requestBody && (method === 'POST' || method === 'PUT' || method === 'PATCH')) {
      const body = formatJson(requestBody);
      curl += ` \\\n  -d '${body}'`;
    }

    return curl;
  };

  const copyToClipboard = async (text: string, section: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopiedSection(section);
      setTimeout(() => setCopiedSection(null), 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  };

  const CopyButton: React.FC<{ text: string; section: string }> = ({ text, section }) => (
    <button
      className="copy-button"
      onClick={() => copyToClipboard(text, section)}
      title={`Copy ${section}`}
    >
      {copiedSection === section ? (
        <span className="copy-icon">✓</span>
      ) : (
        <span className="copy-icon">📋</span>
      )}
    </button>
  );

  return (
    <div className="api-request-panel">
      {/* Endpoint Section */}
      <div className="panel-section endpoint-section">
        <div className="section-header">
          <span className="method-badge" data-method={method}>
            {method}
          </span>
          <h3>Endpoint</h3>
        </div>
        <div className="section-content">
          <code className="endpoint-url">{endpoint}</code>
          <CopyButton text={endpoint} section="endpoint" />
        </div>
      </div>

      {/* Request Body Section */}
      {(requestBody || editable) && (
        <div className="panel-section request-section">
          <div className="section-header">
            <h3>Request Body</h3>
            {!editable && <CopyButton text={formatJson(requestBody)} section="request" />}
          </div>
          <div className="section-content">
            {editable ? (
              <div className="editable-body-container">
                <textarea
                  className={`editable-body ${jsonError ? 'error' : ''}`}
                  value={editableBody}
                  onChange={handleBodyChange}
                  placeholder='{"key": "value"}'
                  rows={10}
                  aria-label="Request body JSON"
                  aria-invalid={!!jsonError}
                  aria-describedby={jsonError ? 'json-error' : undefined}
                />
                {jsonError && (
                  <div className="json-error" id="json-error" role="alert">
                    <span className="error-icon">⚠️</span>
                    <span>Invalid JSON: {jsonError}</span>
                  </div>
                )}
                {onSubmit && (
                  <button
                    className="submit-button"
                    onClick={handleSubmit}
                    disabled={isSubmitDisabled}
                    title={jsonError ? 'Fix JSON errors before submitting' : 'Submit request'}
                  >
                    {isLoading ? 'Submitting...' : 'Submit Request'}
                  </button>
                )}
              </div>
            ) : (
              <pre className="code-block">
                <code>{formatJson(requestBody)}</code>
              </pre>
            )}
          </div>
        </div>
      )}

      {/* Response Section */}
      <div className="panel-section response-section">
        <div className="section-header">
          <h3>Response</h3>
          {response && <CopyButton text={formatJson(response)} section="response" />}
        </div>
        <div className="section-content">
          {isLoading ? (
            <div className="skeleton-loader">
              <div className="skeleton-line"></div>
              <div className="skeleton-line"></div>
              <div className="skeleton-line short"></div>
            </div>
          ) : error ? (
            <div className="error-message">
              <span className="error-icon">⚠️</span>
              <span>{error}</span>
            </div>
          ) : response ? (
            <pre className="code-block">
              <code>{formatJson(response)}</code>
            </pre>
          ) : (
            <div className="empty-state">No response yet</div>
          )}
        </div>
      </div>

      {/* cURL Command Section */}
      <div className="panel-section curl-section">
        <div className="section-header">
          <h3>cURL Command</h3>
          <CopyButton text={generateCurl()} section="curl" />
        </div>
        <div className="section-content">
          <pre className="code-block curl-block">
            <code>{generateCurl()}</code>
          </pre>
        </div>
      </div>
    </div>
  );
};
