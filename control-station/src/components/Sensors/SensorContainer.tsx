import { useContext, useState } from "react";
import SensorBox from "./SensorBox";
import SpeedDistBox from "./SpeedDistBox";
import RotationIndicator from "@/components/RotationIndicator/RotationIndicator";
function SensorContainer() {
  const [TempLocation, setTempLoc] = useState(["loc1", "loc2", "loc3"]);
  const [Temperature, setTemp] = useState([25, 50, 75]);
  const [PressureLocation, setPressLoc] = useState(["loc1", "loc2", "loc3"]);
  const [Pressure, setPressure] = useState([25, 50, 75]);
  const [VoltageLocation, setVoltLoc] = useState(["loc1", "loc2", "loc3"]);
  const [Voltage, setVolt] = useState([25, 50, 75]);
  const [CurrentLocation, setCurrLoc] = useState(["loc1", "loc2", "loc3"]);
  const [Current, setCurrent] = useState([25, 50, 75]);
  const [Speed, setSpeed] = useState(25);
  const [Distance, setDistance] = useState(25);
  const [pitch, setPitch] = useState(25);
  const [roll, setRoll] = useState(50);
  const [yaw, setYaw] = useState(75);
  return (
    <div className="SensorContainer">
      <RotationIndicator pitch={pitch} roll={roll} yaw={yaw} />
      <div className="SpeedDistContainer">
        <SpeedDistBox title="Speed" value={Speed} unit="km/h" />
        <SpeedDistBox title="Distance" value={Distance} unit="km/h" />
      </div>
      <SensorBox
        title="Temperature"
        sensor_location={TempLocation}
        value={Temperature}
        unit="°C"
      />
      <SensorBox
        title="Pressure"
        sensor_location={PressureLocation}
        value={Pressure}
        unit="PSI"
      />
      <SensorBox
        title="Voltage"
        sensor_location={VoltageLocation}
        value={Voltage}
        unit="V"
      />
      <SensorBox
        title="Current"
        sensor_location={CurrentLocation}
        value={Current}
        unit="A"
      />
    </div>
  );
}

export default SensorContainer;
