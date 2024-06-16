import socket
import struct
import time

def push_integer():
    # Create a UDP socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

    # Set the IP address and port
    ip_address = '127.0.0.1'
    port = 1236

    # Initialize the integer value
    value = 0

    while True:
        # Convert the integer to little endian bytes
        data = struct.pack('<i', value)

        # Send the data to the specified IP address and port
        sock.sendto(data, (ip_address, port))

        # Increment the integer value
        value += 1
        print("Pushed:"+str(value))
        
        # Wait for 1 second
        time.sleep(1)

# Call the function to start pushing integers
push_integer()