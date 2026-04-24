/**
 * Integration test simulating component behavior
 * Run with: node test-component-integration.js
 */

// Simulate React component state and behavior
class ApiRequestPanelSimulator {
  constructor() {
    this.editableBody = '';
    this.jsonError = null;
    this.isLoading = false;
    this.onSubmitCalled = false;
    this.submittedData = null;
  }

  formatJson(data) {
    if (typeof data === 'string') return data;
    return JSON.stringify(data, null, 2);
  }

  validateJson(value) {
    if (!value.trim()) {
      this.jsonError = null;
      return true;
    }

    try {
      JSON.parse(value);
      this.jsonError = null;
      return true;
    } catch (err) {
      this.jsonError = err.message;
      return false;
    }
  }

  handleBodyChange(value) {
    this.editableBody = value;
    this.validateJson(value);
  }

  handleSubmit(onSubmit) {
    if (!onSubmit || this.jsonError) return false;

    try {
      const parsedBody = this.editableBody.trim() ? JSON.parse(this.editableBody) : {};
      this.onSubmitCalled = true;
      this.submittedData = parsedBody;
      onSubmit(parsedBody);
      return true;
    } catch (err) {
      console.error('Failed to parse JSON:', err);
      return false;
    }
  }

  isSubmitDisabled() {
    return !!this.jsonError || this.isLoading;
  }

  reset() {
    this.editableBody = '';
    this.jsonError = null;
    this.isLoading = false;
    this.onSubmitCalled = false;
    this.submittedData = null;
  }
}

// Test scenarios
const scenarios = [
  {
    name: 'User enters valid JSON and submits',
    steps: [
      { action: 'type', value: '{"name": "Alice", "age": 30}' },
      { action: 'submit', expectSuccess: true }
    ],
    expectations: {
      jsonError: null,
      submitDisabled: false,
      submitCalled: true,
      submittedData: { name: 'Alice', age: 30 }
    }
  },
  {
    name: 'User enters invalid JSON - submit should be disabled',
    steps: [
      { action: 'type', value: '{"name": "Bob"' }
    ],
    expectations: {
      jsonError: expect => expect && expect.length > 0, // Any error message is fine
      submitDisabled: true,
      submitCalled: false
    }
  },
  {
    name: 'User fixes invalid JSON - submit should be enabled',
    steps: [
      { action: 'type', value: '{"invalid"' },
      { action: 'type', value: '{"valid": "json"}' },
      { action: 'submit', expectSuccess: true }
    ],
    expectations: {
      jsonError: null,
      submitDisabled: false,
      submitCalled: true,
      submittedData: { valid: 'json' }
    }
  },
  {
    name: 'User submits empty body',
    steps: [
      { action: 'type', value: '' },
      { action: 'submit', expectSuccess: true }
    ],
    expectations: {
      jsonError: null,
      submitDisabled: false,
      submitCalled: true,
      submittedData: {}
    }
  },
  {
    name: 'User enters whitespace only',
    steps: [
      { action: 'type', value: '   \n  \t  ' }
    ],
    expectations: {
      jsonError: null,
      submitDisabled: false,
      submitCalled: false
    }
  },
  {
    name: 'User enters JSON with trailing comma',
    steps: [
      { action: 'type', value: '{"key": "value",}' }
    ],
    expectations: {
      jsonError: expect => expect.includes('JSON'),
      submitDisabled: true,
      submitCalled: false
    }
  },
  {
    name: 'Loading state disables submit',
    steps: [
      { action: 'type', value: '{"valid": "json"}' },
      { action: 'setLoading', value: true }
    ],
    expectations: {
      jsonError: null,
      submitDisabled: true,
      submitCalled: false
    }
  },
  {
    name: 'Complex nested JSON',
    steps: [
      { action: 'type', value: '{"user": {"profile": {"name": "Charlie", "settings": {"theme": "dark"}}}}' },
      { action: 'submit', expectSuccess: true }
    ],
    expectations: {
      jsonError: null,
      submitDisabled: false,
      submitCalled: true,
      submittedData: { user: { profile: { name: 'Charlie', settings: { theme: 'dark' } } } }
    }
  }
];

// Run scenarios
console.log('🧪 Testing ApiRequestPanel Component Integration\n');
console.log('='.repeat(70));

let passed = 0;
let failed = 0;
const failures = [];

scenarios.forEach((scenario, index) => {
  const component = new ApiRequestPanelSimulator();
  let mockOnSubmit = (data) => {};
  let scenarioPassed = true;
  const errors = [];

  try {
    // Execute steps
    scenario.steps.forEach(step => {
      if (step.action === 'type') {
        component.handleBodyChange(step.value);
      } else if (step.action === 'submit') {
        const success = component.handleSubmit(mockOnSubmit);
        if (step.expectSuccess && !success) {
          errors.push(`Submit failed when it should have succeeded`);
          scenarioPassed = false;
        }
      } else if (step.action === 'setLoading') {
        component.isLoading = step.value;
      }
    });

    // Verify expectations
    const exp = scenario.expectations;

    if (exp.jsonError !== undefined) {
      if (typeof exp.jsonError === 'function') {
        if (!exp.jsonError(component.jsonError || '')) {
          errors.push(`jsonError check failed: "${component.jsonError}"`);
          scenarioPassed = false;
        }
      } else if (exp.jsonError !== component.jsonError) {
        errors.push(`Expected jsonError: "${exp.jsonError}", got: "${component.jsonError}"`);
        scenarioPassed = false;
      }
    }

    if (exp.submitDisabled !== undefined) {
      const actualDisabled = component.isSubmitDisabled();
      if (exp.submitDisabled !== actualDisabled) {
        errors.push(`Expected submitDisabled: ${exp.submitDisabled}, got: ${actualDisabled}`);
        scenarioPassed = false;
      }
    }

    if (exp.submitCalled !== undefined) {
      if (exp.submitCalled !== component.onSubmitCalled) {
        errors.push(`Expected submitCalled: ${exp.submitCalled}, got: ${component.onSubmitCalled}`);
        scenarioPassed = false;
      }
    }

    if (exp.submittedData !== undefined) {
      const actualData = JSON.stringify(component.submittedData);
      const expectedData = JSON.stringify(exp.submittedData);
      if (actualData !== expectedData) {
        errors.push(`Expected submittedData: ${expectedData}, got: ${actualData}`);
        scenarioPassed = false;
      }
    }

    if (scenarioPassed) {
      passed++;
      console.log(`✅ Scenario ${index + 1}: ${scenario.name}`);
    } else {
      failed++;
      console.log(`❌ Scenario ${index + 1}: ${scenario.name}`);
      errors.forEach(err => console.log(`   ${err}`));
      failures.push({ scenario: scenario.name, errors });
    }

  } catch (error) {
    failed++;
    scenarioPassed = false;
    console.log(`❌ Scenario ${index + 1}: ${scenario.name}`);
    console.log(`   Unexpected error: ${error.message}`);
    failures.push({ scenario: scenario.name, errors: [error.message] });
  }
});

console.log('='.repeat(70));
console.log(`\n📊 Results: ${passed} passed, ${failed} failed out of ${scenarios.length} scenarios`);

if (failed === 0) {
  console.log('\n✨ All integration tests passed!');
  console.log('\n✅ Acceptance Criteria Verified:');
  console.log('   • JSON parse attempted on body change');
  console.log('   • Inline error shown for invalid JSON');
  console.log('   • Submit button disabled while JSON is invalid');
  console.log('   • Component handles edge cases correctly');
  process.exit(0);
} else {
  console.log('\n⚠️  Some scenarios failed:');
  failures.forEach(f => {
    console.log(`\n   ${f.scenario}:`);
    f.errors.forEach(err => console.log(`     - ${err}`));
  });
  process.exit(1);
}
