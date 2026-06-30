import React, { useState } from 'react';

export interface AnchorHealthBadgeProps {
  /** Health score 0–100 */
  score: number;
  /** Optionally show the numeric score inside the badge */
  showScore?: boolean;
  className?: string;
}

type Tier = { label: string; color: string; bg: string; border: string };

function getTier(score: number): Tier {
  if (score >= 80) return { label: 'Healthy', color: '#166534', bg: '#dcfce7', border: '#86efac' };
  if (score >= 60) return { label: 'Fair',    color: '#92400e', bg: '#fef9c3', border: '#fde047' };
  return               { label: 'Poor',     color: '#991b1b', bg: '#fee2e2', border: '#fca5a5' };
}

export function AnchorHealthBadge({ score, showScore = true, className }: AnchorHealthBadgeProps) {
  const clamped = Math.max(0, Math.min(100, Math.round(score)));
  const tier = getTier(clamped);
  const [hovered, setHovered] = useState(false);

  return (
    <span
      role="status"
      aria-label={`Anchor health: ${tier.label} (${clamped}/100)`}
      className={className}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      style={{
        position: 'relative',
        display: 'inline-flex',
        alignItems: 'center',
        gap: '4px',
        padding: '2px 8px',
        borderRadius: '9999px',
        border: `1px solid ${tier.border}`,
        background: tier.bg,
        color: tier.color,
        fontSize: '0.75rem',
        fontWeight: 600,
        lineHeight: '1.5',
        userSelect: 'none',
        cursor: 'default',
        whiteSpace: 'nowrap',
      }}
    >
      <span aria-hidden="true" style={{ fontSize: '0.6rem' }}>●</span>
      {tier.label}
      {showScore && <span style={{ opacity: 0.75 }}>· {clamped}</span>}

      {hovered && (
        <span
          role="tooltip"
          style={{
            position: 'absolute',
            bottom: 'calc(100% + 6px)',
            left: '50%',
            transform: 'translateX(-50%)',
            background: '#1e293b',
            color: '#f8fafc',
            fontSize: '0.7rem',
            padding: '4px 8px',
            borderRadius: '4px',
            whiteSpace: 'nowrap',
            pointerEvents: 'none',
            zIndex: 10,
          }}
        >
          Health score: {clamped}/100
        </span>
      )}
    </span>
  );
}
