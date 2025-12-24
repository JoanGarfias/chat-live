import threading
import json
from core.broadcast import broadcast

def manejar_cliente(cliente_socket, direccion, clientes):
    print(f"Cliente conectado desde {direccion}")
    
    # Leer el primer mensaje para identificar tipo de conexión
    try:
        primer_mensaje = cliente_socket.recv(1024).decode()
        tipo_info = json.loads(primer_mensaje)
        
        if tipo_info.get("type") == "write":
            clientes.append(cliente_socket)
            print(f"Conexión de escritura agregada a clientes: {direccion}")
        else:
            print(f"Conexión de lectura (no se agrega a lista): {direccion}")
    except:
        print(f"Error al identificar tipo de conexión de {direccion}")
        cliente_socket.close()
        return

    while True:
        try:
            mensaje = cliente_socket.recv(1024).decode()

            if mensaje:
                print(f"Mensaje recibido de {direccion}: {mensaje}")
                broadcast(mensaje, clientes, cliente_socket)
            else:
                print(f"Cliente {direccion} desconectado.")
                if cliente_socket in clientes:
                    clientes.remove(cliente_socket)
                cliente_socket.close()
                break

        except:
            print(f"Error al manejar cliente {direccion}.")
            if cliente_socket in clientes:
                clientes.remove(cliente_socket)
            cliente_socket.close()
            break


def iniciar_hilo_cliente(cliente_socket, direccion, clientes):
    hilo = threading.Thread(
        target=manejar_cliente,
        args=(cliente_socket, direccion, clientes)
    )
    hilo.start()
