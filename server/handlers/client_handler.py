import threading
import json
from core.broadcast import broadcast

def manejar_cliente(cliente_socket, direccion, clientes):
    print(f"Cliente conectado desde {direccion}")
    ip = direccion[0]
    
    # Leer el primer mensaje para identificar tipo de conexión
    try:
        primer_mensaje = cliente_socket.recv(1024).decode()
        tipo_info = json.loads(primer_mensaje)
        tipo_conexion = tipo_info.get("type")
        
        # Inicializar entrada del cliente si no existe
        if ip not in clientes:
            clientes[ip] = {"read": None, "write": None}
        
        # Registrar el socket según su tipo
        if tipo_conexion == "write":
            clientes[ip]["write"] = cliente_socket
            print(f"Socket de ESCRITURA registrado para {ip}")
        elif tipo_conexion == "read":
            clientes[ip]["read"] = cliente_socket
            print(f"Socket de LECTURA registrado para {ip}")
        
        print(f"Estado del cliente {ip}: read={clientes[ip]['read'] is not None}, write={clientes[ip]['write'] is not None}")
    except Exception as e:
        print(f"Error al identificar tipo de conexión de {direccion}: {e}")
        cliente_socket.close()
        return

    while True:
        try:
            mensaje = cliente_socket.recv(1024).decode()

            if mensaje:
                print(f"Mensaje recibido de {direccion}: {mensaje}")
                broadcast(mensaje, clientes, ip)
            else:
                print(f"Cliente {direccion} desconectado.")
                # Limpiar el socket del diccionario
                if ip in clientes:
                    if clientes[ip]["read"] == cliente_socket:
                        clientes[ip]["read"] = None
                    if clientes[ip]["write"] == cliente_socket:
                        clientes[ip]["write"] = None
                    # Si ambos son None, eliminar la entrada
                    if clientes[ip]["read"] is None and clientes[ip]["write"] is None:
                        del clientes[ip]
                cliente_socket.close()
                break

        except Exception as e:
            print(f"Error al manejar cliente {direccion}: {e}")
            # Limpiar el socket del diccionario
            if ip in clientes:
                if clientes[ip]["read"] == cliente_socket:
                    clientes[ip]["read"] = None
                if clientes[ip]["write"] == cliente_socket:
                    clientes[ip]["write"] = None
                # Si ambos son None, eliminar la entrada
                if clientes[ip]["read"] is None and clientes[ip]["write"] is None:
                    del clientes[ip]
            cliente_socket.close()
            break


def iniciar_hilo_cliente(cliente_socket, direccion, clientes):
    hilo = threading.Thread(
        target=manejar_cliente,
        args=(cliente_socket, direccion, clientes)
    )
    hilo.start()
