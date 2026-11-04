# Troubleshooting Guide

## Common Issues and Solutions

### Issue: "Cannot find native binding" error with rolldown

**Error message:**
```
Error: Cannot find native binding. npm has a bug related to optional dependencies
```

**Solution:**
This was caused by using `rolldown-vite` package. The issue has been fixed by switching to standard Vite.

If you still encounter this:
```bash
rm -rf node_modules package-lock.json
npm install
```

### Issue: Port 5173 already in use

**Error message:**
```
Port 5173 is already in use
```

**Solution:**
Either kill the existing process or use a different port:
```bash
npm run dev -- --port 5174
```

### Issue: "Failed to connect to API"

**Error in browser console:**
```
Failed to fetch
```

**Solution:**
1. Make sure the backend API is running:
   ```bash
   uvicorn src.api.main:app --reload
   ```

2. Check that API is accessible at http://localhost:8000

3. Verify `.env` file has correct API URL:
   ```bash
   VITE_API_URL=http://localhost:8000
   ```

### Issue: CORS errors

**Error in browser console:**
```
Access to fetch at 'http://localhost:8000' from origin 'http://localhost:5173' has been blocked by CORS policy
```

**Solution:**
The backend already has CORS enabled. If you still see this error:
1. Restart the backend API
2. Clear browser cache
3. Check that the API is running on the correct port

### Issue: "Module not found" errors

**Error message:**
```
Cannot find module 'react' or its corresponding type declarations
```

**Solution:**
Install dependencies:
```bash
npm install
```

### Issue: Blank page in browser

**Symptoms:**
- Page loads but shows nothing
- No errors in console

**Solution:**
1. Check browser console for JavaScript errors
2. Hard refresh the page (Ctrl+Shift+R or Cmd+Shift+R)
3. Clear browser cache
4. Check that all components are properly imported

### Issue: npm install fails

**Error message:**
```
npm ERR! code EACCES
```

**Solution:**
Permission issue. Try:
```bash
sudo chown -R $USER:$USER ~/.npm
npm install
```

### Issue: Slow npm install

**Symptoms:**
- npm install takes very long
- Hangs during installation

**Solution:**
1. Clear npm cache:
   ```bash
   npm cache clean --force
   ```

2. Use a different registry:
   ```bash
   npm install --registry=https://registry.npmjs.org/
   ```

### Issue: Hot reload not working

**Symptoms:**
- Changes to code don't reflect in browser
- Need to manually refresh

**Solution:**
1. Restart the dev server
2. Check that files are being saved
3. Try hard refresh in browser

### Issue: Build fails

**Error during `npm run build`:**

**Solution:**
1. Check for TypeScript errors (if using TypeScript)
2. Ensure all imports are correct
3. Check for syntax errors in JSX
4. Clear node_modules and reinstall:
   ```bash
   rm -rf node_modules package-lock.json
   npm install
   npm run build
   ```

## Getting Help

If you encounter an issue not listed here:

1. Check the browser console for errors
2. Check the terminal where `npm run dev` is running
3. Check the backend API logs
4. Verify all environment variables are set correctly
5. Try restarting both frontend and backend

## Useful Commands

```bash
# Clean install
rm -rf node_modules package-lock.json
npm install

# Check for outdated packages
npm outdated

# Update packages
npm update

# Check for security vulnerabilities
npm audit

# Fix security vulnerabilities
npm audit fix

# Clear Vite cache
rm -rf node_modules/.vite

# Check Node.js version
node --version  # Should be 18+

# Check npm version
npm --version
```

## System Requirements

- Node.js 18 or higher
- npm 9 or higher
- Modern browser (Chrome, Firefox, Safari, Edge)
- Backend API running on http://localhost:8000

## Still Having Issues?

1. Make sure you're in the `frontend-react` directory
2. Ensure the backend API is running
3. Check that all dependencies are installed
4. Try the clean install steps above
5. Check the main README.md for additional information
