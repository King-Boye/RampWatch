"""Encryption utilities for PII data."""
import hashlib
import base64
from typing import Optional


class PIIEncryption:
    """Simple encryption for PII fields.
    
    Note: In production, use proper encryption libraries like cryptography.fernet
    This is a simplified implementation for demonstration.
    """
    
    def __init__(self, key: str = "fraud-detection-key"):
        """Initialize with encryption key."""
        self.key = key.encode()
    
    def encrypt(self, value: str) -> str:
        """Encrypt a string value.
        
        Args:
            value: Plain text value to encrypt
            
        Returns:
            Base64 encoded encrypted value
        """
        if not value:
            return value
            
        # Simple XOR encryption with key (for demo purposes)
        # In production, use proper encryption like AES
        encrypted = bytearray()
        key_bytes = self.key
        
        for i, char in enumerate(value.encode()):
            key_byte = key_bytes[i % len(key_bytes)]
            encrypted.append(char ^ key_byte)
        
        return base64.b64encode(bytes(encrypted)).decode()
    
    def decrypt(self, encrypted_value: str) -> str:
        """Decrypt an encrypted value.
        
        Args:
            encrypted_value: Base64 encoded encrypted value
            
        Returns:
            Decrypted plain text value
        """
        if not encrypted_value:
            return encrypted_value
            
        try:
            encrypted_bytes = base64.b64decode(encrypted_value.encode())
            decrypted = bytearray()
            key_bytes = self.key
            
            for i, byte in enumerate(encrypted_bytes):
                key_byte = key_bytes[i % len(key_bytes)]
                decrypted.append(byte ^ key_byte)
            
            return bytes(decrypted).decode()
        except Exception:
            return encrypted_value
    
    def hash_value(self, value: str) -> str:
        """Create a one-way hash of a value (for indexing).
        
        Args:
            value: Value to hash
            
        Returns:
            SHA256 hash of the value
        """
        if not value:
            return value
        return hashlib.sha256(value.encode()).hexdigest()


# Global encryption instance
pii_encryption = PIIEncryption()
