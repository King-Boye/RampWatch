import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { describe, it, expect, vi, beforeEach } from 'vitest'
import * as FreighterAPI from '@stellar/freighter-api'
import WalletConnect from '../WalletConnect'

vi.mock('@stellar/freighter-api')

describe('WalletConnect Error Handling', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  describe('Freighter not installed', () => {
    it('should show install prompt when Freighter is not installed', async () => {
      FreighterAPI.isConnected.mockResolvedValue(false)
      
      render(<WalletConnect walletAddress={null} setWalletAddress={vi.fn()} />)
      
      fireEvent.click(screen.getByTestId('connect-wallet-btn'))
      
      await waitFor(() => {
        expect(screen.getByText(/Freighter wallet not found/i)).toBeInTheDocument()
        expect(screen.getByRole('link', { name: /Install Freighter Wallet/i })).toHaveAttribute(
          'href',
          'https://www.freighter.app/'
        )
      })
    })
  })

  describe('Network mismatch', () => {
    it('should show network mismatch error with guidance', async () => {
      FreighterAPI.isConnected.mockResolvedValue(true)
      FreighterAPI.requestAccess.mockRejectedValue(
        new Error('Network mismatch: wallet on mainnet but testnet expected')
      )
      
      render(<WalletConnect walletAddress={null} setWalletAddress={vi.fn()} />)
      
      fireEvent.click(screen.getByTestId('connect-wallet-btn'))
      
      await waitFor(() => {
        expect(screen.getByText(/network/i)).toBeInTheDocument()
        expect(screen.getByText(/switch your Freighter wallet/i)).toBeInTheDocument()
      })
    })
  })

  describe('User rejection', () => {
    it('should show retry option when user rejects connection', async () => {
      FreighterAPI.isConnected.mockResolvedValue(true)
      FreighterAPI.requestAccess.mockRejectedValue(
        new Error('User denied access')
      )
      
      render(<WalletConnect walletAddress={null} setWalletAddress={vi.fn()} />)
      
      fireEvent.click(screen.getByTestId('connect-wallet-btn'))
      
      await waitFor(() => {
        expect(screen.getByText(/Connection rejected/i)).toBeInTheDocument()
        expect(screen.getByText(/Retry Connection/i)).toBeInTheDocument()
      })
    })

    it('should allow retry after rejection', async () => {
      FreighterAPI.isConnected.mockResolvedValue(true)
      FreighterAPI.requestAccess
        .mockRejectedValueOnce(new Error('User denied access'))
        .mockResolvedValueOnce(undefined)
      FreighterAPI.getPublicKey.mockResolvedValue('GADDRESS')
      
      const setWalletAddress = vi.fn()
      render(<WalletConnect walletAddress={null} setWalletAddress={setWalletAddress} />)
      
      fireEvent.click(screen.getByTestId('connect-wallet-btn'))
      
      await waitFor(() => {
        expect(screen.getByText(/Connection rejected/i)).toBeInTheDocument()
      })
      
      fireEvent.click(screen.getByText(/Retry Connection/i))
      
      await waitFor(() => {
        expect(setWalletAddress).toHaveBeenCalledWith('GADDRESS')
      })
    })
  })

  describe('Generic error', () => {
    it('should show retry button for generic errors', async () => {
      FreighterAPI.isConnected.mockResolvedValue(true)
      FreighterAPI.requestAccess.mockRejectedValue(
        new Error('Something went wrong')
      )
      
      render(<WalletConnect walletAddress={null} setWalletAddress={vi.fn()} />)
      
      fireEvent.click(screen.getByTestId('connect-wallet-btn'))
      
      await waitFor(() => {
        expect(screen.getByText(/Something went wrong/i)).toBeInTheDocument()
        expect(screen.getByRole('button', { name: /Retry/i })).toBeInTheDocument()
      })
    })
  })

  describe('Success flow', () => {
    it('should show connected state with disconnect button on success', async () => {
      FreighterAPI.isConnected.mockResolvedValue(true)
      FreighterAPI.requestAccess.mockResolvedValue(undefined)
      FreighterAPI.getPublicKey.mockResolvedValue('GBDHJKD7NDHJKSD7HJKSJHD7JKSHD7JKSHDKJH7')
      
      const setWalletAddress = vi.fn()
      render(<WalletConnect walletAddress={null} setWalletAddress={setWalletAddress} />)
      
      fireEvent.click(screen.getByTestId('connect-wallet-btn'))
      
      await waitFor(() => {
        expect(setWalletAddress).toHaveBeenCalledWith('GBDHJKD7NDHJKSD7HJKSJHD7JKSHD7JKSHDKJH7')
      })
    })

    it('should display connected wallet address truncated', () => {
      render(
        <WalletConnect 
          walletAddress="GBDHJKD7NDHJKSD7HJKSJHD7JKSHD7JKSHDKJH7" 
          setWalletAddress={vi.fn()} 
        />
      )
      
      expect(screen.getByText(/Connected: GBDHJKD7...JKSHDKJH7/)).toBeInTheDocument()
      expect(screen.getByRole('button', { name: /Disconnect/i })).toBeInTheDocument()
    })

    it('should disconnect wallet on button click', () => {
      const setWalletAddress = vi.fn()
      render(
        <WalletConnect 
          walletAddress="GBDHJKD7NDHJKSD7HJKSJHD7JKSHD7JKSHDKJH7" 
          setWalletAddress={setWalletAddress} 
        />
      )
      
      fireEvent.click(screen.getByRole('button', { name: /Disconnect/i }))
      
      expect(setWalletAddress).toHaveBeenCalledWith(null)
    })
  })

  describe('Loading state', () => {
    it('should show loading text while connecting', async () => {
      FreighterAPI.isConnected.mockImplementation(() => new Promise(() => {}))
      
      render(<WalletConnect walletAddress={null} setWalletAddress={vi.fn()} />)
      
      fireEvent.click(screen.getByTestId('connect-wallet-btn'))
      
      expect(screen.getByText(/Connecting.../)).toBeInTheDocument()
      expect(screen.getByTestId('connect-wallet-btn')).toBeDisabled()
    })
  })
})
