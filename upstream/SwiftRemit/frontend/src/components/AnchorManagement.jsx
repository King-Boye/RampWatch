import { useState, useEffect } from 'react';
import { validateAnchorNumbers, parseList } from '../utils/anchorValidation';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

const EMPTY_FORM = {
  name: '', domain: '', description: '',
  deposit_fee_percent: '', withdrawal_fee_percent: '',
  min_amount: '', max_amount: '',
  kyc_required: false, kyc_level: 'basic',
  supported_countries: '', supported_currencies: '',
};

export default function AnchorManagement() {
  const [anchors, setAnchors] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [form, setForm] = useState(EMPTY_FORM);
  const [editingId, setEditingId] = useState(null);
  const [saving, setSaving] = useState(false);
  const [healthStatus, setHealthStatus] = useState({});
  const [fieldErrors, setFieldErrors] = useState({});

  const updateNumber = (key, value) => {
    setForm(f => ({ ...f, [key]: value }));
    setFieldErrors(errs => (errs[key] ? { ...errs, [key]: undefined } : errs));
  };

  useEffect(() => { fetchAnchors(); }, []);

  async function fetchAnchors() {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`${API_URL}/api/anchors`);
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      setAnchors(await res.json());
    } catch (e) {
      setError(e.message);
      setAnchors([]);
    } finally {
      setLoading(false);
    }
  }

  async function handleSubmit(e) {
    e.preventDefault();
    setError(null);

    // Validate instead of coercing bad input to 0 (#1720).
    const { valid, errors, values } = validateAnchorNumbers(form);
    const currencies = parseList(form.supported_currencies);
    if (currencies.length === 0) errors.supported_currencies = 'At least one supported currency is required.';
    setFieldErrors(errors);
    if (!valid || !values || errors.supported_currencies) {
      setError('Please fix the highlighted fields.');
      return;
    }

    setSaving(true);
    try {
      const payload = {
        name: form.name,
        domain: form.domain,
        description: form.description,
        fees: {
          deposit_fee_percent: values.deposit_fee_percent,
          withdrawal_fee_percent: values.withdrawal_fee_percent,
        },
        limits: {
          min_amount: values.min_amount,
          max_amount: values.max_amount,
        },
        compliance: {
          kyc_required: form.kyc_required,
          kyc_level: form.kyc_level,
          supported_countries: parseList(form.supported_countries),
        },
        supported_currencies: currencies,
      };
      const method = editingId ? 'PUT' : 'POST';
      const url = editingId ? `${API_URL}/api/anchors/${editingId}` : `${API_URL}/api/anchors`;
      const res = await fetch(url, {
        method,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      setForm(EMPTY_FORM);
      setFieldErrors({});
      setEditingId(null);
      await fetchAnchors();
    } catch (e) {
      setError(e.message);
    } finally {
      setSaving(false);
    }
  }

  async function toggleStatus(anchor) {
    try {
      const newStatus = anchor.status === 'active' ? 'inactive' : 'active';
      const res = await fetch(`${API_URL}/api/anchors/${anchor.id}`, {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ status: newStatus }),
      });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      await fetchAnchors();
    } catch (e) {
      setError(e.message);
    }
  }

  async function handleDelete(id) {
    if (!confirm('Delete this anchor provider?')) return;
    try {
      const res = await fetch(`${API_URL}/api/anchors/${id}`, { method: 'DELETE' });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      await fetchAnchors();
    } catch (e) {
      setError(e.message);
    }
  }

  async function checkHealth(anchor) {
    setHealthStatus(prev => ({ ...prev, [anchor.id]: 'checking…' }));
    try {
      const res = await fetch(`${API_URL}/api/anchors/${anchor.id}/health`);
      const data = res.ok ? await res.json() : null;
      setHealthStatus(prev => ({ ...prev, [anchor.id]: data?.healthy ? '✅ Healthy' : '❌ Unhealthy' }));
    } catch {
      setHealthStatus(prev => ({ ...prev, [anchor.id]: '❌ Unreachable' }));
    }
  }

  function startEdit(anchor) {
    setEditingId(anchor.id);
    setFieldErrors({});
    setForm({
      name: anchor.name,
      domain: anchor.domain,
      description: anchor.description || '',
      deposit_fee_percent: anchor.fees?.deposit_fee_percent ?? '',
      withdrawal_fee_percent: anchor.fees?.withdrawal_fee_percent ?? '',
      min_amount: anchor.limits?.min_amount ?? '',
      max_amount: anchor.limits?.max_amount ?? '',
      kyc_required: anchor.compliance?.kyc_required ?? false,
      kyc_level: anchor.compliance?.kyc_level ?? 'basic',
      supported_countries: anchor.compliance?.supported_countries?.join(', ') ?? '',
      supported_currencies: anchor.supported_currencies?.join(', ') ?? '',
    });
  }

  const statusColor = { active: '#38a169', inactive: '#718096', maintenance: '#d69e2e' };

  return (
    <div className="panel" role="main" aria-label="Anchor Management">
      <h2>Anchor Catalog</h2>

      <form onSubmit={handleSubmit} aria-label={editingId ? 'Edit anchor' : 'Add anchor'}>
        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '12px' }}>
          <div className="form-group">
            <label htmlFor="anc-name">Name *</label>
            <input id="anc-name" value={form.name} onChange={e => setForm(f => ({ ...f, name: e.target.value }))} required />
          </div>
          <div className="form-group">
            <label htmlFor="anc-domain">Domain *</label>
            <input id="anc-domain" value={form.domain} onChange={e => setForm(f => ({ ...f, domain: e.target.value }))} placeholder="anchor.example.com" required />
          </div>
          <div className="form-group" style={{ gridColumn: '1 / -1' }}>
            <label htmlFor="anc-desc">Description</label>
            <input id="anc-desc" value={form.description} onChange={e => setForm(f => ({ ...f, description: e.target.value }))} />
          </div>
          <div className="form-group">
            <label htmlFor="anc-dep-fee">Deposit Fee % *</label>
            <input id="anc-dep-fee" type="text" inputMode="decimal" value={form.deposit_fee_percent} onChange={e => updateNumber('deposit_fee_percent', e.target.value)} aria-invalid={!!fieldErrors.deposit_fee_percent} aria-describedby={fieldErrors.deposit_fee_percent ? 'anc-dep-fee-error' : undefined} />
            {fieldErrors.deposit_fee_percent && <span id="anc-dep-fee-error" className="field-error" role="alert">{fieldErrors.deposit_fee_percent}</span>}
          </div>
          <div className="form-group">
            <label htmlFor="anc-wit-fee">Withdrawal Fee % *</label>
            <input id="anc-wit-fee" type="text" inputMode="decimal" value={form.withdrawal_fee_percent} onChange={e => updateNumber('withdrawal_fee_percent', e.target.value)} aria-invalid={!!fieldErrors.withdrawal_fee_percent} aria-describedby={fieldErrors.withdrawal_fee_percent ? 'anc-wit-fee-error' : undefined} />
            {fieldErrors.withdrawal_fee_percent && <span id="anc-wit-fee-error" className="field-error" role="alert">{fieldErrors.withdrawal_fee_percent}</span>}
          </div>
          <div className="form-group">
            <label htmlFor="anc-min">Min Amount *</label>
            <input id="anc-min" type="text" inputMode="decimal" value={form.min_amount} onChange={e => updateNumber('min_amount', e.target.value)} aria-invalid={!!fieldErrors.min_amount} aria-describedby={fieldErrors.min_amount ? 'anc-min-error' : undefined} />
            {fieldErrors.min_amount && <span id="anc-min-error" className="field-error" role="alert">{fieldErrors.min_amount}</span>}
          </div>
          <div className="form-group">
            <label htmlFor="anc-max">Max Amount *</label>
            <input id="anc-max" type="text" inputMode="decimal" value={form.max_amount} onChange={e => updateNumber('max_amount', e.target.value)} aria-invalid={!!fieldErrors.max_amount} aria-describedby={fieldErrors.max_amount ? 'anc-max-error' : undefined} />
            {fieldErrors.max_amount && <span id="anc-max-error" className="field-error" role="alert">{fieldErrors.max_amount}</span>}
          </div>
          <div className="form-group">
            <label htmlFor="anc-currencies">Currencies (comma-separated) *</label>
            <input id="anc-currencies" value={form.supported_currencies} onChange={e => setForm(f => ({ ...f, supported_currencies: e.target.value }))} placeholder="USD, EUR, NGN" aria-invalid={!!fieldErrors.supported_currencies} aria-describedby={fieldErrors.supported_currencies ? 'anc-currencies-error' : undefined} />
            {fieldErrors.supported_currencies && <span id="anc-currencies-error" className="field-error" role="alert">{fieldErrors.supported_currencies}</span>}
          </div>
          <div className="form-group">
            <label htmlFor="anc-countries">Countries (comma-separated)</label>
            <input id="anc-countries" value={form.supported_countries} onChange={e => setForm(f => ({ ...f, supported_countries: e.target.value }))} placeholder="US, NG, GH" />
          </div>
          <div className="form-group">
            <label htmlFor="anc-kyc-level">KYC Level</label>
            <select id="anc-kyc-level" value={form.kyc_level} onChange={e => setForm(f => ({ ...f, kyc_level: e.target.value }))}>
              <option value="basic">Basic</option>
              <option value="intermediate">Intermediate</option>
              <option value="advanced">Advanced</option>
            </select>
          </div>
          <div className="form-group" style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <input id="anc-kyc-req" type="checkbox" checked={form.kyc_required} onChange={e => setForm(f => ({ ...f, kyc_required: e.target.checked }))} />
            <label htmlFor="anc-kyc-req">KYC Required</label>
          </div>
        </div>
        <div style={{ display: 'flex', gap: '8px', marginTop: '8px' }}>
          <button type="submit" className="btn-primary" disabled={saving}>
            {saving ? 'Saving…' : editingId ? 'Update Anchor' : 'Add Anchor'}
          </button>
          {editingId && (
            <button type="button" onClick={() => { setEditingId(null); setForm(EMPTY_FORM); setFieldErrors({}); }}>Cancel</button>
          )}
        </div>
      </form>

      {error && <div className="error" role="alert">{error}</div>}

      <hr style={{ margin: '24px 0' }} />

      {loading ? (
        <p aria-live="polite">Loading…</p>
      ) : anchors.length === 0 ? (
        <p>No anchor providers yet.</p>
      ) : (
        <ul style={{ listStyle: 'none', padding: 0 }} aria-label="Anchor list">
          {anchors.map(anchor => (
            <li key={anchor.id} style={{ border: '1px solid #e2e8f0', borderRadius: '8px', padding: '16px', marginBottom: '12px' }}>
              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', flexWrap: 'wrap', gap: '8px' }}>
                <div>
                  <strong>{anchor.name}</strong>
                  <span style={{ marginLeft: '8px', fontSize: '0.75rem', color: statusColor[anchor.status] || '#718096', fontWeight: 600 }}>
                    ● {anchor.status}
                  </span>
                  <div style={{ fontSize: '0.8rem', color: '#666' }}>{anchor.domain}</div>
                  {anchor.description && <div style={{ fontSize: '0.8rem', color: '#888', marginTop: '2px' }}>{anchor.description}</div>}
                </div>
                <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap' }}>
                  <button
                    onClick={() => toggleStatus(anchor)}
                    aria-label={`${anchor.status === 'active' ? 'Disable' : 'Enable'} ${anchor.name}`}
                  >
                    {anchor.status === 'active' ? 'Disable' : 'Enable'}
                  </button>
                  <button onClick={() => checkHealth(anchor)} aria-label={`Check health of ${anchor.name}`}>
                    Health
                  </button>
                  <button onClick={() => startEdit(anchor)} aria-label={`Edit ${anchor.name}`}>Edit</button>
                  <button onClick={() => handleDelete(anchor.id)} aria-label={`Delete ${anchor.name}`} style={{ color: '#e53e3e' }}>Delete</button>
                </div>
              </div>
              {healthStatus[anchor.id] && (
                <div style={{ marginTop: '6px', fontSize: '0.85rem' }} aria-live="polite">{healthStatus[anchor.id]}</div>
              )}
              <div style={{ marginTop: '8px', fontSize: '0.8rem', color: '#555', display: 'flex', gap: '16px', flexWrap: 'wrap' }}>
                <span>Deposit: {anchor.fees?.deposit_fee_percent ?? '—'}%</span>
                <span>Withdrawal: {anchor.fees?.withdrawal_fee_percent ?? '—'}%</span>
                <span>Limits: {anchor.limits?.min_amount ?? '—'} – {anchor.limits?.max_amount ?? '—'}</span>
                <span>KYC: {anchor.compliance?.kyc_level ?? '—'}</span>
                <span>Currencies: {anchor.supported_currencies?.join(', ') || '—'}</span>
              </div>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
