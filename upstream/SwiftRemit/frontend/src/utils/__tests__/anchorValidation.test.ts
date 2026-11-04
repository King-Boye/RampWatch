import { describe, expect, it } from 'vitest';
import { parseList, parseStrictDecimal, validateAnchorNumbers } from '../anchorValidation';

const valid = {
  deposit_fee_percent: '1.5',
  withdrawal_fee_percent: '0',
  min_amount: '10',
  max_amount: '10000',
};

describe('parseStrictDecimal', () => {
  it.each([
    ['10', 10],
    ['0', 0],
    ['2.5', 2.5],
    [' 12 ', 12],
  ])('accepts %j', (raw, expected) => {
    expect(parseStrictDecimal(raw)).toBe(expected);
  });

  it.each(['', 'abc', '1e3', '0x10', '-1', '+5', '1.', '.5', '1,000', 'NaN', 'Infinity', '12abc'])(
    'rejects %j',
    (raw) => {
      expect(parseStrictDecimal(raw)).toBeNull();
    },
  );
});

describe('validateAnchorNumbers', () => {
  it('accepts valid input and returns parsed numbers', () => {
    const r = validateAnchorNumbers(valid);
    expect(r.valid).toBe(true);
    expect(r.values).toEqual({ deposit_fee_percent: 1.5, withdrawal_fee_percent: 0, min_amount: 10, max_amount: 10000 });
  });

  it('allows an explicit 0% fee but not an empty one', () => {
    expect(validateAnchorNumbers({ ...valid, deposit_fee_percent: '0' }).valid).toBe(true);
    const r = validateAnchorNumbers({ ...valid, deposit_fee_percent: '' });
    expect(r.valid).toBe(false);
    expect(r.errors.deposit_fee_percent).toMatch(/required/);
  });

  it('never coerces invalid input to 0', () => {
    const r = validateAnchorNumbers({ ...valid, max_amount: 'abc' });
    expect(r.valid).toBe(false);
    expect(r.values).toBeNull();
    expect(r.errors.max_amount).toMatch(/must be a number/);
  });

  it('rejects fees over 100%', () => {
    expect(validateAnchorNumbers({ ...valid, withdrawal_fee_percent: '150' }).errors.withdrawal_fee_percent).toMatch(/0 and 100/);
  });

  it('rejects max_amount of 0', () => {
    expect(validateAnchorNumbers({ ...valid, min_amount: '0', max_amount: '0' }).errors.max_amount).toMatch(/greater than 0/);
  });

  it('rejects min greater than max', () => {
    expect(validateAnchorNumbers({ ...valid, min_amount: '500', max_amount: '100' }).errors.max_amount).toMatch(/minimum/);
  });
});

describe('parseList', () => {
  it('trims and drops empty entries', () => {
    expect(parseList(' USDC, ,NGN ,')).toEqual(['USDC', 'NGN']);
    expect(parseList('')).toEqual([]);
  });
});
