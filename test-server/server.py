from socketio import Server, WSGIApp
import eventlet

class SocketServer:
    def __init__(self):
        self.sio = Server(cors_allowed_origins="*")
        self.app = WSGIApp(self.sio)

    def send_data(self, event, data):
        self.sio.emit(event, data)

    def register_event(self, event_name, callback):
        @self.sio.event
        def wrapper(data):
            callback(data)

    def start_server(self, host='127.0.0.1', port=5000):
        print(f"Socket.IO server running at http://{host}:{port}")
        eventlet.wsgi.server(eventlet.listen((host, port)), self.app)
