# AnchorKit UI Components

Reusable React components for building AnchorKit applications.

## Components

### API Request Panel

A comprehensive component for displaying API requests and responses with copy-to-clipboard functionality.

**Features:**
- Display endpoint with HTTP method badges
- Show formatted request body (JSON)
- Display response with loading/error states
- Generate and copy cURL commands
- Skeleton loaders for async operations
- Dark mode support
- Fully responsive
- Accessible (WCAG compliant)

**Quick Start:**
```tsx
import { ApiRequestPanel } from './components/ApiRequestPanel';

<ApiRequestPanel
  endpoint="https://api.anchorkit.stellar.org/v1/attestations"
  method="POST"
  requestBody={{ issuer: 'GANCHOR123...', subject: 'GUSER456...' }}
  response={{ success: true, attestation_id: 'att_123456' }}
/>
```

**Documentation:**
- [Component README](./components/README.md) - Full documentation
- [Quick Start Guide](./QUICK_START.md) - Get started in 5 minutes
- [Component Structure](./COMPONENT_STRUCTURE.md) - Architecture details
- [Examples](./components/ApiRequestPanel.example.tsx) - Usage examples

## Installation

### Option 1: Copy Files

```bash
# Copy component to your project
cp ui/components/ApiRequestPanel.tsx src/components/
cp ui/components/ApiRequestPanel.css src/components/
```

### Option 2: Install Package (Coming Soon)

```bash
npm install @anchorkit/ui-components
```

## Development

### Setup

```bash
cd ui
npm install
```

### Run Tests

```bash
npm test                # Run tests once
npm run test:watch      # Watch mode
npm run test:coverage   # With coverage
```

### Type Checking

```bash
npm run type-check
```

### Linting

```bash
npm run lint
```

## Project Structure

```
ui/
├── components/
│   ├── ApiRequestPanel.tsx          # Main component
│   ├── ApiRequestPanel.css          # Styles
│   ├── ApiRequestPanel.test.tsx     # Tests
│   ├── ApiRequestPanel.example.tsx  # Examples
│   ├── index.ts                     # Exports
│   └── README.md                    # Component docs
├── .github/
│   └── ISSUE_TEMPLATE/
│       └── api-request-panel-enhancement.md
├── package.json                     # Dependencies
├── tsconfig.json                    # TypeScript config
├── jest.config.js                   # Test config
├── jest.setup.js                    # Test setup
├── QUICK_START.md                   # Quick start guide
├── COMPONENT_STRUCTURE.md           # Architecture
└── README.md                        # This file
```

## Design System

All components follow the AnchorKit design system:

- **8pt Grid System**: All spacing uses 8pt increments
- **Color Palette**: Web3 aesthetic with technical reliability
- **Typography**: System fonts for performance
- **Accessibility**: WCAG 2.1 AA compliant
- **Responsive**: Mobile-first approach
- **Dark Mode**: Automatic system preference detection

## Browser Support

- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
- Mobile browsers (iOS Safari, Chrome Mobile)

## Contributing

We welcome contributions! Please:

1. Follow the AnchorKit design system
2. Maintain accessibility standards
3. Add tests for new features
4. Update documentation
5. Ensure responsive design

### Adding a New Component

1. Create component file: `components/YourComponent.tsx`
2. Create styles: `components/YourComponent.css`
3. Create tests: `components/YourComponent.test.tsx`
4. Create examples: `components/YourComponent.example.tsx`
5. Add documentation: Update `components/README.md`
6. Export from `components/index.ts`

## Testing

All components include comprehensive test suites:

- Unit tests for all functionality
- Accessibility tests
- Responsive design tests
- Edge case handling
- Browser compatibility tests

Run tests:
```bash
npm test
```

## Documentation

Each component includes:

- **README.md**: Complete API documentation
- **Examples**: Real-world usage patterns
- **Tests**: Test examples and patterns
- **TypeScript**: Full type definitions

## Roadmap

### Current Components
- ✅ API Request Panel

### Planned Components
- [ ] Request Builder Form
- [ ] Response Formatter
- [ ] Authentication Manager
- [ ] Rate Limit Display
- [ ] WebSocket Monitor
- [ ] Request History
- [ ] API Documentation Viewer
- [ ] Endpoint Explorer

## Integration with AnchorKit

These components are designed to work seamlessly with:

- **Soroban Contracts**: Display contract call results
- **Skeleton Loaders**: Show loading states
- **Session Tracking**: Display session information
- **Health Monitoring**: Show anchor health status
- **Metadata Caching**: Display cached data

## Performance

All components are optimized for:

- Minimal re-renders
- Efficient DOM updates
- GPU-accelerated animations
- Lazy loading support
- Code splitting ready

## Accessibility

All components meet WCAG 2.1 AA standards:

- Semantic HTML
- ARIA labels
- Keyboard navigation
- Screen reader support
- High contrast mode
- Color-blind friendly

## License

Part of the AnchorKit project - MIT License

## Support

- 📖 [Component Documentation](./components/README.md)
- 🚀 [Quick Start Guide](./QUICK_START.md)
- 🏗️ [Component Structure](./COMPONENT_STRUCTURE.md)
- 💡 [Examples](./components/ApiRequestPanel.example.tsx)
- 🐛 [Report Issues](https://github.com/Haroldwonder/AnchorKit/issues)

## Related Documentation

- [AnchorKit Main README](../README.md)
- [API Specification](../API_SPEC.md)
- [Design System](../Design%20System%20UI.txt)
- [Skeleton Loaders](../SKELETON_LOADERS.md)

---

**Status**: Active Development  
**Version**: 0.1.0  
**Components**: 1  
**Tests**: 30+  
**Coverage**: 80%+
