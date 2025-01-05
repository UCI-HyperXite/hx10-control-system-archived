import React, { useState }from 'react';
import Navbar from "@/components/Navbar/Navbar";
import StatusIndicator from "@/components/StatusIndicator/StatusIndicator";
import ControlPanel from "@/components/ControlPanel/ControlPanel";
import SensorData from './components/Sensors/SensorData';
import './App.css';

function App() {

  return (
    <main>
      <Navbar />
      <SensorData/>
      <ControlPanel />
      <StatusIndicator status="green" />
		</main>
  );
}

export default App;
