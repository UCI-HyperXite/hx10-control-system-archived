
import React from 'react';
import './RotationIndicator.css';

interface RotationIndicatorProps {
  pitch: number; // percentage value for pitch
  roll: number;  // percentage value for roll
  yaw: number;   // percentage value for yaw
}

const RotationIndicator: React.FC<RotationIndicatorProps> = ({ pitch, roll, yaw }) => {
  return (
    <div className="rotation-container">
        <div className="circle">
          <div className="circle-direction " style={{ transform: `rotate(${pitch * 3.6}deg)` }}>
            Pitch
          </div>
          <div className="percentage">{pitch}%</div>
        </div>
        <div className="circle">
          <div className="circle-direction " style={{ transform: `rotate(${roll * 3.6}deg)` }}>
            Roll
          </div>
          <div className="percentage">{roll}%</div>
        </div>
        <div className="circle">
          <div className="circle-direction " style={{ transform: `rotate(${yaw * 3.6}deg)` }}>
            Yaw
          </div>
          <div className="percentage">{yaw}%</div>
        </div>
      </div>
  );
};

export default RotationIndicator;
