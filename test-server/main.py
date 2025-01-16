from server import SocketServer
from data import generate_sensor_data
import time
import threading

def emit_sensor_data(server):
    while True:
        data = generate_sensor_data()
        server.send_data('sensor_data', data)
        time.sleep(1)

if __name__ == "__main__":
    myServer = SocketServer()

    threading.Thread(target=emit_sensor_data, args=(myServer,), daemon=True).start()

    myServer.start_server()
