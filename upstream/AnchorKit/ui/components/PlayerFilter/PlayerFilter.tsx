import React, { useState, useCallback, useRef } from 'react';
import { EmptyState } from '../ui/EmptyState';
import {
  filterPlayers,
  PlayerFilters,
  Player,
  DEFAULT_FILTERS,
  FILTER_OPTIONS,
} from './filterPlayers';
import './PlayerFilter.css';

type FilterStatus = 'idle' | 'loading' | 'done';

export interface PlayerFilterProps {
  onFilter?: (filters: PlayerFilters) => Promise<Player[]>;
}

export function PlayerFilter({ onFilter = filterPlayers }: PlayerFilterProps) {
  const [filters, setFilters] = useState<PlayerFilters>(DEFAULT_FILTERS);
  const [players, setPlayers] = useState<Player[]>([]);
  const [status, setStatus] = useState<FilterStatus>('idle');
  const requestIdRef = useRef(0);

  const runFilter = useCallback(
    async (f: PlayerFilters) => {
      const id = ++requestIdRef.current;
      setStatus('loading');
      try {
        const result = await onFilter(f);
        if (id !== requestIdRef.current) return;
        setPlayers(result);
        setStatus('done');
      } catch {
        if (id !== requestIdRef.current) return;
        setPlayers([]);
        setStatus('done');
      }
    },
    [onFilter],
  );

  const handleFilterChange = (field: keyof PlayerFilters, value: string) => {
    const next = { ...filters, [field]: value };
    setFilters(next);
    runFilter(next);
  };

  const handleClearFilters = () => {
    setFilters(DEFAULT_FILTERS);
    runFilter(DEFAULT_FILTERS);
  };

  const showLoading   = status === 'loading';
  const showEmptyState = status === 'done' && players.length === 0;
  const showGrid      = status === 'done' && players.length > 0;

  return (
    <div className="player-filter">
      {/* ── Filter Controls ─────────────────────────────────────────── */}
      <div className="player-filter__controls">
        <div className="player-filter__field">
          <label htmlFor="filter-region" className="player-filter__label">
            Region
          </label>
          <select
            id="filter-region"
            className="player-filter__select"
            value={filters.region}
            onChange={(e) => handleFilterChange('region', e.target.value)}
          >
            <option value="">All Regions</option>
            {FILTER_OPTIONS.regions.map((r) => (
              <option key={r} value={r}>{r}</option>
            ))}
          </select>
        </div>

        <div className="player-filter__field">
          <label htmlFor="filter-position" className="player-filter__label">
            Position
          </label>
          <select
            id="filter-position"
            className="player-filter__select"
            value={filters.position}
            onChange={(e) => handleFilterChange('position', e.target.value)}
          >
            <option value="">All Positions</option>
            {FILTER_OPTIONS.positions.map((p) => (
              <option key={p} value={p}>{p}</option>
            ))}
          </select>
        </div>

        <div className="player-filter__field">
          <label htmlFor="filter-level" className="player-filter__label">
            Level
          </label>
          <select
            id="filter-level"
            className="player-filter__select"
            value={filters.level}
            onChange={(e) => handleFilterChange('level', e.target.value)}
          >
            <option value="">All Levels</option>
            {FILTER_OPTIONS.levels.map((l) => (
              <option key={l} value={l}>{l}</option>
            ))}
          </select>
        </div>
      </div>

      {/* ── Results Area ────────────────────────────────────────────── */}
      <div
        className="player-filter__results"
        aria-live="polite"
        aria-busy={showLoading}
      >
        {showLoading && (
          <div
            className="player-filter__loading"
            role="status"
            aria-label="Loading players"
          >
            <div className="player-filter__spinner" aria-hidden="true" />
            <span>Loading players…</span>
          </div>
        )}

        {showEmptyState && (
          <EmptyState
            icon="🔍"
            heading="No players found"
            subtext="Try adjusting your region, position, or level filter."
            action={
              <button
                className="player-filter__clear-btn"
                onClick={handleClearFilters}
              >
                Clear Filters
              </button>
            }
          />
        )}

        {showGrid && (
          <div
            className="player-filter__grid"
            role="list"
            aria-label="Player results"
          >
            {players.map((player) => (
              <div
                key={player.id}
                className="player-filter__card"
                role="listitem"
              >
                {player.avatar && (
                  <div className="player-filter__avatar" aria-hidden="true">
                    {player.avatar}
                  </div>
                )}
                <div className="player-filter__card-info">
                  <div className="player-filter__card-name">{player.name}</div>
                  <div className="player-filter__card-meta">
                    <span className="player-filter__badge">{player.position}</span>
                    <span className="player-filter__badge">{player.level}</span>
                    <span className="player-filter__badge player-filter__badge--muted">
                      {player.region}
                    </span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
