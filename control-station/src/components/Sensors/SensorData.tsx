import Camera from "@/components/Sensors/Camera";
import Console from "@/components/Sensors/Console";
import SensorContainer from "@/components/Sensors/SensorContainer";

import "./SensorData.css";

function SensorData() {
	return (
		<div className="sensordata">
			<SensorContainer />
            <Camera/>
            <Console/>
		</div>
	);
}

export default SensorData;