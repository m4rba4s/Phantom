import re
import math
import sys
import os

def shannon_entropy(data):
    if not data:
        return 0
    entropy = 0
    for x in range(256):
        p_x = float(data.count(chr(x)))/len(data)
        if p_x > 0:
            entropy += - p_x*math.log(p_x, 2)
    return entropy

def analyze_js(filename):
    print(f"[+] Analyzing {filename}...")
    
    try:
        with open(filename, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
    except FileNotFoundError:
        print("[-] File not found.")
        return

    # 1. Extract Strings (Single and Double Quoted)
    # Simple regex for string literals (handles escaped quotes basic)
    strings = re.findall(r'["\'](.*?)["\']', content)
    
    print(f"[+] Found {len(strings)} string literals.")

    api_endpoints = []
    secrets = []
    urls = []
    
    for s in strings:
        # Filter out short trash
        if len(s) < 4:
            continue

        # Check for API endpoints
        if s.startswith('/api/') or s.startswith('/v1/') or '/user/' in s or '/admin/' in s:
            api_endpoints.append(s)
        
        # Check for URLs
        if s.startswith('http://') or s.startswith('https://') or s.startswith('ws://'):
            urls.append(s)

        # Check for Secrets (High Entropy + Key-like patterns)
        # Entropy threshold usually > 4.5 for random base64/hex strings
        entropy = shannon_entropy(s)
        if entropy > 4.5 and len(s) > 16 and " " not in s:
            secrets.append((s, entropy))
        
        # Keyword search
        if "token" in s.lower() or "secret" in s.lower() or "password" in s.lower() or "key" in s.lower():
             if len(s) < 100: # Ignore long texts
                secrets.append((s, 0.0)) # 0.0 entropy for keyword matches

    print("\n--- [ API Endpoints / Routes ] ---")
    for api in sorted(set(api_endpoints))[:20]: # Show top 20
        print(f"  {api}")

    print("\n--- [ Potential URLs ] ---")
    for url in sorted(set(urls))[:20]:
        print(f"  {url}")

    print("\n--- [ Potential Secrets / High Entropy Strings ] ---")
    unique_secrets = sorted(list(set(secrets)), key=lambda x: x[1], reverse=True)
    for s, ent in unique_secrets[:20]:
        print(f"  [{ent:.2f}] {s}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python3 js_hunter.py <file.js>")
    else:
        analyze_js(sys.argv[1])
