import crypto from 'crypto';
import { Keypair } from '@stellar/stellar-sdk';

export interface WebhookPayload {
  transaction_id: string;
  status: string;
  timestamp: string;
  [key: string]: any;
}

export class WebhookVerifier {
  private readonly replayWindow: number;
  private readonly processedNonces: Set<string>;

  constructor(replayWindowSeconds: number = 300) {
    this.replayWindow = replayWindowSeconds * 1000;
    this.processedNonces = new Set();
    this.cleanupOldNonces();
  }

  /**
   * Verify webhook signature using anchor's public key
   */
  verifySignature(
    payload: string,
    signature: string,
    anchorPublicKey: string
  ): boolean {
    try {
      const keypair = Keypair.fromPublicKey(anchorPublicKey);
      const payloadBuffer = Buffer.from(payload, 'utf8');
      const signatureBuffer = Buffer.from(signature, 'base64');
      
      return keypair.verify(payloadBuffer, signatureBuffer);
    } catch (error) {
      console.error('Signature verification failed:', error);
      return false;
    }
  }

  /**
   * Verify HMAC signature (alternative method)
   */
  verifyHMAC(
    payload: string,
    signature: string,
    secret: string
  ): boolean {
    try {
      const expectedSignature = crypto
        .createHmac('sha256', secret)
        .update(payload)
        .digest('hex');
      
      // Ensure both signatures are the same length before comparison
      if (signature.length !== expectedSignature.length) {
        return false;
      }
      
      return crypto.timingSafeEqual(
        Buffer.from(signature),
        Buffer.from(expectedSignature)
      );
    } catch (error) {
      console.error('HMAC verification failed:', error);
      return false;
    }
  }

  /**
   * Validate timestamp to prevent replay attacks
   */
  validateTimestamp(timestamp: string): boolean {
    const webhookTime = new Date(timestamp).getTime();
    const now = Date.now();
    
    if (isNaN(webhookTime)) {
      return false;
    }

    const timeDiff = Math.abs(now - webhookTime);
    return timeDiff <= this.replayWindow;
  }

  /**
   * Check and record nonce to prevent replay attacks
   */
  validateNonce(nonce: string): boolean {
    if (this.processedNonces.has(nonce)) {
      return false;
    }
    
    this.processedNonces.add(nonce);
    return true;
  }

  /**
   * Cleanup old nonces periodically
   */
  private cleanupOldNonces(): void {
    setInterval(() => {
      this.processedNonces.clear();
    }, this.replayWindow);
  }
}
