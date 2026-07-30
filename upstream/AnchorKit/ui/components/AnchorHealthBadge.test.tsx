import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import { AnchorHealthBadge } from './AnchorHealthBadge';

describe('AnchorHealthBadge', () => {
  describe('Tier boundaries', () => {
    it('labels score 0 as Poor', () => {
      render(<AnchorHealthBadge score={0} />);
      expect(screen.getByText('Poor')).toBeInTheDocument();
    });

    it('labels score 59 as Poor', () => {
      render(<AnchorHealthBadge score={59} />);
      expect(screen.getByText('Poor')).toBeInTheDocument();
    });

    it('labels score 60 as Fair', () => {
      render(<AnchorHealthBadge score={60} />);
      expect(screen.getByText('Fair')).toBeInTheDocument();
    });

    it('labels score 79 as Fair', () => {
      render(<AnchorHealthBadge score={79} />);
      expect(screen.getByText('Fair')).toBeInTheDocument();
    });

    it('labels score 80 as Healthy', () => {
      render(<AnchorHealthBadge score={80} />);
      expect(screen.getByText('Healthy')).toBeInTheDocument();
    });

    it('labels score 100 as Healthy', () => {
      render(<AnchorHealthBadge score={100} />);
      expect(screen.getByText('Healthy')).toBeInTheDocument();
    });
  });

  describe('Score clamping', () => {
    it('clamps negative scores to 0', () => {
      render(<AnchorHealthBadge score={-5} />);
      expect(screen.getByText('Poor')).toBeInTheDocument();
      expect(screen.getByText('· 0')).toBeInTheDocument();
    });

    it('clamps scores above 100 to 100', () => {
      render(<AnchorHealthBadge score={150} />);
      expect(screen.getByText('Healthy')).toBeInTheDocument();
      expect(screen.getByText('· 100')).toBeInTheDocument();
    });

    it('rounds fractional scores', () => {
      render(<AnchorHealthBadge score={79.6} />);
      expect(screen.getByText('· 80')).toBeInTheDocument();
      expect(screen.getByText('Healthy')).toBeInTheDocument();
    });
  });

  describe('showScore', () => {
    it('shows the numeric score by default', () => {
      render(<AnchorHealthBadge score={42} />);
      expect(screen.getByText('· 42')).toBeInTheDocument();
    });

    it('hides the numeric score when showScore is false', () => {
      render(<AnchorHealthBadge score={42} showScore={false} />);
      expect(screen.queryByText('· 42')).not.toBeInTheDocument();
    });
  });

  describe('Accessibility', () => {
    it('exposes a status role with a descriptive aria-label', () => {
      render(<AnchorHealthBadge score={85} />);
      const badge = screen.getByRole('status');
      expect(badge).toHaveAttribute('aria-label', 'Anchor health: Healthy (85/100)');
    });
  });

  describe('Hover tooltip', () => {
    it('shows the tooltip on mouse enter and hides it on mouse leave', () => {
      render(<AnchorHealthBadge score={72} />);
      const badge = screen.getByRole('status');

      expect(screen.queryByRole('tooltip')).not.toBeInTheDocument();

      fireEvent.mouseEnter(badge);
      expect(screen.getByRole('tooltip')).toHaveTextContent('Health score: 72/100');

      fireEvent.mouseLeave(badge);
      expect(screen.queryByRole('tooltip')).not.toBeInTheDocument();
    });
  });
});
