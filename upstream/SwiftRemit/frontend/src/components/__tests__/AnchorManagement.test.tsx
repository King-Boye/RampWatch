import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type { ComponentType } from 'react';
import AnchorManagementTsx from '../AnchorManagement.tsx';

function fill(values: Record<string, string>): void {
  for (const [label, value] of Object.entries(values)) {
    fireEvent.change(screen.getByLabelText(label), { target: { value } });
  }
}

interface Variant {
  name: string;
  Component: ComponentType;
  labels: { name: string; domain: string; dep: string; wit: string; min: string; max: string; currencies: string };
}

const VARIANTS: Variant[] = [
  {
    name: 'AnchorManagement.tsx',
    Component: AnchorManagementTsx,
    labels: { name: 'Anchor Name', domain: 'Domain', dep: 'Deposit fee (%)', wit: 'Withdrawal fee (%)', min: 'Minimum amount', max: 'Maximum amount', currencies: 'Supported currencies (comma-separated)' },
  },
];

describe.each(VARIANTS)('$name validation (#1720)', ({ Component, labels: L }) => {
  const VALID = {
    [L.name]: 'Test Anchor',
    [L.domain]: 'anchor.example',
    [L.dep]: '1.5',
    [L.wit]: '2',
    [L.min]: '10',
    [L.max]: '5000',
    [L.currencies]: 'USDC, NGN',
  };

  let fetchMock: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    fetchMock = vi.fn().mockResolvedValue({ ok: true, json: async () => [] });
    vi.stubGlobal('fetch', fetchMock);
  });

  afterEach(() => vi.unstubAllGlobals());

  const submit = () => fireEvent.click(screen.getByRole('button', { name: 'Add Anchor' }));
  const postCalls = () => fetchMock.mock.calls.filter(([, init]) => init?.method === 'POST');

  it('shows errors and does not submit when numbers are invalid', async () => {
    render(<Component />);
    await waitFor(() => expect(screen.queryByText('Loading…')).not.toBeInTheDocument());
    fill({ ...VALID, [L.max]: 'abc', [L.dep]: '' });
    submit();
    expect(await screen.findByText(/Maximum amount must be a number/)).toBeInTheDocument();
    expect(screen.getByText(/Deposit fee is required/)).toBeInTheDocument();
    expect(screen.getByLabelText(L.max)).toHaveAttribute('aria-invalid', 'true');
    expect(postCalls()).toHaveLength(0);
  });

  it('rejects max_amount of 0 instead of saving it', async () => {
    render(<Component />);
    await waitFor(() => expect(screen.queryByText('Loading…')).not.toBeInTheDocument());
    fill({ ...VALID, [L.min]: '0', [L.max]: '0' });
    submit();
    expect(await screen.findByText(/greater than 0/)).toBeInTheDocument();
    expect(postCalls()).toHaveLength(0);
  });

  it('submits parsed numbers when the form is valid', async () => {
    render(<Component />);
    await waitFor(() => expect(screen.queryByText('Loading…')).not.toBeInTheDocument());
    fill(VALID);
    submit();
    await waitFor(() => expect(postCalls()).toHaveLength(1));
    const body = JSON.parse(postCalls()[0][1].body);
    expect(body.fees).toEqual({ deposit_fee_percent: 1.5, withdrawal_fee_percent: 2 });
    expect(body.limits).toEqual({ min_amount: 10, max_amount: 5000 });
    expect(body.supported_currencies).toEqual(['USDC', 'NGN']);
  });
});
