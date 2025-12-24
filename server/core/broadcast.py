def broadcast(mensaje, clientes, cliente_excluido=None):
    print("Transmitiendo mensaje a otros clientes...")
    print("Clientes actuales:")
    print(clientes)
    print("Cliente excluido:")
    print(cliente_excluido)
    for cliente in clientes:
        if cliente != cliente_excluido:
            try:
                cliente.send(mensaje.encode())
            except:
                print("Error al enviar mensaje a un cliente.")
                clientes.remove(cliente)
                cliente.close()
