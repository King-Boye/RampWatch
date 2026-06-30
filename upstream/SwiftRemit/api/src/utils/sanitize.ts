import xss from 'xss';

/**
 * Sanitize a user-supplied string to prevent XSS attacks before storage or reflection.
 * Returns an empty string for non-string or empty inputs.
 */
export function sanitizeInput(input: unknown): string {
  if (!input || typeof input !== 'string') return '';
  return xss(input.trim());
}

/**
 * Sanitize all string values in a plain object one level deep.
 * Non-string values are left unchanged. Returns a new object.
 */
export function sanitizeObject<T extends Record<string, unknown>>(obj: T): T {
  const result = { ...obj };
  for (const key of Object.keys(result)) {
    if (typeof result[key] === 'string') {
      (result as Record<string, unknown>)[key] = sanitizeInput(result[key] as string);
    }
  }
  return result;
}
