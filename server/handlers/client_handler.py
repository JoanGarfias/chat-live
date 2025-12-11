import threading
from core.broadcast import broadcast

def manejar_cliente(cliente_socket, direccion, clientes):
    print(f"Cliente conectado desde {direccion}")

    while True:
        try:
            mensaje = cliente_socket.recv(1024).decode()

            if mensaje:
                print(f"Mensaje recibido de {direccion}: {mensaje}")
                broadcast(mensaje, clientes, cliente_socket)
            else:
                print(f"Cliente {direccion} desconectado.")
                clientes.remove(cliente_socket)
                cliente_socket.close()
                break

        except:
            print(f"Error al manejar cliente {direccion}.")
            clientes.remove(cliente_socket)
            cliente_socket.close()
            break


def iniciar_hilo_cliente(cliente_socket, direccion, clientes):
    hilo = threading.Thread(
        target=manejar_cliente,
        args=(cliente_socket, direccion, clientes)
    )
    hilo.start()
