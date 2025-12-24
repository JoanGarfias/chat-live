def broadcast(mensaje, clientes, ip_excluida=None):
    """Envía mensaje a todos los sockets de LECTURA excepto al cliente que lo envió.
    
    Args:
        mensaje: El mensaje a transmitir
        clientes: Diccionario {ip: {"read": socket, "write": socket}}
        ip_excluida: IP del cliente que envió el mensaje (no se le reenvía)
    """
    print(f"Transmitiendo mensaje a otros clientes (excluir: {ip_excluida})...")
    print(f"Clientes conectados: {list(clientes.keys())}")
    
    for ip, sockets in clientes.items():
        # No reenviar al cliente que envió el mensaje
        if ip == ip_excluida:
            print(f"  Saltando {ip} (es el emisor)")
            continue
        
        # Solo enviar si tiene socket de lectura
        if sockets["read"] is not None:
            try:
                sockets["read"].send(mensaje.encode())
                print(f"  ✓ Mensaje enviado a {ip}")
            except Exception as e:
                print(f"  ✗ Error al enviar mensaje a {ip}: {e}")
                try:
                    sockets["read"].close()
                    sockets["read"] = None
                except:
                    pass
        else:
            print(f"  - {ip} no tiene socket de lectura")
