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
import ssl
import socket

def handle_client(client_socket):
    request = client_socket.recv(1024)
    printf(f"Received: {requiest}")
    if not request:
        return
    print(f"Received: {request.decode()}")
    client_socket.send(b"Server hello")
    client_socket.close()
 


server_address = ('0.0.0.0', 45612)
certfile = ""
keyfile = ""

server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

server_socket.bind(server_address)

context = ssl.create_default_context(ssl.Purpose.CLIENT_AUTH)
context.load_cert_chain(certfile=certfile,keyfile=keyfile)
context.options |= ssl.OP_NO_TLSv1 | ssl.OP_NO_TLSv1_1 #disable tls v1 and v1.1

server_socket.listen(5)

print(f"Listening on {server_ip}:{server_port}...")

try:
    while True:
        client_socket, client_address = server_socket.accept()
        print(f"Accecpted connection from {client_address[0]}:{client_address[1]}")

        ssl_client_socket = context.wrap_socket(client_socket, server_side=True)

        handle_client(ssl_client_socket)

finally:
    server_socket.close()
