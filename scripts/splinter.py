import socket
import struct
import sys
import time
import random

def checksum(msg):
    s = 0
    # loop taking 2 characters at a time
    for i in range(0, len(msg), 2):
        if i+1 < len(msg):
            w = (msg[i] << 8) + msg[i+1]
        else:
            w = (msg[i] << 8)
        s = s + w
    
    s = (s >> 16) + (s & 0xffff)
    s = s + (s >> 16)
    
    # complement and mask to 4 byte short
    s = ~s & 0xffff
    
    return s

def create_ip_header(source_ip, dest_ip, proto, packet_id, frag_offset=0, mf=0):
    # IP Header fields
    version = 4
    ihl = 5
    tos = 0
    tot_len = 20 + 20 # IP Header + TCP Header (minimum) - payload added later
    id = packet_id
    # Flags (3 bits): 0, DF, MF. Offset (13 bits)
    flags_offset = (mf << 13) | frag_offset
    ttl = 255
    protocol = proto
    check = 0
    saddr = socket.inet_aton(source_ip)
    daddr = socket.inet_aton(dest_ip)

    header_fmt = '!BBHHHBBH4s4s'
    header = struct.pack(header_fmt, 
                         (version << 4) + ihl, 
                         tos, 
                         tot_len, 
                         id, 
                         flags_offset, 
                         ttl, 
                         protocol, 
                         check, 
                         saddr, 
                         daddr)
    return header

# Note: Raw socket implementation requires ROOT and manual TCP crafting which is complex for a quick script.
# Instead, we will use a simpler approach:
# We open a legitimate socket, but we force small sends to induce fragmentation at the NIC level
# OR we rely on the OS to handle the handshake and we just spam small packets.

# HOWEVER, to bypass a firewall that DROPS the handshake, we must be stealthy.
# Since standard connect() fails (as we saw), we MUST use raw sockets to spoof or fragment the handshake itself.

# BUT, crafting a full TCP stack in one script is overkill and prone to errors (seq/ack sync).
# STRATEGY CHANGE: We will use the 'phantom' technique but carry a payload.

print("[-] Complex Raw Socket TCP Stack required for full evasion.")
print("[*] Switching to Application-Layer Fragmentation strategy.")

def fragment_flood(target_ip, target_port):
    try:
        # Standard socket
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.settimeout(5)
        
        print(f"[+] Connecting to {target_ip}:{target_port}...")
        # If the firewall blocks the SYN, this will fail. 
        # But earlier scans showed ports OPEN via SYN scan.
        # This implies the firewall allows SYN but maybe drops PSH/ACK with data?
        
        s.connect((target_ip, target_port)) 
        print("[+] Connected! (Unexpected?)")
        
        payload = b"GET /version HTTP/1.0\r\n\r\n"
        
        # Send byte by byte (Application Layer Fragmentation)
        print("[*] Sending fragmented payload...")
        for byte in payload:
            s.send(bytes([byte]))
            time.sleep(0.1) # Delay to confuse IDS
            
        response = s.recv(4096)
        print(f"\n[+] RESPONSE:\n{response.decode(errors='ignore')}")
        s.close()
        
    except Exception as e:
        print(f"[-] Failed: {e}")

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print(f"Usage: {sys.argv[0]} <IP> <PORT>")
        sys.exit(1)
        
    fragment_flood(sys.argv[1], int(sys.argv[2]))
