# Skeleton Loaders

AnchorKit provides skeleton loader states for UI components to display loading, error, and success states for asynchronous operations.

## Overview

Skeleton loaders help create better user experiences by providing visual feedback during data fetching and validation operations. They support three main use cases:

1. **Anchor Information Loading** - Display loading state while fetching anchor metadata
2. **Transaction Status Tracking** - Show progress for transaction processing
3. **Authentication Validation** - Multi-step validation feedback for attestor authentication

## Features

- Loading state management
- Error handling with messages
- Progress tracking for transactions
- Multi-step validation tracking
- Type-safe state transitions

## Data Structures

### AnchorInfoSkeleton

Represents the loading state for anchor information.

```rust
pub struct AnchorInfoSkeleton {
    pub anchor: Address,
    pub is_loading: bool,
    pub has_error: bool,
    pub error_message: Option<String>,
}
```

**States:**
- `loading(anchor)` - Initial loading state
- `loaded(anchor)` - Successfully loaded
- `error(anchor, message)` - Error occurred

### TransactionStatusSkeleton

Represents the loading state for transaction status with progress tracking.

```rust
pub struct TransactionStatusSkeleton {
    pub transaction_id: u64,
    pub is_loading: bool,
    pub has_error: bool,
    pub error_message: Option<String>,
    pub progress_percentage: u32, // 0-10000 (100.00%)
}
```

**States:**
- `loading(transaction_id)` - Initial loading state (0% progress)
- `loading_with_progress(transaction_id, progress)` - Loading with progress indicator
- `loaded(transaction_id)` - Successfully completed (100% progress)
- `error(transaction_id, message)` - Error occurred

### AuthValidationSkeleton

Represents the loading state for authentication validation with step tracking.

```rust
pub struct AuthValidationSkeleton {
    pub attestor: Address,
    pub is_validating: bool,
    pub is_valid: bool,
    pub has_error: bool,
    pub error_message: Option<String>,
    pub validation_steps: Vec<ValidationStep>,
}

pub struct ValidationStep {
    pub step_name: String,
    pub is_complete: bool,
    pub is_loading: bool,
}
```

**States:**
- `validating(attestor)` - Initial validation state
- `validating_with_steps(attestor, steps)` - Validation with step tracking
- `validated(attestor)` - Successfully validated
- `error(attestor, message)` - Validation failed

## Contract Methods

### get_anchor_info_skeleton

Get the loading state for anchor information.

```rust
pub fn get_anchor_info_skeleton(
    env: Env,
    anchor: Address,
) -> Result<AnchorInfoSkeleton, Error>
```

**Returns:**
- `loading` - If anchor exists but metadata is not yet available
- `loaded` - If anchor metadata is available
- `error` - If anchor is not registered

**Example:**
```javascript
const skeleton = await contract.get_anchor_info_skeleton(anchorAddress);

if (skeleton.is_loading) {
    // Show loading spinner
} else if (skeleton.has_error) {
    // Show error message
    console.error(skeleton.error_message);
} else {
    // Data is loaded, fetch full anchor info
}
```

### get_transaction_status_skeleton

Get the loading state for transaction status with progress.

```rust
pub fn get_transaction_status_skeleton(
    env: Env,
    intent_id: u64,
) -> Result<TransactionStatusSkeleton, Error>
```

**Returns:**
- `loading_with_progress` - If transaction is in progress (calculates progress based on time)
- `error` - If transaction not found or expired

**Progress Calculation:**
Progress is calculated based on elapsed time vs. total TTL:
```
progress = (current_time - created_at) / (expires_at - created_at) * 10000
```

**Example:**
```javascript
const skeleton = await contract.get_transaction_status_skeleton(intentId);

if (skeleton.is_loading) {
    const progressPercent = skeleton.progress_percentage / 100;
    // Show progress bar: progressPercent%
} else if (skeleton.has_error) {
    // Show error (expired or not found)
    console.error(skeleton.error_message);
}
```

### get_auth_validation_skeleton

Get the loading state for authentication validation with step tracking.

```rust
pub fn get_auth_validation_skeleton(
    env: Env,
    attestor: Address,
) -> Result<AuthValidationSkeleton, Error>
```

**Returns:**
- `validating_with_steps` - If validation is in progress with step details
- `validated` - If all validation steps are complete
- `error` - If attestor is not registered

**Validation Steps:**
1. Registration verification
2. Credential policy check
3. Endpoint configuration check

**Example:**
```javascript
const skeleton = await contract.get_auth_validation_skeleton(attestorAddress);

if (skeleton.is_validating) {
    // Show validation steps
    skeleton.validation_steps.forEach(step => {
        if (step.is_complete) {
            console.log(`✓ ${step.step_name}`);
        } else if (step.is_loading) {
            console.log(`⏳ ${step.step_name}`);
        }
    });
} else if (skeleton.is_valid) {
    // Validation complete
    console.log('Authentication validated');
} else if (skeleton.has_error) {
    console.error(skeleton.error_message);
}
```

## Usage Patterns

### Pattern 1: Simple Loading State

```javascript
// Poll for anchor info
async function loadAnchorInfo(anchorAddress) {
    const skeleton = await contract.get_anchor_info_skeleton(anchorAddress);
    
    if (skeleton.is_loading) {
        // Show skeleton UI
        return { loading: true };
    }
    
    if (skeleton.has_error) {
        return { error: skeleton.error_message };
    }
    
    // Fetch full data
    const metadata = await contract.get_anchor_metadata(anchorAddress);
    return { data: metadata };
}
```

### Pattern 2: Progress Tracking

```javascript
// Monitor transaction progress
async function monitorTransaction(intentId) {
    const interval = setInterval(async () => {
        const skeleton = await contract.get_transaction_status_skeleton(intentId);
        
        if (skeleton.is_loading) {
            updateProgressBar(skeleton.progress_percentage / 100);
        } else {
            clearInterval(interval);
            
            if (skeleton.has_error) {
                showError(skeleton.error_message);
            } else {
                showSuccess();
            }
        }
    }, 1000);
}
```

### Pattern 3: Multi-Step Validation

```javascript
// Display validation steps
async function validateAuth(attestorAddress) {
    const skeleton = await contract.get_auth_validation_skeleton(attestorAddress);
    
    if (skeleton.has_error) {
        return { error: skeleton.error_message };
    }
    
    if (skeleton.is_validating) {
        return {
            validating: true,
            steps: skeleton.validation_steps.map(step => ({
                name: step.step_name,
                status: step.is_complete ? 'complete' : 'loading'
            }))
        };
    }
    
    return { validated: true };
}
```

## UI Integration

### React Example

```jsx
function AnchorInfo({ anchorAddress }) {
    const [skeleton, setSkeleton] = useState(null);
    const [data, setData] = useState(null);
    
    useEffect(() => {
        async function load() {
            const skel = await contract.get_anchor_info_skeleton(anchorAddress);
            setSkeleton(skel);
            
            if (!skel.is_loading && !skel.has_error) {
                const metadata = await contract.get_anchor_metadata(anchorAddress);
                setData(metadata);
            }
        }
        load();
    }, [anchorAddress]);
    
    if (skeleton?.is_loading) {
        return <SkeletonLoader />;
    }
    
    if (skeleton?.has_error) {
        return <ErrorMessage message={skeleton.error_message} />;
    }
    
    return <AnchorDetails data={data} />;
}
```

### Progress Bar Example

```jsx
function TransactionProgress({ intentId }) {
    const [skeleton, setSkeleton] = useState(null);
    
    useEffect(() => {
        const interval = setInterval(async () => {
            const skel = await contract.get_transaction_status_skeleton(intentId);
            setSkeleton(skel);
            
            if (!skel.is_loading) {
                clearInterval(interval);
            }
        }, 1000);
        
        return () => clearInterval(interval);
    }, [intentId]);
    
    if (!skeleton) return null;
    
    if (skeleton.has_error) {
        return <ErrorMessage message={skeleton.error_message} />;
    }
    
    const progress = skeleton.progress_percentage / 100;
    
    return (
        <div>
            <ProgressBar value={progress} max={100} />
            <span>{progress.toFixed(1)}%</span>
        </div>
    );
}
```

### Validation Steps Example

```jsx
function AuthValidation({ attestorAddress }) {
    const [skeleton, setSkeleton] = useState(null);
    
    useEffect(() => {
        async function validate() {
            const skel = await contract.get_auth_validation_skeleton(attestorAddress);
            setSkeleton(skel);
        }
        validate();
    }, [attestorAddress]);
    
    if (!skeleton) return null;
    
    if (skeleton.has_error) {
        return <ErrorMessage message={skeleton.error_message} />;
    }
    
    return (
        <div>
            <h3>Validation Steps</h3>
            {skeleton.validation_steps.map((step, i) => (
                <div key={i}>
                    {step.is_complete ? '✓' : '⏳'} {step.step_name}
                </div>
            ))}
            {skeleton.is_valid && <div>✓ Validation Complete</div>}
        </div>
    );
}
```

## Best Practices

1. **Poll Responsibly**: Use appropriate intervals (1-2 seconds) to avoid overwhelming the network
2. **Handle Errors**: Always check for error states and display meaningful messages
3. **Show Progress**: Use progress indicators for long-running operations
4. **Cache Results**: Cache loaded data to avoid unnecessary re-fetching
5. **Cleanup**: Clear intervals and timeouts when components unmount

## Error Handling

All skeleton loader methods return `Result<Skeleton, Error>`. Common errors:

- `AnchorNotFound` - Anchor is not registered
- `TransactionNotFound` - Transaction intent doesn't exist
- `AttestorNotRegistered` - Attestor is not registered

Always handle errors gracefully in your UI:

```javascript
try {
    const skeleton = await contract.get_anchor_info_skeleton(anchor);
    // Handle skeleton state
} catch (error) {
    console.error('Failed to get skeleton state:', error);
    // Show fallback UI
}
```

## Testing

The skeleton loaders include comprehensive tests in `src/skeleton_loader_tests.rs`:

```bash
cargo test skeleton_loader
```

Tests cover:
- State transitions
- Error conditions
- Progress calculations
- Validation step tracking
- Contract integration

## Performance Considerations

- Skeleton loaders are lightweight and fast
- Progress calculation is O(1)
- No storage writes (read-only operations)
- Minimal gas consumption

## Future Enhancements

Potential future additions:
- Custom validation steps
- Configurable progress calculation
- Batch skeleton queries
- WebSocket support for real-time updates
