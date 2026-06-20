import requests
import urllib3

urllib3.disable_warnings(urllib3.exceptions.InsecureRequestWarning)

TARGETS = [
    "http://g.orehus.club",
    "http://g.orehus.club/user/login",
    "http://79.137.248.131:3003",
    "http://79.137.248.131:3003/_matrix/client/versions",
    "http://77.221.143.142:3003",
    "http://79.137.248.131:2375/_ping"
]

ART = """
      _    _ 
     / \  / \\
    (   \/   )  HACKED BY 0ut
     \      /   OREHUS & SKUNK = LAME
      \    /    CARDING IS FOR LOSERS
       \  /     WE SEE EVERYTHING
        \/ 
"""

HEADERS = {
    "User-Agent": "Mozilla/5.0 (Pentest) FUCKYOU_SCAMMERS/6.6.6",
    "X-Hacked-By": "0ut",
    "X-Target-Status": "PWNED",
    "Referer": "https://interpol.int/crimes/carding",
    "From": "you_are_watched@nsa.gov"
}

print("--- GRAFFITI PAINTER V2.0 ---")
for t in TARGETS:
    try:
        print(f"[*] Spraying {t}...", end="")
        # GET with headers
        requests.get(t, headers=HEADERS, timeout=3, verify=False)
        
        # POST with body art
        requests.post(t, headers=HEADERS, data=ART, timeout=3, verify=False)
        print(" [Painted]")
    except Exception:
        print(" [Hit]") # Errors are expected on raw ports

print("\n[*] JOB DONE. Logs infected.")