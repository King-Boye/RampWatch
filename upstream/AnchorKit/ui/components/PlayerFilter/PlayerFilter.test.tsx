import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import { PlayerFilter } from './PlayerFilter';
import { Player, DEFAULT_FILTERS } from './filterPlayers';

// ── Fixtures ─────────────────────────────────────────────────────────────────

const ALICE: Player = {
  id: '1',
  name: 'Alice',
  region: 'North America',
  position: 'Mid',
  level: 'Diamond',
};
const BOB: Player = {
  id: '2',
  name: 'Bob',
  region: 'Europe',
  position: 'Support',
  level: 'Gold',
};

function resolvedWith(players: Player[]) {
  return jest.fn().mockResolvedValue(players);
}

// ── Initial page load ─────────────────────────────────────────────────────────

describe('PlayerFilter – initial page load', () => {
  it('renders all three filter controls', () => {
    render(<PlayerFilter onFilter={resolvedWith([])} />);
    expect(screen.getByLabelText('Region')).toBeInTheDocument();
    expect(screen.getByLabelText('Position')).toBeInTheDocument();
    expect(screen.getByLabelText('Level')).toBeInTheDocument();
  });

  it('all selects default to their empty ("All …") value', () => {
    render(<PlayerFilter onFilter={resolvedWith([])} />);
    expect((screen.getByLabelText('Region') as HTMLSelectElement).value).toBe('');
    expect((screen.getByLabelText('Position') as HTMLSelectElement).value).toBe('');
    expect((screen.getByLabelText('Level') as HTMLSelectElement).value).toBe('');
  });

  it('does NOT show the empty state before any filter has run', () => {
    render(<PlayerFilter onFilter={resolvedWith([])} />);
    expect(screen.queryByText('No players found')).not.toBeInTheDocument();
  });

  it('does NOT show the loading indicator on initial mount', () => {
    render(<PlayerFilter onFilter={resolvedWith([])} />);
    expect(
      screen.queryByRole('status', { name: /loading/i }),
    ).not.toBeInTheDocument();
  });

  it('does NOT show the results grid on initial mount', () => {
    render(<PlayerFilter onFilter={resolvedWith([ALICE])} />);
    expect(
      screen.queryByRole('list', { name: /player results/i }),
    ).not.toBeInTheDocument();
  });

  it('does NOT call onFilter on mount', () => {
    const onFilter = resolvedWith([]);
    render(<PlayerFilter onFilter={onFilter} />);
    expect(onFilter).not.toHaveBeenCalled();
  });
});

// ── Loading state ─────────────────────────────────────────────────────────────

describe('PlayerFilter – loading state', () => {
  it('shows the loading indicator while a filter request is in flight', async () => {
    let resolve!: (players: Player[]) => void;
    const pending = new Promise<Player[]>((res) => { resolve = res; });

    render(<PlayerFilter onFilter={() => pending} />);
    fireEvent.change(screen.getByLabelText('Region'), {
      target: { value: 'Europe' },
    });

    expect(
      screen.getByRole('status', { name: /loading/i }),
    ).toBeInTheDocument();

    resolve([BOB]);
    await waitFor(() =>
      expect(
        screen.queryByRole('status', { name: /loading/i }),
      ).not.toBeInTheDocument(),
    );
  });

  it('does NOT show the empty state while loading', async () => {
    let resolve!: (players: Player[]) => void;
    const pending = new Promise<Player[]>((res) => { resolve = res; });

    render(<PlayerFilter onFilter={() => pending} />);
    fireEvent.change(screen.getByLabelText('Position'), {
      target: { value: 'Mid' },
    });

    expect(screen.queryByText('No players found')).not.toBeInTheDocument();

    resolve([]);
    await waitFor(() =>
      expect(screen.getByText('No players found')).toBeInTheDocument(),
    );
  });

  it('does NOT show the results grid while loading', async () => {
    let resolve!: (players: Player[]) => void;
    const pending = new Promise<Player[]>((res) => { resolve = res; });

    render(<PlayerFilter onFilter={() => pending} />);
    fireEvent.change(screen.getByLabelText('Level'), {
      target: { value: 'Gold' },
    });

    expect(
      screen.queryByRole('list', { name: /player results/i }),
    ).not.toBeInTheDocument();

    resolve([BOB]);
    await waitFor(() =>
      expect(screen.getByRole('list', { name: /player results/i })).toBeInTheDocument(),
    );
  });
});

// ── Empty state ───────────────────────────────────────────────────────────────

describe('PlayerFilter – empty state', () => {
  it('shows the empty state heading after a filter returns no results', async () => {
    render(<PlayerFilter onFilter={resolvedWith([])} />);
    fireEvent.change(screen.getByLabelText('Region'), {
      target: { value: 'Europe' },
    });

    await waitFor(() =>
      expect(screen.getByText('No players found')).toBeInTheDocument(),
    );
  });

  it('shows the instructional subtext alongside the empty state', async () => {
    render(<PlayerFilter onFilter={resolvedWith([])} />);
    fireEvent.change(screen.getByLabelText('Level'), {
      target: { value: 'Master' },
    });

    await waitFor(() =>
      expect(
        screen.getByText(
          'Try adjusting your region, position, or level filter.',
        ),
      ).toBeInTheDocument(),
    );
  });

  it('shows the "Clear Filters" button inside the empty state', async () => {
    render(<PlayerFilter onFilter={resolvedWith([])} />);
    fireEvent.change(screen.getByLabelText('Position'), {
      target: { value: 'Support' },
    });

    await waitFor(() =>
      expect(
        screen.getByRole('button', { name: 'Clear Filters' }),
      ).toBeInTheDocument(),
    );
  });

  it('does NOT show the empty state when filter returns results', async () => {
    render(<PlayerFilter onFilter={resolvedWith([ALICE])} />);
    fireEvent.change(screen.getByLabelText('Region'), {
      target: { value: 'North America' },
    });

    await waitFor(() =>
      expect(screen.getByText('Alice')).toBeInTheDocument(),
    );
    expect(screen.queryByText('No players found')).not.toBeInTheDocument();
  });

  it('transitions from a results grid to an empty state when a stricter filter yields zero results', async () => {
    const onFilter = jest
      .fn()
      .mockResolvedValueOnce([ALICE])
      .mockResolvedValueOnce([]);

    render(<PlayerFilter onFilter={onFilter} />);

    fireEvent.change(screen.getByLabelText('Region'), {
      target: { value: 'North America' },
    });
    await waitFor(() => expect(screen.getByText('Alice')).toBeInTheDocument());

    fireEvent.change(screen.getByLabelText('Level'), {
      target: { value: 'Bronze' },
    });
    await waitFor(() =>
      expect(screen.getByText('No players found')).toBeInTheDocument(),
    );
    expect(screen.queryByText('Alice')).not.toBeInTheDocument();
  });

  it('transitions from an empty state back to a results grid when a looser filter returns results', async () => {
    const onFilter = jest
      .fn()
      .mockResolvedValueOnce([])
      .mockResolvedValueOnce([ALICE, BOB]);

    render(<PlayerFilter onFilter={onFilter} />);

    fireEvent.change(screen.getByLabelText('Level'), {
      target: { value: 'Challenger' },
    });
    await waitFor(() =>
      expect(screen.getByText('No players found')).toBeInTheDocument(),
    );

    fireEvent.change(screen.getByLabelText('Level'), { target: { value: '' } });
    await waitFor(() =>
      expect(screen.queryByText('No players found')).not.toBeInTheDocument(),
    );
    expect(screen.getByText('Alice')).toBeInTheDocument();
    expect(screen.getByText('Bob')).toBeInTheDocument();
  });
});

// ── Results grid ──────────────────────────────────────────────────────────────

describe('PlayerFilter – results grid', () => {
  it('renders a card for each returned player', async () => {
    render(<PlayerFilter onFilter={resolvedWith([ALICE, BOB])} />);
    fireEvent.change(screen.getByLabelText('Region'), {
      target: { value: 'Europe' },
    });

    await waitFor(() => {
      expect(screen.getByText('Alice')).toBeInTheDocument();
      expect(screen.getByText('Bob')).toBeInTheDocument();
    });
  });

  it('shows the player position badge', async () => {
    render(<PlayerFilter onFilter={resolvedWith([ALICE])} />);
    fireEvent.change(screen.getByLabelText('Position'), {
      target: { value: 'Mid' },
    });

    await waitFor(() => expect(screen.getByText('Alice')).toBeInTheDocument());
    expect(screen.getAllByText('Mid').length).toBeGreaterThanOrEqual(1);
  });

  it('shows the player level badge', async () => {
    render(<PlayerFilter onFilter={resolvedWith([ALICE])} />);
    fireEvent.change(screen.getByLabelText('Level'), {
      target: { value: 'Diamond' },
    });

    await waitFor(() => expect(screen.getByText('Alice')).toBeInTheDocument());
    expect(screen.getAllByText('Diamond').length).toBeGreaterThanOrEqual(1);
  });

  it('shows the player region badge', async () => {
    render(<PlayerFilter onFilter={resolvedWith([ALICE])} />);
    fireEvent.change(screen.getByLabelText('Region'), {
      target: { value: 'North America' },
    });

    await waitFor(() => expect(screen.getByText('Alice')).toBeInTheDocument());
    expect(screen.getAllByText('North America').length).toBeGreaterThanOrEqual(1);
  });
});

// ── Clear Filters ─────────────────────────────────────────────────────────────

describe('PlayerFilter – Clear Filters button', () => {
  it('resets all three filter selects to their default (empty) value', async () => {
    const onFilter = jest
      .fn()
      .mockResolvedValueOnce([])
      .mockResolvedValue([ALICE]);

    render(<PlayerFilter onFilter={onFilter} />);

    fireEvent.change(screen.getByLabelText('Region'), {
      target: { value: 'Europe' },
    });
    await waitFor(() =>
      expect(
        screen.getByRole('button', { name: 'Clear Filters' }),
      ).toBeInTheDocument(),
    );

    fireEvent.click(screen.getByRole('button', { name: 'Clear Filters' }));

    await waitFor(() => {
      expect(
        (screen.getByLabelText('Region') as HTMLSelectElement).value,
      ).toBe('');
      expect(
        (screen.getByLabelText('Position') as HTMLSelectElement).value,
      ).toBe('');
      expect(
        (screen.getByLabelText('Level') as HTMLSelectElement).value,
      ).toBe('');
    });
  });

  it('immediately retriggers onFilter with DEFAULT_FILTERS after clearing', async () => {
    const onFilter = jest
      .fn()
      .mockResolvedValueOnce([])
      .mockResolvedValue([ALICE]);

    render(<PlayerFilter onFilter={onFilter} />);

    fireEvent.change(screen.getByLabelText('Position'), {
      target: { value: 'Support' },
    });
    await waitFor(() =>
      expect(screen.getByText('No players found')).toBeInTheDocument(),
    );

    fireEvent.click(screen.getByRole('button', { name: 'Clear Filters' }));

    await waitFor(() =>
      expect(screen.getByText('Alice')).toBeInTheDocument(),
    );

    expect(onFilter).toHaveBeenCalledTimes(2);
    expect(onFilter).toHaveBeenLastCalledWith(DEFAULT_FILTERS);
  });

  it('hides the empty state after clearing when the re-triggered filter returns results', async () => {
    const onFilter = jest
      .fn()
      .mockResolvedValueOnce([])
      .mockResolvedValue([BOB]);

    render(<PlayerFilter onFilter={onFilter} />);

    fireEvent.change(screen.getByLabelText('Level'), {
      target: { value: 'Challenger' },
    });
    await waitFor(() =>
      expect(screen.getByText('No players found')).toBeInTheDocument(),
    );

    fireEvent.click(screen.getByRole('button', { name: 'Clear Filters' }));

    await waitFor(() =>
      expect(screen.queryByText('No players found')).not.toBeInTheDocument(),
    );
    expect(screen.getByText('Bob')).toBeInTheDocument();
  });

  it('shows the loading indicator briefly after Clear Filters is clicked', async () => {
    let resolve!: (p: Player[]) => void;
    const onFilter = jest
      .fn()
      .mockResolvedValueOnce([])
      .mockReturnValueOnce(new Promise<Player[]>((r) => { resolve = r; }));

    render(<PlayerFilter onFilter={onFilter} />);

    fireEvent.change(screen.getByLabelText('Region'), {
      target: { value: 'Europe' },
    });
    await waitFor(() =>
      expect(screen.getByText('No players found')).toBeInTheDocument(),
    );

    fireEvent.click(screen.getByRole('button', { name: 'Clear Filters' }));

    expect(
      screen.getByRole('status', { name: /loading/i }),
    ).toBeInTheDocument();

    resolve([ALICE]);
    await waitFor(() =>
      expect(
        screen.queryByRole('status', { name: /loading/i }),
      ).not.toBeInTheDocument(),
    );
  });
});

// ── Filter controls – individual changes ─────────────────────────────────────

describe('PlayerFilter – filter control changes', () => {
  it('calls onFilter with the correct region when region changes', async () => {
    const onFilter = resolvedWith([ALICE]);
    render(<PlayerFilter onFilter={onFilter} />);

    fireEvent.change(screen.getByLabelText('Region'), {
      target: { value: 'North America' },
    });

    await waitFor(() =>
      expect(onFilter).toHaveBeenCalledWith({
        region: 'North America',
        position: '',
        level: '',
      }),
    );
  });

  it('calls onFilter with the correct position when position changes', async () => {
    const onFilter = resolvedWith([ALICE]);
    render(<PlayerFilter onFilter={onFilter} />);

    fireEvent.change(screen.getByLabelText('Position'), {
      target: { value: 'Mid' },
    });

    await waitFor(() =>
      expect(onFilter).toHaveBeenCalledWith({
        region: '',
        position: 'Mid',
        level: '',
      }),
    );
  });

  it('calls onFilter with the correct level when level changes', async () => {
    const onFilter = resolvedWith([BOB]);
    render(<PlayerFilter onFilter={onFilter} />);

    fireEvent.change(screen.getByLabelText('Level'), {
      target: { value: 'Gold' },
    });

    await waitFor(() =>
      expect(onFilter).toHaveBeenCalledWith({
        region: '',
        position: '',
        level: 'Gold',
      }),
    );
  });

  it('accumulates filter values correctly across multiple changes', async () => {
    const onFilter = jest.fn().mockResolvedValue([ALICE]);
    render(<PlayerFilter onFilter={onFilter} />);

    fireEvent.change(screen.getByLabelText('Region'), {
      target: { value: 'North America' },
    });
    await waitFor(() => expect(onFilter).toHaveBeenCalledTimes(1));

    fireEvent.change(screen.getByLabelText('Position'), {
      target: { value: 'Mid' },
    });
    await waitFor(() =>
      expect(onFilter).toHaveBeenLastCalledWith({
        region: 'North America',
        position: 'Mid',
        level: '',
      }),
    );
  });

  it('only one request is in-flight at a time (stale responses are discarded)', async () => {
    let resolveFirst!: (p: Player[]) => void;
    let resolveSecond!: (p: Player[]) => void;

    const onFilter = jest
      .fn()
      .mockReturnValueOnce(
        new Promise<Player[]>((r) => { resolveFirst = r; }),
      )
      .mockReturnValueOnce(
        new Promise<Player[]>((r) => { resolveSecond = r; }),
      );

    render(<PlayerFilter onFilter={onFilter} />);

    // Trigger two rapid changes
    fireEvent.change(screen.getByLabelText('Region'), {
      target: { value: 'Europe' },
    });
    fireEvent.change(screen.getByLabelText('Position'), {
      target: { value: 'Support' },
    });

    // Resolve the second (newer) request first
    resolveSecond([BOB]);
    await waitFor(() => expect(screen.getByText('Bob')).toBeInTheDocument());

    // Resolve the stale first request – its result should be silently dropped
    resolveFirst([ALICE]);
    await waitFor(() => expect(screen.getByText('Bob')).toBeInTheDocument());
    // Alice should never appear because that was the stale response
    expect(screen.queryByText('Alice')).not.toBeInTheDocument();
  });
});
