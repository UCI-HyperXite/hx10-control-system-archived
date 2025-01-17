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
            <h3 style={{ textAlign: "center", height: "1%", fontSize: "110%", marginTop: "1%"}}>{title}</h3>
            
            <div className="sensor-values-wrapper">
                {value.map((val, valindex) => (
                    <p key={valindex} className="sensor-value">
                        <span className="value-number">
                            <h3 style={{ fontSize: "100%", display: "inline" }}>{sensor_location[valindex]}: </h3> 
                            <span style={{ display: "inline" }}>{val} {unit}</span>
                        </span>
                    </p>
                ))}
            </div>
        </div>
    );
}

export default SensorBox;
