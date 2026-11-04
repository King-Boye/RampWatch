import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import AddressBook from '../AddressBook'
import { recipientsKey, LEGACY_RECIPIENTS_KEY } from '../../utils/recipientStorage'

const WALLET = 'G' + 'A'.repeat(55)
const OTHER_WALLET = 'G' + 'B'.repeat(55)
const VALID_ADDRESS = 'G' + 'C'.repeat(55)

describe('AddressBook Component', () => {
  beforeEach(() => {
    localStorage.clear()
    vi.clearAllMocks()
  })

  afterEach(() => {
    localStorage.clear()
  })

  it('should render address book with add button', () => {
    render(<AddressBook onSelectRecipient={vi.fn()} walletAddress={WALLET} />)
    expect(screen.getByText('Saved Recipients')).toBeInTheDocument()
    expect(screen.getByTestId('add-recipient-btn')).toBeInTheDocument()
  })

  it('should show empty state when no recipients', () => {
    render(<AddressBook onSelectRecipient={vi.fn()} walletAddress={WALLET} />)
    expect(screen.getByText('No saved recipients. Add one to get started!')).toBeInTheDocument()
  })

  describe('Adding recipients', () => {
    it('should open form when add button is clicked', () => {
      render(<AddressBook onSelectRecipient={vi.fn()} walletAddress={WALLET} />)
      fireEvent.click(screen.getByTestId('add-recipient-btn'))
      expect(screen.getByTestId('recipient-name-input')).toBeInTheDocument()
      expect(screen.getByTestId('recipient-address-input')).toBeInTheDocument()
      expect(screen.getByTestId('recipient-country-input')).toBeInTheDocument()
    })

    it('should show validation error for empty name', () => {
      render(<AddressBook onSelectRecipient={vi.fn()} walletAddress={WALLET} />)
      
      fireEvent.click(screen.getByTestId('add-recipient-btn'))
      fireEvent.change(screen.getByTestId('recipient-address-input'), { 
        target: { value: 'GBDHJKD7NDHJKSD7HJKSJHD7JKSHD7JKSHDKJH7' } 
      })
      fireEvent.change(screen.getByTestId('recipient-country-input'), { target: { value: 'Nigeria' } })
      
      fireEvent.click(screen.getByTestId('save-recipient-btn'))
      
      expect(screen.getByText('Name is required')).toBeInTheDocument()
    })

    it('should show validation error for invalid address', () => {
      render(<AddressBook onSelectRecipient={vi.fn()} walletAddress={WALLET} />)
      
      fireEvent.click(screen.getByTestId('add-recipient-btn'))
      fireEvent.change(screen.getByTestId('recipient-name-input'), { target: { value: 'John' } })
      fireEvent.change(screen.getByTestId('recipient-address-input'), { target: { value: 'INVALID' } })
      fireEvent.change(screen.getByTestId('recipient-country-input'), { target: { value: 'Nigeria' } })
      
      fireEvent.click(screen.getByTestId('save-recipient-btn'))
      
      expect(screen.getByText('Invalid Stellar address')).toBeInTheDocument()
    })
  })

  describe('Recipient display', () => {
    it('should display saved recipient from localStorage', () => {
      const savedRecipients = [
        {
          id: 1,
          name: 'John Doe',
          address: 'GBDHJKD7NDHJKSD7HJKSJHD7JKSHD7JKSHDKJH7',
          country: 'Nigeria',
          memo: ''
        }
      ]
      localStorage.setItem('swiftremit_recipients', JSON.stringify(savedRecipients))
      
      render(<AddressBook onSelectRecipient={vi.fn()} walletAddress={WALLET} />)
      
      expect(screen.getByText('John Doe')).toBeInTheDocument()
      expect(screen.getByText('Nigeria')).toBeInTheDocument()
    })
  })

  describe('Editing recipients', () => {
    beforeEach(() => {
      const savedRecipients = [
        {
          id: 1,
          name: 'John Doe',
          address: 'GBDHJKD7NDHJKSD7HJKSJHD7JKSHD7JKSHDKJH7',
          country: 'Nigeria',
          memo: 'Monthly support'
        }
      ]
      localStorage.setItem('swiftremit_recipients', JSON.stringify(savedRecipients))
    })

    it('should load recipients from localStorage', () => {
      render(<AddressBook onSelectRecipient={vi.fn()} walletAddress={WALLET} />)
      expect(screen.getByText('John Doe')).toBeInTheDocument()
    })

    it('should delete recipient', () => {
      render(<AddressBook onSelectRecipient={vi.fn()} walletAddress={WALLET} />)
      
      expect(screen.getByText('John Doe')).toBeInTheDocument()
      
      fireEvent.click(screen.getByTestId('delete-recipient-1'))
      
      expect(screen.queryByText('John Doe')).not.toBeInTheDocument()
      expect(screen.getByText('No saved recipients. Add one to get started!')).toBeInTheDocument()
    })
  })

  describe('Selecting recipient', () => {
    beforeEach(() => {
      const savedRecipients = [
        {
          id: 1,
          name: 'John Doe',
          address: 'GBDHJKD7NDHJKSD7HJKSJHD7JKSHD7JKSHDKJH7',
          country: 'Nigeria',
          memo: 'Monthly support'
        }
      ]
      localStorage.setItem('swiftremit_recipients', JSON.stringify(savedRecipients))
    })

    it('should call onSelectRecipient with recipient data', () => {
      const onSelect = vi.fn()
      render(<AddressBook onSelectRecipient={onSelect} walletAddress={WALLET} />)
      
      fireEvent.click(screen.getByTestId('select-recipient-1'))
      
      expect(onSelect).toHaveBeenCalledWith({
        name: 'John Doe',
        address: 'GBDHJKD7NDHJKSD7HJKSJHD7JKSHD7JKSHDKJH7',
        country: 'Nigeria',
        memo: 'Monthly support'
      })
    })
  })

  describe('Form cancellation', () => {
    it('should cancel form and clear input', () => {
      render(<AddressBook onSelectRecipient={vi.fn()} walletAddress={WALLET} />)
      
      fireEvent.click(screen.getByTestId('add-recipient-btn'))
      fireEvent.change(screen.getByTestId('recipient-name-input'), { target: { value: 'Test' } })
      fireEvent.click(screen.getByText('Cancel'))
      
      expect(screen.queryByTestId('recipient-name-input')).not.toBeInTheDocument()
      expect(screen.getByText('No saved recipients. Add one to get started!')).toBeInTheDocument()
    })
  })

  describe('Multiple recipients', () => {
    it('should display multiple recipients from localStorage', () => {
      const savedRecipients = [
        {
          id: 1,
          name: 'Person 1',
          address: 'GBDHJKD7NDHJKSD7HJKSJHD7JKSHD7JKSHDKJH7',
          country: 'Nigeria',
          memo: ''
        },
        {
          id: 2,
          name: 'Person 2',
          address: 'GBDHJKD7NDHJKSD7HJKSJHD7JKSHD7JKSHDKJH7',
          country: 'Ghana',
          memo: ''
        }
      ]
      localStorage.setItem('swiftremit_recipients', JSON.stringify(savedRecipients))
      
      render(<AddressBook onSelectRecipient={vi.fn()} walletAddress={WALLET} />)
      
      expect(screen.getByText('Person 1')).toBeInTheDocument()
      expect(screen.getByText('Person 2')).toBeInTheDocument()
    })
  })

  describe('Storage safety (#1721)', () => {
    const fillAndSave = () => {
      fireEvent.click(screen.getByTestId('add-recipient-btn'))
      fireEvent.change(screen.getByTestId('recipient-name-input'), { target: { value: 'Jane' } })
      fireEvent.change(screen.getByTestId('recipient-address-input'), { target: { value: VALID_ADDRESS } })
      fireEvent.change(screen.getByTestId('recipient-country-input'), { target: { value: 'Kenya' } })
      fireEvent.click(screen.getByTestId('save-recipient-btn'))
    }

    afterEach(() => {
      vi.restoreAllMocks()
    })

    it('scopes saved recipients to the connected wallet', () => {
      const { rerender } = render(<AddressBook onSelectRecipient={vi.fn()} walletAddress={WALLET} />)
      fillAndSave()
      expect(screen.getByText('Jane')).toBeInTheDocument()
      expect(JSON.parse(localStorage.getItem(recipientsKey(WALLET)))[0].name).toBe('Jane')
      expect(localStorage.getItem(LEGACY_RECIPIENTS_KEY)).toBeNull()

      rerender(<AddressBook onSelectRecipient={vi.fn()} walletAddress={OTHER_WALLET} />)
      expect(screen.queryByText('Jane')).not.toBeInTheDocument()
    })

    it('does not persist anything without a wallet', () => {
      render(<AddressBook onSelectRecipient={vi.fn()} />)
      expect(screen.getByTestId('address-book-no-wallet')).toBeInTheDocument()
      fillAndSave()
      expect(screen.getByText('Jane')).toBeInTheDocument()
      expect(localStorage.length).toBe(0)
    })

    it('moves legacy unscoped data to the wallet and deletes the plain copy', () => {
      localStorage.setItem(LEGACY_RECIPIENTS_KEY, JSON.stringify([{ id: 1, name: 'Old', address: VALID_ADDRESS, country: 'Ghana' }]))
      render(<AddressBook onSelectRecipient={vi.fn()} walletAddress={WALLET} />)
      expect(screen.getByText('Old')).toBeInTheDocument()
      expect(localStorage.getItem(LEGACY_RECIPIENTS_KEY)).toBeNull()
      expect(localStorage.getItem(recipientsKey(WALLET))).not.toBeNull()
    })

    it('does not crash when reading storage throws', () => {
      vi.spyOn(Storage.prototype, 'getItem').mockImplementation(() => {
        throw new DOMException('denied', 'SecurityError')
      })
      render(<AddressBook onSelectRecipient={vi.fn()} walletAddress={WALLET} />)
      expect(screen.getByText(/browser storage is blocked/)).toBeInTheDocument()
    })

    it('keeps working when writing storage throws', () => {
      vi.spyOn(Storage.prototype, 'setItem').mockImplementation(() => {
        throw new DOMException('quota', 'QuotaExceededError')
      })
      render(<AddressBook onSelectRecipient={vi.fn()} walletAddress={WALLET} />)
      fillAndSave()
      expect(screen.getByText('Jane')).toBeInTheDocument()
      expect(screen.getByText(/Failed to save recipients/)).toBeInTheDocument()
    })

    it('ignores corrupt or malformed stored data', () => {
      localStorage.setItem(recipientsKey(WALLET), JSON.stringify([{ id: 1, name: 'Good', address: VALID_ADDRESS, country: 'X' }, { bogus: true }]))
      render(<AddressBook onSelectRecipient={vi.fn()} walletAddress={WALLET} />)
      expect(screen.getByText('Good')).toBeInTheDocument()
    })
  })
})
