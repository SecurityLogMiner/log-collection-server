import socket

server_ip = '0.0.0.0'
server_port = 45612

server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

server_socket.bind((server_ip,server_port))

server_socket.listen(5)

print(f"Listening on {server_ip}:{server_port}...")

while True:
    client_socket, client_address = server_socket.accept()
    print(f"Accecpted connection from {client_address[0]}:{client_address[1]}")

    data = client_socket.recv(1024)
    
    if not data:
        break

    print(f"Received: {data.decode()}")

    client_socket.send(data)

    client_socket.close()

server_socket.close()
