import "./SensorBox.css";

interface SensorBoxProps {
    title: string;
    sensor_location: string[]
    value: number[];
    unit: string;
}

function SensorBox({ title, sensor_location, value, unit }: SensorBoxProps) {
    return (
        <div className="sensorbox">
            <h3 style={{ textAlign: "center", height: "1%", fontSize: "90%"}}>{title}</h3>
            
            <div className="sensor-values-wrapper">
                {value.map((val, valindex) => (
                    <p key={valindex} className="sensor-value">
                        <span className="value-number">
                            {sensor_location[valindex]} {val} {unit} 
                        </span>
                    </p>
                ))}
            </div>
        </div>
    );
}

export default SensorBox;
