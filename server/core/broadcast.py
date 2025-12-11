def broadcast(mensaje, clientes, cliente_excluido=None):
    for cliente in clientes:
        if cliente != cliente_excluido:
            try:
                cliente.send(mensaje.encode())
            except:
                print("Error al enviar mensaje a un cliente.")
                clientes.remove(cliente)
                cliente.close()
