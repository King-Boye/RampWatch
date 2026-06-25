import { filterPlayers, DEFAULT_FILTERS } from './filterPlayers';

describe('filterPlayers', () => {
  it('returns a promise', () => {
    expect(filterPlayers(DEFAULT_FILTERS)).toBeInstanceOf(Promise);
  });

  it('returns all players when all filters are empty', async () => {
    const result = await filterPlayers(DEFAULT_FILTERS);
    expect(result.length).toBeGreaterThan(0);
  });

  it('filters by region only', async () => {
    const result = await filterPlayers({ ...DEFAULT_FILTERS, region: 'Europe' });
    expect(result.length).toBeGreaterThan(0);
    expect(result.every((p) => p.region === 'Europe')).toBe(true);
  });

  it('filters by position only', async () => {
    const result = await filterPlayers({ ...DEFAULT_FILTERS, position: 'Mid' });
    expect(result.length).toBeGreaterThan(0);
    expect(result.every((p) => p.position === 'Mid')).toBe(true);
  });

  it('filters by level only', async () => {
    const result = await filterPlayers({ ...DEFAULT_FILTERS, level: 'Diamond' });
    expect(result.length).toBeGreaterThan(0);
    expect(result.every((p) => p.level === 'Diamond')).toBe(true);
  });

  it('filters by all three criteria simultaneously', async () => {
    const result = await filterPlayers({
      region: 'North America',
      position: 'Mid',
      level: 'Diamond',
    });
    expect(
      result.every(
        (p) =>
          p.region === 'North America' &&
          p.position === 'Mid' &&
          p.level === 'Diamond',
      ),
    ).toBe(true);
  });

  it('returns an empty array when no players match the filters', async () => {
    const result = await filterPlayers({
      region: 'North America',
      position: 'Support',
      level: 'Challenger',
    });
    expect(result).toEqual([]);
  });

  it('each returned player has the required shape', async () => {
    const result = await filterPlayers(DEFAULT_FILTERS);
    for (const player of result) {
      expect(player).toHaveProperty('id');
      expect(player).toHaveProperty('name');
      expect(player).toHaveProperty('region');
      expect(player).toHaveProperty('position');
      expect(player).toHaveProperty('level');
    }
  });

  it('DEFAULT_FILTERS has empty strings for all three keys', () => {
    expect(DEFAULT_FILTERS.region).toBe('');
    expect(DEFAULT_FILTERS.position).toBe('');
    expect(DEFAULT_FILTERS.level).toBe('');
  });

  it('does not mutate the filters object', async () => {
    const filters = { region: 'Europe', position: '', level: '' };
    const copy = { ...filters };
    await filterPlayers(filters);
    expect(filters).toEqual(copy);
  });
});
