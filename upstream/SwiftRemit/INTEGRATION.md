# SwiftRemit Integration Guide

Guide for integrating SwiftRemit smart contract into your application.

## Architecture Overview

```
┌─────────────┐         ┌──────────────┐         ┌─────────────┐
│   Sender    │────────▶│  SwiftRemit  │────────▶│    Agent    │
│  Frontend   │         │   Contract   │         │  Dashboard  │
└─────────────┘         └──────────────┘         └─────────────┘
      │                        │                         │
      │                        │                         │
      ▼                        ▼                         ▼
┌─────────────┐         ┌──────────────┐         ┌─────────────┐
│    USDC     │         │   Platform   │         │    Fiat     │
│   Wallet    │         │    Admin     │         │   Payout    │
└─────────────┘         └──────────────┘         └─────────────┘
```

## Integration Steps

### 1. Frontend Integration (Sender Side)

#### Install Dependencies

```bash
npm install @stellar/stellar-sdk soroban-client
```

#### Connect Wallet

```typescript
import { SorobanRpc, TransactionBuilder, Networks } from '@stellar/stellar-sdk';

const server = new SorobanRpc.Server('https://soroban-testnet.stellar.org');

// Connect to Freighter wallet
async function connectWallet() {
  if (window.freighter) {
    const publicKey = await window.freighter.getPublicKey();
    return publicKey;
  }
  throw new Error('Freighter wallet not installed');
}
```

#### Create Remittance

```typescript
import { Contract, Address } from '@stellar/stellar-sdk';

async function createRemittance(
  senderPublicKey: string,
  agentAddress: string,
  amount: bigint,
  contractId: string
) {
  const contract = new Contract(contractId);
  
  const tx = new TransactionBuilder(account, {
    fee: '1000',
    networkPassphrase: Networks.TESTNET,
  })
    .addOperation(
      contract.call(
        'create_remittance',
        Address.fromString(senderPublicKey),
        Address.fromString(agentAddress),
        amount
      )
    )
    .setTimeout(30)
    .build();

  // Sign with Freighter
  const signedTx = await window.freighter.signTransaction(tx.toXDR(), {
    network: 'TESTNET',
    networkPassphrase: Networks.TESTNET,
  });

  // Submit transaction
  const result = await server.sendTransaction(signedTx);
  
  return result;
}
```

#### Monitor Remittance Status

```typescript
async function getRemittanceStatus(contractId: string, remittanceId: number) {
  const contract = new Contract(contractId);
  
  const result = await contract.call('get_remittance', remittanceId);
  
  return {
    id: result.id,
    sender: result.sender,
    agent: result.agent,
    amount: result.amount,
    fee: result.fee,
    status: result.status, // Pending, Completed, Cancelled
  };
}
```

#### Listen to Events

```typescript
async function subscribeToEvents(contractId: string) {
  const eventStream = server.getEvents({
    startLedger: 'latest',
    filters: [
      {
        type: 'contract',
        contractIds: [contractId],
      },
    ],
  });

  for await (const event of eventStream) {
    console.log('Event:', event);
    
    if (event.topic.includes('created')) {
      // Handle remittance created
      handleRemittanceCreated(event);
    } else if (event.topic.includes('completed')) {
      // Handle remittance completed
      handleRemittanceCompleted(event);
    } else if (event.topic.includes('cancelled')) {
      // Handle remittance cancelled
      handleRemittanceCancelled(event);
    }
  }
}
```

### 2. Agent Dashboard Integration

#### Check Agent Registration

```typescript
async function isAgentRegistered(contractId: string, agentAddress: string) {
  const contract = new Contract(contractId);
  const result = await contract.call('is_agent_registered', Address.fromString(agentAddress));
  return result;
}
```

#### Confirm Payout

```typescript
async function confirmPayout(
  agentPublicKey: string,
  contractId: string,
  remittanceId: number
) {
  const contract = new Contract(contractId);
  
  const tx = new TransactionBuilder(account, {
    fee: '1000',
    networkPassphrase: Networks.TESTNET,
  })
    .addOperation(
      contract.call('confirm_payout', remittanceId)
    )
    .setTimeout(30)
    .build();

  // Sign and submit
  const signedTx = await window.freighter.signTransaction(tx.toXDR(), {
    network: 'TESTNET',
    networkPassphrase: Networks.TESTNET,
  });

  const result = await server.sendTransaction(signedTx);
  return result;
}
```

### 3. Admin Dashboard Integration

#### Register Agent

```typescript
async function registerAgent(
  adminPublicKey: string,
  contractId: string,
  agentAddress: string
) {
  const contract = new Contract(contractId);
  
  const tx = new TransactionBuilder(account, {
    fee: '1000',
    networkPassphrase: Networks.TESTNET,
  })
    .addOperation(
      contract.call('register_agent', Address.fromString(agentAddress))
    )
    .setTimeout(30)
    .build();

  const signedTx = await window.freighter.signTransaction(tx.toXDR(), {
    network: 'TESTNET',
    networkPassphrase: Networks.TESTNET,
  });

  const result = await server.sendTransaction(signedTx);
  return result;
}
```

#### Update Platform Fee

```typescript
async function updateFee(
  adminPublicKey: string,
  contractId: string,
  feeBps: number
) {
  const contract = new Contract(contractId);
  
  const tx = new TransactionBuilder(account, {
    fee: '1000',
    networkPassphrase: Networks.TESTNET,
  })
    .addOperation(
      contract.call('update_fee', feeBps)
    )
    .setTimeout(30)
    .build();

  const signedTx = await window.freighter.signTransaction(tx.toXDR(), {
    network: 'TESTNET',
    networkPassphrase: Networks.TESTNET,
  });

  const result = await server.sendTransaction(signedTx);
  return result;
}
```

#### Withdraw Fees

```typescript
async function withdrawFees(
  adminPublicKey: string,
  contractId: string,
  recipientAddress: string
) {
  const contract = new Contract(contractId);
  
  const tx = new TransactionBuilder(account, {
    fee: '1000',
    networkPassphrase: Networks.TESTNET,
  })
    .addOperation(
      contract.call('withdraw_fees', Address.fromString(recipientAddress))
    )
    .setTimeout(30)
    .build();

  const signedTx = await window.freighter.signTransaction(tx.toXDR(), {
    network: 'TESTNET',
    networkPassphrase: Networks.TESTNET,
  });

  const result = await server.sendTransaction(signedTx);
  return result;
}
```

## Backend Integration

### Node.js Example

```typescript
import { Keypair, SorobanRpc, TransactionBuilder, Networks, Contract } from '@stellar/stellar-sdk';

class SwiftRemitService {
  private server: SorobanRpc.Server;
  private contractId: string;

  constructor(contractId: string, rpcUrl: string) {
    this.server = new SorobanRpc.Server(rpcUrl);
    this.contractId = contractId;
  }

  async getRemittance(remittanceId: number) {
    const contract = new Contract(this.contractId);
    const result = await contract.call('get_remittance', remittanceId);
    return result;
  }

  async getAccumulatedFees() {
    const contract = new Contract(this.contractId);
    const result = await contract.call('get_accumulated_fees');
    return result;
  }

  async getPlatformFeeBps() {
    const contract = new Contract(this.contractId);
    const result = await contract.call('get_platform_fee_bps');
    return result;
  }
}

// Usage
const service = new SwiftRemitService(
  'CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD2KM',
  'https://soroban-testnet.stellar.org'
);

const remittance = await service.getRemittance(1);
console.log('Remittance:', remittance);
```

### Python Example

```python
from stellar_sdk import SorobanServer, Keypair, TransactionBuilder, Network
from stellar_sdk.soroban_rpc import GetEventsRequest

class SwiftRemitService:
    def __init__(self, contract_id: str, rpc_url: str):
        self.server = SorobanServer(rpc_url)
        self.contract_id = contract_id
    
    def get_remittance(self, remittance_id: int):
        # Implementation using stellar_sdk
        pass
    
    def subscribe_to_events(self):
        request = GetEventsRequest(
            start_ledger="latest",
            filters=[{
                "type": "contract",
                "contractIds": [self.contract_id]
            }]
        )
        
        events = self.server.get_events(request)
        return events

# Usage
service = SwiftRemitService(
    "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD2KM",
    "https://soroban-testnet.stellar.org"
)
```

## Event Handling

### Event Types

1. **created** - Remittance created
   - Data: `(remittance_id, sender, agent, amount, fee)`

2. **completed** - Payout confirmed
   - Data: `(remittance_id, agent, payout_amount)`

3. **cancelled** - Remittance cancelled
   - Data: `(remittance_id, sender, refund_amount)`

4. **agent_reg** - Agent registered
   - Data: `agent`

5. **agent_rem** - Agent removed
   - Data: `agent`

6. **fee_upd** - Fee updated
   - Data: `fee_bps`

7. **fees_with** - Fees withdrawn
   - Data: `(to, amount)`

### Webhook Integration

```typescript
import express from 'express';

const app = express();

// Webhook endpoint for event notifications
app.post('/webhook/swiftremit', async (req, res) => {
  const event = req.body;
  
  switch (event.type) {
    case 'created':
      await handleRemittanceCreated(event.data);
      break;
    case 'completed':
      await handleRemittanceCompleted(event.data);
      break;
    case 'cancelled':
      await handleRemittanceCancelled(event.data);
      break;
  }
  
  res.status(200).send('OK');
});

async function handleRemittanceCreated(data: any) {
  // Send notification to sender
  // Update database
  // Notify agent
}

async function handleRemittanceCompleted(data: any) {
  // Send confirmation to sender
  // Update agent balance
  // Record transaction
}

async function handleRemittanceCancelled(data: any) {
  // Notify sender of refund
  // Update status in database
}
```

## Error Handling

```typescript
async function safeContractCall(fn: () => Promise<any>) {
  try {
    return await fn();
  } catch (error) {
    if (error.message.includes('Error(Contract, #1)')) {
      throw new Error('Contract already initialized');
    } else if (error.message.includes('Error(Contract, #3)')) {
      throw new Error('Invalid amount - must be greater than 0');
    } else if (error.message.includes('Error(Contract, #5)')) {
      throw new Error('Agent not registered');
    } else if (error.message.includes('Error(Contract, #7)')) {
      throw new Error('Invalid status for this operation');
    }
    throw error;
  }
}
```

## Testing Integration

```typescript
import { describe, it, expect } from 'vitest';

describe('SwiftRemit Integration', () => {
  it('should create remittance', async () => {
    const result = await createRemittance(
      senderKey,
      agentAddress,
      1000000000n,
      contractId
    );
    
    expect(result.status).toBe('success');
  });

  it('should confirm payout', async () => {
    const result = await confirmPayout(
      agentKey,
      contractId,
      1
    );
    
    expect(result.status).toBe('success');
  });
});
```

## Best Practices

1. **Always validate inputs** before calling contract functions
2. **Handle errors gracefully** with user-friendly messages
3. **Monitor events** for real-time updates
4. **Cache contract data** to reduce RPC calls
5. **Implement retry logic** for failed transactions
6. **Use proper authorization** for sensitive operations
7. **Test thoroughly** on testnet before mainnet
8. **Monitor gas costs** and optimize transactions
9. **Implement rate limiting** to prevent abuse
10. **Log all transactions** for audit trail

## Security Considerations

1. Never expose private keys in frontend code
2. Validate all user inputs
3. Implement proper authentication
4. Use HTTPS for all API calls
5. Monitor for suspicious activity
6. Implement transaction limits
7. Regular security audits
8. Keep dependencies updated

## Support

For integration support:
- Stellar Discord: https://discord.gg/stellar
- GitHub Issues: Report integration issues
- Documentation: https://soroban.stellar.org/docs
