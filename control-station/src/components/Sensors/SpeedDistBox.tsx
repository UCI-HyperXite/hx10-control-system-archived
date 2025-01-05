import "./SpeedDistBox.css";

interface SpeedDistProps {
    title: string;
    value: number;
    unit: string;
}

function SpeedDistBox({ title, value, unit }: SpeedDistProps) {
    return (
        <div className="speed-dist-box">
            <div className="speed-dist-content">
                <span className="speed-dist-title">{title}</span>
                <span className="speed-dist-value">{value}{unit}</span>
            </div>
            
        </div>
    );
}

export default SpeedDistBox;
