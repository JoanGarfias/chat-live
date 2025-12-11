import socket
from core.config import IP, PORT
from handlers.client_handler import iniciar_hilo_cliente

server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server.bind((IP, PORT))
server.listen()

clientes = []

print(f"Servidor escuchando en {IP}:{PORT}")

while True:
    cliente_socket, direccion = server.accept()
    clientes.append(cliente_socket)
    iniciar_hilo_cliente(cliente_socket, direccion, clientes)
