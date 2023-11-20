'''
Goal:
    Create a TLS connection to a client, or set of clients.

Steps to consider:
    1. Client sends a hello, supported TLS version and supported cipher suites, and
    a random number.

    2. Server responds with a hello message, chosen TLS version, cipher suite,
    and a random number.

    3. Server sends its certificate to the Client containing the public key.

    4. Client verifies certificate and sends its own certificate to the Server.

    5. Client generates a pre-master secret, encrypts it using the server's public
    key and sends the encrypted pre-master secret to the Server.

    6. Server decrypts the master key with its private key and generates the master
    secret.

    7. Client and Server use the master secret to generate session keys for
    symmetric encryption and message authentication.

    8. Client send a Change Cipher Spec, indicating that it will begin using the
    session keys for encryption and message authentication.

    9. Client sends an encrypted handshake message to the server, using the
    session keys.

    10. Server sends its own Change Cipher Spec, indicating that it will start
    using the session keys for encryption and message authentication.

    11. Server sends an encrypted handshake message to the Client, which is
    encrypted using the session keys.

    12. TLS session is established.

'''
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
