import React, { useRef, useEffect, useState } from 'react';
import './ProofOfPayout.css';

interface ProofOfPayoutProps {
  transferId: number;
  onRelease: (transferId: number, proofImage: string) => Promise<void>;
}

export const ProofOfPayout: React.FC<ProofOfPayoutProps> = ({ transferId, onRelease }) => {
  const videoRef = useRef<HTMLVideoElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [stream, setStream] = useState<MediaStream | null>(null);
  const [capturedImage, setCapturedImage] = useState<string | null>(null);
  const [isReleasing, setIsReleasing] = useState(false);

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

    startCamera();

    return () => {
      if (stream) {
        stream.getTracks().forEach(track => track.stop());
      }
    };
  }, []);

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
    if (capturedImage) {
      setIsReleasing(true);
      try {
        await onRelease(transferId, capturedImage);
        // Handle success, maybe show confirmation
      } catch (error) {
        console.error('Error releasing funds:', error);
      } finally {
        setIsReleasing(false);
      }
    }
  };

  const retake = () => {
    setCapturedImage(null);
  };

  return (
    <div className="proof-of-payout">
      <h2>Proof of Payout</h2>
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
    </div>
  );
};