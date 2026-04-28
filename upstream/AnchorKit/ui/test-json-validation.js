/**
 * Standalone test for JSON validation logic
 * Run with: node test-json-validation.js
 */

// Simulate the validateJson function from ApiRequestPanel
function validateJson(value) {
  const errors = [];
  
  if (!value.trim()) {
    return { valid: true, error: null };
  }

  try {
    JSON.parse(value);
    return { valid: true, error: null };
  } catch (err) {
    return { valid: false, error: err.message };
  }
}

// Test cases
const testCases = [
  {
    name: 'Valid JSON object',
    input: '{"key": "value"}',
    expectedValid: true
  },
  {
    name: 'Valid JSON array',
    input: '[1, 2, 3]',
    expectedValid: true
  },
  {
    name: 'Valid nested JSON',
    input: '{"user": {"name": "John", "age": 30}}',
    expectedValid: true
  },
  {
    name: 'Empty string (should be valid)',
    input: '',
    expectedValid: true
  },
  {
    name: 'Whitespace only (should be valid)',
    input: '   \n  \t  ',
    expectedValid: true
  },
  {
    name: 'Invalid JSON - missing closing brace',
    input: '{"key": "value"',
    expectedValid: false
  },
  {
    name: 'Invalid JSON - trailing comma',
    input: '{"key": "value",}',
    expectedValid: false
  },
  {
    name: 'Invalid JSON - unquoted key',
    input: '{key: "value"}',
    expectedValid: false
  },
  {
    name: 'Invalid JSON - single quotes',
    input: "{'key': 'value'}",
    expectedValid: false
  },
  {
    name: 'Invalid JSON - missing value',
    input: '{"key": }',
    expectedValid: false
  },
  {
    name: 'Valid JSON with special characters',
    input: '{"email": "test@example.com", "url": "https://example.com"}',
    expectedValid: true
  },
  {
    name: 'Valid JSON with numbers',
    input: '{"count": 42, "price": 19.99, "negative": -5}',
    expectedValid: true
  },
  {
    name: 'Valid JSON with booleans and null',
    input: '{"active": true, "deleted": false, "data": null}',
    expectedValid: true
  }
];

// Run tests
console.log('🧪 Testing JSON Validation Logic\n');
console.log('='.repeat(60));

let passed = 0;
let failed = 0;

testCases.forEach((test, index) => {
  const result = validateJson(test.input);
  const success = result.valid === test.expectedValid;
  
  if (success) {
    passed++;
    console.log(`✅ Test ${index + 1}: ${test.name}`);
  } else {
    failed++;
    console.log(`❌ Test ${index + 1}: ${test.name}`);
    console.log(`   Expected valid: ${test.expectedValid}, Got: ${result.valid}`);
    if (result.error) {
      console.log(`   Error: ${result.error}`);
    }
  }
});

console.log('='.repeat(60));
console.log(`\n📊 Results: ${passed} passed, ${failed} failed out of ${testCases.length} tests`);

if (failed === 0) {
  console.log('✨ All tests passed!');
  process.exit(0);
} else {
  console.log('⚠️  Some tests failed');
  process.exit(1);
}
