import AsyncStorage from '@react-native-async-storage/async-storage';
import * as SecureStore from 'expo-secure-store';
import { readThroughCache, clearOfflineCacheForUser } from '../../services/offlineCache';

const storage = AsyncStorage as jest.Mocked<typeof AsyncStorage>;
const getWallet = SecureStore.getItemAsync as jest.Mock;
const cacheKey = 'history:wallet:p0';
const testData = { remittances: [{ id: 1 }], nextCursor: null };
const keyFor = (wallet: string) => `cache:v2:${encodeURIComponent(wallet)}:${encodeURIComponent(cacheKey)}`;

beforeEach(() => {
  jest.clearAllMocks();
  getWallet.mockResolvedValue('wallet-A');
  storage.setItem.mockResolvedValue(undefined);
  storage.getItem.mockResolvedValue(null);
  storage.getAllKeys.mockResolvedValue([]);
  storage.removeItem.mockResolvedValue(undefined);
  storage.multiRemove.mockResolvedValue(undefined);
});

describe('offline read-through cache, scoped by wallet', () => {
  it('stores a fresh response under the wallet-scoped key', async () => {
    const result = await readThroughCache(cacheKey, async () => testData);
    expect(result.fromCache).toBe(false);
    expect(storage.setItem).toHaveBeenCalledWith(keyFor('wallet-A'), expect.any(String));
  });

  it('returns a valid cached response when the network fails', async () => {
    storage.getItem.mockResolvedValue(JSON.stringify({ data: testData, cachedAt: Date.now() }));
    const result = await readThroughCache(cacheKey, async () => { throw new Error('offline'); });
    expect(result.data).toEqual(testData);
    expect(result.fromCache).toBe(true);
    expect(result.stale).toBe(false);
    expect(storage.getItem).toHaveBeenCalledWith(keyFor('wallet-A'));
  });

  it('marks old cached results stale', async () => {
    storage.getItem.mockResolvedValue(JSON.stringify({ data: testData, cachedAt: Date.now() - 11 * 60_000 }));
    const result = await readThroughCache(cacheKey, async () => { throw new Error('offline'); });
    expect(result.stale).toBe(true);
  });

  it('does not read the cache if no wallet is signed in', async () => {
    getWallet.mockResolvedValue(null);
    await expect(readThroughCache(cacheKey, async () => testData)).rejects.toThrow('signed-in wallet');
    expect(storage.getItem).not.toHaveBeenCalled();
    expect(storage.setItem).not.toHaveBeenCalled();
  });

  it('never reads a different wallet\'s cache', async () => {
    getWallet.mockResolvedValue('wallet-B');
    const failure = new Error('offline');
    await expect(readThroughCache(cacheKey, async () => { throw failure; })).rejects.toBe(failure);
    expect(storage.getItem).toHaveBeenCalledWith(keyFor('wallet-B'));
    expect(storage.getItem).not.toHaveBeenCalledWith(keyFor('wallet-A'));
  });

  it('discards corrupt or structurally invalid entries and rethrows the fetch error', async () => {
    for (const invalid of ['{bad json', '{"cachedAt":"bad","data":1}', 'null']) {
      storage.getItem.mockResolvedValue(invalid);
      const failure = new Error('offline');
      await expect(readThroughCache(cacheKey, async () => { throw failure; })).rejects.toBe(failure);
      expect(storage.removeItem).toHaveBeenCalledWith(keyFor('wallet-A'));
    }
  });

  it('does not write results if the wallet changes while fetching', async () => {
    getWallet.mockResolvedValueOnce('wallet-A').mockResolvedValue('wallet-B');
    await expect(readThroughCache(cacheKey, async () => testData)).rejects.toThrow('Wallet changed');
    expect(storage.setItem).not.toHaveBeenCalled();
    expect(storage.getItem).not.toHaveBeenCalled();
  });

  it('clears only the logged-out wallet and unsafe legacy cache keys', async () => {
    storage.getAllKeys.mockResolvedValue([
      keyFor('wallet-A'), keyFor('wallet-B'), 'cache:history:wallet:p0', 'other:data',
    ]);
    await clearOfflineCacheForUser('wallet-A');
    expect(storage.multiRemove).toHaveBeenCalledWith([keyFor('wallet-A'), 'cache:history:wallet:p0']);
  });
});
