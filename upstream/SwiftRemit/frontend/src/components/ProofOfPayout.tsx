import React, { useRef, useEffect, useState } from 'react';
import './ProofOfPayout.css';
import { horizonService, SettlementCompletedEvent } from '../services/horizonService';

interface ProofOfPayoutProps {
  remittanceId: number;
  onRelease?: (remittanceId: number, proofImage: string) => Promise<void>;
}

export const ProofOfPayout: React.FC<ProofOfPayoutProps> = ({ remittanceId, onRelease }) => {
  const videoRef = useRef<HTMLVideoElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [stream, setStream] = useState<MediaStream | null>(null);
  const [capturedImage, setCapturedImage] = useState<string | null>(null);
  const [isReleasing, setIsReleasing] = useState(false);
  const [eventData, setEventData] = useState<SettlementCompletedEvent | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchEventData = async () => {
      setIsLoading(true);
      setError(null);
      
      try {
        const data = await horizonService.fetchCompletedEvent(remittanceId);
        
        if (data) {
          setEventData(data);
        } else {
          setError('No completed event found for this remittance ID');
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to fetch event data');
      } finally {
        setIsLoading(false);
      }
    };

    fetchEventData();
  }, [remittanceId]);

  useEffect(() => {
    const startCamera = async () => {
      try {
        const mediaStream = await navigator.mediaDevices.getUserMedia({
          video: { facingMode: 'environment' }, // Use back camera if available
        });
        setStream(mediaStream);
        if (videoRef.current) {
          videoRef.current.srcObject = mediaStream;
        }
      } catch (error) {
        console.error('Error accessing camera:', error);
      }
    };

    // Only start camera if onRelease callback is provided (camera mode)
    if (onRelease) {
      startCamera();
    }

    return () => {
      if (stream) {
        stream.getTracks().forEach(track => track.stop());
      }
    };
  }, [onRelease]);

  const captureImage = () => {
    if (videoRef.current && canvasRef.current) {
      const canvas = canvasRef.current;
      const video = videoRef.current;
      canvas.width = video.videoWidth;
      canvas.height = video.videoHeight;
      const ctx = canvas.getContext('2d');
      if (ctx) {
        ctx.drawImage(video, 0, 0);
        const imageDataUrl = canvas.toDataURL('image/png');
        setCapturedImage(imageDataUrl);
      }
    }
  };

  const handleRelease = async () => {
    if (capturedImage && onRelease) {
      setIsReleasing(true);
      try {
        await onRelease(remittanceId, capturedImage);
        // Handle success, maybe show confirmation
      } catch (error) {
        console.error('Error releasing funds:', error);
      } finally {
        setIsReleasing(false);
      }
    }
  };

  const formatAmount = (amount: string): string => {
    const num = parseFloat(amount);
    if (isNaN(num)) return amount;
    return (num / 10000000).toFixed(7); // Convert from stroops to XLM/USDC
  };

  const formatTimestamp = (timestamp: string): string => {
    return new Date(timestamp).toLocaleString();
  };

  const truncateAddress = (address: string): string => {
    if (address.length <= 12) return address;
    return `${address.slice(0, 6)}...${address.slice(-6)}`;
  };

  const retake = () => {
    setCapturedImage(null);
  };

  return (
    <div className="proof-of-payout">
      <h2>Proof of Payout</h2>
      
      {isLoading && (
        <div className="loading-state">
          <p>Loading payout details...</p>
        </div>
      )}

      {error && (
        <div className="error-state">
          <p className="error-message">{error}</p>
        </div>
      )}

      {!isLoading && !error && eventData && (
        <div className="payout-details">
          <div className="detail-section">
            <h3>Transaction Details</h3>
            <div className="detail-row">
              <span className="detail-label">Remittance ID:</span>
              <span className="detail-value">{eventData.remittanceId}</span>
            </div>
            <div className="detail-row">
              <span className="detail-label">Sender:</span>
              <span className="detail-value" title={eventData.sender}>
                {truncateAddress(eventData.sender)}
              </span>
            </div>
            <div className="detail-row">
              <span className="detail-label">Agent:</span>
              <span className="detail-value" title={eventData.agent}>
                {truncateAddress(eventData.agent)}
              </span>
            </div>
            <div className="detail-row">
              <span className="detail-label">Amount:</span>
              <span className="detail-value">{formatAmount(eventData.amount)} USDC</span>
            </div>
            <div className="detail-row">
              <span className="detail-label">Fee:</span>
              <span className="detail-value">{formatAmount(eventData.fee)} USDC</span>
            </div>
            <div className="detail-row">
              <span className="detail-label">Timestamp:</span>
              <span className="detail-value">{formatTimestamp(eventData.timestamp)}</span>
            </div>
            <div className="detail-row">
              <span className="detail-label">Transaction Hash:</span>
              <span className="detail-value" title={eventData.transactionHash}>
                {truncateAddress(eventData.transactionHash)}
              </span>
            </div>
          </div>

          <div className="action-section">
            <a
              href={horizonService.getStellarExpertLink(eventData.transactionHash)}
              target="_blank"
              rel="noopener noreferrer"
              className="stellar-expert-link"
            >
              View on Stellar Expert →
            </a>
          </div>
        </div>
      )}

      {onRelease && (
        <>
          <p>Capture an image as proof that the payout has been made to the recipient.</p>
          {!capturedImage ? (
            <div className="camera-container">
              <video ref={videoRef} autoPlay playsInline muted className="camera-video" />
              <div className="camera-overlay">
                <div className="overlay-frame"></div>
                <p className="overlay-text">Position the proof document within the frame</p>
              </div>
              <button onClick={captureImage} className="capture-button">Capture</button>
            </div>
          ) : (
            <div className="preview-container">
              <img src={capturedImage} alt="Captured proof" className="captured-image" />
              <div className="preview-actions">
                <button onClick={retake} className="retake-button">Retake</button>
                <button onClick={handleRelease} disabled={isReleasing} className="release-button">
                  {isReleasing ? 'Releasing...' : 'Release Funds'}
                </button>
              </div>
            </div>
          )}
          <canvas ref={canvasRef} style={{ display: 'none' }} />
        </>
      )}
    </div>
  );