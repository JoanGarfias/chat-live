import socket
import threading
import json

IP = "192.168.100.56"
PORT = 5000

cliente = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
cliente.connect((IP, PORT))

autor = input("Ingresa tu nombre de usuario: ")

print("")

mensajeData = {
    "autor": autor,
    "mensaje": "",
}

def recibir():
    while True:
        try:
            mensaje = cliente.recv(1024).decode()
            if mensaje:
                print(mensaje + "\n")
            else:
                print("Conexión cerrada por el servidor.")
                break
        except:
            print("Error al recibir el mensaje.")
            cliente.close()
            break

hilo = threading.Thread(target=recibir)
hilo.start()

print("Conectado al servidor. Puedes empezar a enviar mensajes.")
while True:
    mensaje = input("")
    if( mensaje.lower() == "salir" ):
        break
    mensajeData["mensaje"] = mensaje
    cliente.send(json.dumps(mensajeData).encode())
cliente.close()
print("Desconectado del servidor.")