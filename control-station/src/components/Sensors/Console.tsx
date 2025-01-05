import { useEffect, useRef, useState } from "react";
import "./SensorData.css";

function Console() {
  // Filler podData with messages
  const [podData, setPodData] = useState({
    messages: [
      { timestamp: new Date(), message: "System initialized" },
      { timestamp: new Date(), message: "All systems nominal" },
      { timestamp: new Date(), message: "Warning: Sensor malfunction" },
    ],
  });
  
  const listEndRef = useRef<HTMLLIElement | null>(null);

  useEffect(() => {
    if (listEndRef.current) {
      listEndRef.current.scrollIntoView({ behavior: "smooth" });
    }
  }, [podData.messages]);

  return (
    <div className="console">
      <h2>Console</h2>
      <ul className="console-list">
        {podData.messages.map((prop, index) => (
          <li
            key={index}
            className="console-list-item"
            ref={index === podData.messages.length - 1 ? listEndRef : null}
          >
            {prop.timestamp.toLocaleTimeString("en-US", { hour12: false })} &nbsp;
            {prop.message.toUpperCase()}
          </li>
        ))}
      </ul>
    </div>
  );
}

export default Console;
