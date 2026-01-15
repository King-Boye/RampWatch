"""Redis caching service for user baselines and features."""
import json
from typing import Optional, Dict, Any
import redis
from config.settings import settings


class CacheService:
    """Redis cache service for storing user baselines and temporary data."""
    
    def __init__(self):
        """Initialize Redis connection."""
        try:
            self.redis_client = redis.Redis(
                host=settings.redis_host,
                port=settings.redis_port,
                db=settings.redis_db,
                decode_responses=True
            )
            # Test connection
            self.redis_client.ping()
            self.enabled = True
        except (redis.ConnectionError, redis.RedisError):
            # Fallback to in-memory cache if Redis unavailable
            self.redis_client = None
            self.enabled = False
            self._memory_cache: Dict[str, Any] = {}
    
    def get(self, key: str) -> Optional[str]:
        """Get value from cache.
        
        Args:
            key: Cache key
            
        Returns:
            Cached value or None if not found
        """
        if self.enabled and self.redis_client:
            try:
                return self.redis_client.get(key)
            except redis.RedisError:
                return None
        else:
            return self._memory_cache.get(key)
    
    def set(self, key: str, value: str, ttl: Optional[int] = None) -> bool:
        """Set value in cache.
        
        Args:
            key: Cache key
            value: Value to cache
            ttl: Time to live in seconds (default from settings)
            
        Returns:
            True if successful, False otherwise
        """
        if ttl is None:
            ttl = settings.redis_ttl
        
        if self.enabled and self.redis_client:
            try:
                self.redis_client.setex(key, ttl, value)
                return True
            except redis.RedisError:
                return False
        else:
            self._memory_cache[key] = value
            return True
    
    def get_json(self, key: str) -> Optional[Dict[str, Any]]:
        """Get JSON value from cache.
        
        Args:
            key: Cache key
            
        Returns:
            Parsed JSON dict or None if not found
        """
        value = self.get(key)
        if value:
            try:
                return json.loads(value)
            except json.JSONDecodeError:
                return None
        return None
    
    def set_json(self, key: str, value: Dict[str, Any], ttl: Optional[int] = None) -> bool:
        """Set JSON value in cache.
        
        Args:
            key: Cache key
            value: Dict to cache as JSON
            ttl: Time to live in seconds
            
        Returns:
            True if successful, False otherwise
        """
        try:
            json_str = json.dumps(value)
            return self.set(key, json_str, ttl)
        except (TypeError, ValueError):
            return False
    
    def delete(self, key: str) -> bool:
        """Delete key from cache.
        
        Args:
            key: Cache key
            
        Returns:
            True if successful, False otherwise
        """
        if self.enabled and self.redis_client:
            try:
                self.redis_client.delete(key)
                return True
            except redis.RedisError:
                return False
        else:
            if key in self._memory_cache:
                del self._memory_cache[key]
            return True
    
    def clear(self) -> bool:
        """Clear all cache entries.
        
        Returns:
            True if successful, False otherwise
        """
        if self.enabled and self.redis_client:
            try:
                self.redis_client.flushdb()
                return True
            except redis.RedisError:
                return False
        else:
            self._memory_cache.clear()
            return True


# Global cache instance
cache_service = CacheService()
