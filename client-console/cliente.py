import socket
import threading

IP = "172.30.144.1"
PORT = 5000

cliente = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
cliente.connect((IP, PORT))

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
    cliente.send(mensaje.encode())
cliente.close()
print("Desconectado del servidor.")