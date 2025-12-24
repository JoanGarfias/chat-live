import socket
from core.config import IP, PORT
from handlers.client_handler import iniciar_hilo_cliente

server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
server.bind((IP, PORT))
server.listen()

# Diccionario: {ip: {"read": socket, "write": socket}}
clientes = {}

print(f"Servidor escuchando en {IP}:{PORT}")

while True:
    cliente_socket, direccion = server.accept()
    iniciar_hilo_cliente(cliente_socket, direccion, clientes)
