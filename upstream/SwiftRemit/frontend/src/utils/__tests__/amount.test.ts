import { describe, expect, it } from 'vitest';
import { MAX_STROOPS, amountToStroops, parseAmount, stroopsToAmount } from '../amount';

describe('parseAmount', () => {
  it.each([
    ['1', 10_000_000n],
    ['0.5', 5_000_000n],
    ['12.34', 123_400_000n],
    ['0.0000001', 1n],
    ['100.1234567', 1_001_234_567n],
    ['922337203685.4775807', MAX_STROOPS],
  ])('accepts %j', (input, stroops) => {
    expect(parseAmount(input)).toEqual({ ok: true, stroops });
  });

  it.each([
    ['1e3'],
    ['1E3'],
    [' 12 '],
    ['12 '],
    ['0x10'],
    ['0b11'],
    ['Infinity'],
    ['NaN'],
    ['abc'],
    ['12abc'],
    ['1,000'],
    ['1.'],
    ['.5'],
    ['+5'],
    ['01'],
    ['1.12345678'],
    ['1..2'],
  ])('rejects %j as a format error', (input) => {
    expect(parseAmount(input)).toEqual({ ok: false, error: 'format' });
  });

  it('reports empty input as required', () => {
    expect(parseAmount('')).toEqual({ ok: false, error: 'required' });
  });

  it('reports zero and negatives as not positive', () => {
    expect(parseAmount('0')).toEqual({ ok: false, error: 'notPositive' });
    expect(parseAmount('0.0000000')).toEqual({ ok: false, error: 'notPositive' });
    expect(parseAmount('-5')).toEqual({ ok: false, error: 'notPositive' });
  });

  it('rejects amounts above int64 stroops', () => {
    expect(parseAmount('922337203685.4775808')).toEqual({ ok: false, error: 'tooLarge' });
  });
});

describe('stroopsToAmount', () => {
  it('formats without exponent notation or trailing zeros', () => {
    expect(stroopsToAmount(1n)).toBe('0.0000001');
    expect(stroopsToAmount(125_000_000n)).toBe('12.5');
    expect(stroopsToAmount(10_000_000n)).toBe('1');
    expect(stroopsToAmount(MAX_STROOPS)).toBe('922337203685.4775807');
  });

  it('round-trips with amountToStroops', () => {
    for (const s of ['0.0000001', '1', '42.42', '100.1234567']) {
      expect(stroopsToAmount(amountToStroops(s)!)).toBe(s);
    }
  });
});
