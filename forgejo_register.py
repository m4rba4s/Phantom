import requests
import re
import sys

URL = "http://g.orehus.club"
USER = "attacker"
PASS = "AttackerPass123!"
EMAIL = "attacker@example.com"

s = requests.Session()

# Get CSRF
print("[*] Getting CSRF...")
try:
    r = s.get(f"{URL}/user/sign_up", timeout=10)
    csrf_match = re.search(r'name="_csrf" value="([^"]+)"', r.text)
    if not csrf_match:
        print("[-] CSRF not found")
        sys.exit(1)
    csrf = csrf_match.group(1)
    print(f"[+] CSRF: {csrf}")
except Exception as e:
    print(f"[-] Error: {e}")
    sys.exit(1)

# Register
data = {
    "_csrf": csrf,
    "user_name": USER,
    "email": EMAIL,
    "password": PASS,
    "retype": PASS
}

print(f"[*] Registering {USER}...")
try:
    r = s.post(f"{URL}/user/sign_up", data=data, timeout=10)
    if r.status_code == 200 and "Log Out" in r.text or "Welcome" in r.text:
        print("[+] Registration SUCCESS!")
    elif "Username has been already taken" in r.text:
         print("[!] User already exists, trying login...")
    else:
        print(f"[-] Registration Failed. Code: {r.status_code}")
        # print(r.text[:500])
except Exception as e:
    print(f"[-] Error: {e}")
    sys.exit(1)

# Login verify
print("[*] Verifying Login...")
try:
    # Get login CSRF
    r = s.get(f"{URL}/user/login", timeout=10)
    csrf_match = re.search(r'name="_csrf" value="([^"]+)"', r.text)
    if csrf_match:
        csrf = csrf_match.group(1)
        data = {
            "_csrf": csrf,
            "user_name": USER,
            "password": PASS
        }
        r = s.post(f"{URL}/user/login", data=data, timeout=10)
        if "Log Out" in r.text:
             print("[+] Login SUCCESS!")
        else:
             print("[-] Login Failed.")
except Exception as e:
    print(f"[-] Error: {e}")
