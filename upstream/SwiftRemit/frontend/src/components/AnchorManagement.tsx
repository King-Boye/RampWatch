import { FC, useState, useEffect, FormEvent } from 'react'
import { ConfirmDialog } from './ConfirmDialog'
import {
  validateAnchorNumbers,
  parseList,
  type AnchorFieldErrors,
  type AnchorNumericField,
} from '../utils/anchorValidation'

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000'

interface AnchorForm {
  name: string
  domain: string
  description: string
  deposit_fee_percent: string
  withdrawal_fee_percent: string
  min_amount: string
  max_amount: string
  kyc_required: boolean
  kyc_level: 'basic' | 'standard' | 'full'
  supported_countries: string
  supported_currencies: string
}

interface Anchor extends AnchorForm {
  id: string
}

/** Shape returned by the API (nested fees/limits/kyc). */
interface ApiAnchor {
  id: string
  name?: string
  domain?: string
  description?: string
  fees?: { deposit_fee_percent?: number; withdrawal_fee_percent?: number }
  limits?: { min_amount?: number; max_amount?: number }
  kyc?: { required?: boolean; level?: AnchorForm['kyc_level'] }
  supported_countries?: string[]
  supported_currencies?: string[]
}

const str = (n: number | undefined): string => (n === undefined || n === null ? '' : String(n))

/** Maps an API anchor (nested or already flat) into editable form strings. */
function toForm(anchor: Anchor | ApiAnchor): AnchorForm {
  if ('deposit_fee_percent' in anchor) {
    const { id: _id, ...flat } = anchor as Anchor
    return { ...EMPTY_FORM, ...flat }
  }
  const a = anchor as ApiAnchor
  return {
    name: a.name ?? '',
    domain: a.domain ?? '',
    description: a.description ?? '',
    deposit_fee_percent: str(a.fees?.deposit_fee_percent),
    withdrawal_fee_percent: str(a.fees?.withdrawal_fee_percent),
    min_amount: str(a.limits?.min_amount),
    max_amount: str(a.limits?.max_amount),
    kyc_required: a.kyc?.required ?? false,
    kyc_level: a.kyc?.level ?? 'basic',
    supported_countries: (a.supported_countries ?? []).join(', '),
    supported_currencies: (a.supported_currencies ?? []).join(', '),
  }
}

const NUMERIC_FIELDS: { field: AnchorNumericField; label: string; placeholder: string }[] = [
  { field: 'deposit_fee_percent', label: 'Deposit fee (%)', placeholder: '1.5' },
  { field: 'withdrawal_fee_percent', label: 'Withdrawal fee (%)', placeholder: '1.5' },
  { field: 'min_amount', label: 'Minimum amount', placeholder: '10' },
  { field: 'max_amount', label: 'Maximum amount', placeholder: '10000' },
]

const EMPTY_FORM: AnchorForm = {
  name: '', domain: '', description: '',
  deposit_fee_percent: '', withdrawal_fee_percent: '',
  min_amount: '', max_amount: '',
  kyc_required: false, kyc_level: 'basic',
  supported_countries: '', supported_currencies: '',
}

const AnchorManagement: FC = () => {
  const [anchors, setAnchors] = useState<Anchor[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [form, setForm] = useState<AnchorForm>(EMPTY_FORM)
  const [editingId, setEditingId] = useState<string | null>(null)
  const [saving, setSaving] = useState(false)
  const [deleteTarget, setDeleteTarget] = useState<Anchor | null>(null)
  const [fieldErrors, setFieldErrors] = useState<AnchorFieldErrors>({})

  useEffect(() => { fetchAnchors() }, [])

  async function fetchAnchors(): Promise<void> {
    setLoading(true)
    setError(null)
    try {
      const res = await fetch(`${API_URL}/api/anchors`)
      if (!res.ok) throw new Error(`HTTP ${res.status}`)
      setAnchors(await res.json())
    } catch (e) {
      setError((e instanceof Error ? e.message : String(e)) || 'Failed to fetch anchors')
      setAnchors([])
    } finally {
      setLoading(false)
    }
  }

  async function handleSubmit(e: FormEvent<HTMLFormElement>): Promise<void> {
    e.preventDefault()
    setError(null)

    // Validate instead of coercing bad input to 0.
    const { valid, errors, values } = validateAnchorNumbers(form)
    const currencies = parseList(form.supported_currencies)
    if (currencies.length === 0) errors.supported_currencies = 'At least one supported currency is required.'
    setFieldErrors(errors)
    if (!valid || !values || errors.supported_currencies) {
      setError('Please fix the highlighted fields.')
      return
    }

    setSaving(true)
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
        kyc: {
          required: form.kyc_required,
          level: form.kyc_level,
        },
        supported_countries: parseList(form.supported_countries),
        supported_currencies: currencies,
      }
      const method = editingId ? 'PUT' : 'POST'
      const url = editingId ? `${API_URL}/api/anchors/${editingId}` : `${API_URL}/api/anchors`
      const res = await fetch(url, {
        method,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      })
      if (!res.ok) throw new Error(`HTTP ${res.status}`)
      setForm(EMPTY_FORM)
      setFieldErrors({})
      setEditingId(null)
      await fetchAnchors()
    } catch (e) {
      setError((e instanceof Error ? e.message : String(e)) || 'Failed to save anchor')
    } finally {
      setSaving(false)
    }
  }

  const handleEdit = (anchor: Anchor | ApiAnchor): void => {
    setForm(toForm(anchor))
    setFieldErrors({})
    setEditingId(anchor.id)
  }

  const handleDelete = async (id: string): Promise<void> => {
    setError(null)
    try {
      const res = await fetch(`${API_URL}/api/anchors/${id}`, { method: 'DELETE' })
      if (!res.ok) throw new Error(`HTTP ${res.status}`)
      await fetchAnchors()
    } catch (e) {
      setError((e instanceof Error ? e.message : String(e)) || 'Failed to delete anchor')
    }
  }

  return (
    <div className="panel" role="main" aria-label="Anchor Management">
      <h2>Anchor Management</h2>

      <ConfirmDialog
        open={deleteTarget !== null}
        title="Delete Anchor"
        message={`Are you sure you want to delete "${deleteTarget?.name}"? This action cannot be undone.`}
        confirmLabel="Delete"
        danger
        onConfirm={() => {
          if (deleteTarget) handleDelete(deleteTarget.id)
          setDeleteTarget(null)
        }}
        onCancel={() => setDeleteTarget(null)}
      />

      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor="anchor-name">Anchor Name</label>
          <input
            id="anchor-name"
            type="text"
            value={form.name}
            onChange={e => setForm({ ...form, name: e.target.value })}
            required
          />
        </div>
        <div className="form-group">
          <label htmlFor="anchor-domain">Domain</label>
          <input
            id="anchor-domain"
            type="text"
            value={form.domain}
            onChange={e => setForm({ ...form, domain: e.target.value })}
            placeholder="example.com"
            required
          />
        </div>
        <div className="form-group">
          <label htmlFor="anchor-desc">Description</label>
          <textarea
            id="anchor-desc"
            value={form.description}
            onChange={e => setForm({ ...form, description: e.target.value })}
          />
        </div>
        {NUMERIC_FIELDS.map(({ field, label, placeholder }) => (
          <div className="form-group" key={field}>
            <label htmlFor={`anchor-${field}`}>{label}</label>
            <input
              id={`anchor-${field}`}
              type="text"
              inputMode="decimal"
              value={form[field]}
              placeholder={placeholder}
              onChange={e => {
                setForm({ ...form, [field]: e.target.value })
                if (fieldErrors[field]) setFieldErrors({ ...fieldErrors, [field]: undefined })
              }}
              aria-invalid={!!fieldErrors[field]}
              aria-describedby={fieldErrors[field] ? `anchor-${field}-error` : undefined}
            />
            {fieldErrors[field] && (
              <span id={`anchor-${field}-error`} className="field-error" role="alert">
                {fieldErrors[field]}
              </span>
            )}
          </div>
        ))}
        <div className="form-group">
          <label htmlFor="anchor-currencies">Supported currencies (comma-separated)</label>
          <input
            id="anchor-currencies"
            type="text"
            value={form.supported_currencies}
            placeholder="USDC, NGN"
            onChange={e => setForm({ ...form, supported_currencies: e.target.value })}
            aria-invalid={!!fieldErrors.supported_currencies}
            aria-describedby={fieldErrors.supported_currencies ? 'anchor-currencies-error' : undefined}
          />
          {fieldErrors.supported_currencies && (
            <span id="anchor-currencies-error" className="field-error" role="alert">
              {fieldErrors.supported_currencies}
            </span>
          )}
        </div>
        <div className="form-group">
          <label htmlFor="anchor-countries">Supported countries (comma-separated)</label>
          <input
            id="anchor-countries"
            type="text"
            value={form.supported_countries}
            placeholder="NG, KE"
            onChange={e => setForm({ ...form, supported_countries: e.target.value })}
          />
        </div>
        <div className="form-group">
          <label htmlFor="anchor-kyc-required">
            <input
              id="anchor-kyc-required"
              type="checkbox"
              checked={form.kyc_required}
              onChange={e => setForm({ ...form, kyc_required: e.target.checked })}
            />
            KYC required
          </label>
          <label htmlFor="anchor-kyc-level">KYC level</label>
          <select
            id="anchor-kyc-level"
            value={form.kyc_level}
            onChange={e => setForm({ ...form, kyc_level: e.target.value as AnchorForm['kyc_level'] })}
          >
            <option value="basic">Basic</option>
            <option value="standard">Standard</option>
            <option value="full">Full</option>
          </select>
        </div>
        <button type="submit" className="btn-primary" disabled={saving}>
          {saving ? 'Saving…' : editingId ? 'Update Anchor' : 'Add Anchor'}
        </button>
      </form>
      {error && <div className="error" role="alert">{error}</div>}
      <hr />
      {loading ? <p>Loading…</p> : anchors.length === 0 ? <p>No anchors configured.</p> : (
        <ul style={{ listStyle: 'none', padding: 0 }}>
          {anchors.map(anchor => (
            <li key={anchor.id} style={{ padding: '12px', border: '1px solid #ddd', marginBottom: '8px', borderRadius: '4px' }}>
              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                <div>
                  <strong>{anchor.name}</strong>
                  <p style={{ margin: '4px 0', fontSize: '0.9em', color: '#666' }}>{anchor.domain}</p>
                </div>
                <div style={{ display: 'flex', gap: '8px' }}>
                  <button onClick={() => handleEdit(anchor)}>Edit</button>
                  <button onClick={() => setDeleteTarget(anchor)} style={{ color: '#e53e3e' }}>Delete</button>
                </div>
              </div>
            </li>
          ))}
        </ul>
      )}
    </div>
  )
}

export default AnchorManagement
