import socket
import ssl

TARGET_IP = "79.137.248.131"
TARGET_PORT = 3003
HOST_HEADER = "vpn.blackpatron.us"

def test_http():
    print(f"[*] Testing HTTP with Host: {HOST_HEADER}")
    try:
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.settimeout(10)
        s.connect((TARGET_IP, TARGET_PORT))
        
        req = (
            f"GET /_matrix/client/versions HTTP/1.1\r\n"
            f"Host: {HOST_HEADER}\r\n"
            f"Connection: close\r\n"
            f"\r\n"
        )
        
        s.sendall(req.encode())
        resp = s.recv(4096)
        print(f"[*] Response:\n{resp.decode(errors='replace')}")
        s.close()
    except Exception as e:
        print(f"[!] HTTP Error: {e}")

def test_https():
    print(f"[*] Testing HTTPS with Host: {HOST_HEADER}")
    try:
        context = ssl.create_default_context()
        context.check_hostname = False
        context.verify_mode = ssl.CERT_NONE
        
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.settimeout(10)
        conn = context.wrap_socket(s, server_hostname=HOST_HEADER)
        conn.connect((TARGET_IP, TARGET_PORT))
        
        req = (
            f"GET /_matrix/client/versions HTTP/1.1\r\n"
            f"Host: {HOST_HEADER}\r\n"
            f"Connection: close\r\n"
            f"\r\n"
        )
        
        conn.sendall(req.encode())
        resp = conn.recv(4096)
        print(f"[*] Response:\n{resp.decode(errors='replace')}")
        conn.close()
    except Exception as e:
        print(f"[!] HTTPS Error: {e}")

if __name__ == "__main__":
    test_http()
    print("-" * 20)
    test_https()

