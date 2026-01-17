import requests
import re
import sys
import random
import string
from bs4 import BeautifulSoup

# TARGET CONFIG
BASE_URL = "http://g.orehus.club"
SIGNUP_URL = f"{BASE_URL}/user/sign_up"
LOGIN_URL = f"{BASE_URL}/user/login"
SEARCH_URL = f"{BASE_URL}/explore/repos"

# ATTACKER PROFILE
USERNAME = "audit_sys_" + ''.join(random.choices(string.ascii_lowercase + string.digits, k=4))
PASSWORD = "StrongP@ssw0rd!" + ''.join(random.choices(string.digits, k=3))
EMAIL = f"dev.alex.smith.{random.randint(100,999)}@gmail.com"

# HEADERS (Act like a browser, not a script)
HEADERS = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Referer": BASE_URL,
    "Origin": BASE_URL,
    "Upgrade-Insecure-Requests": "1"
}

def log(msg, type="INFO"):
    colors = {"INFO": "\033[94m", "SUCCESS": "\033[92m", "FAIL": "\033[91m", "WARN": "\033[93m"}
    end = "\033[0m"
    print(f"[{colors.get(type, '')}{type}{end}] {msg}")

def main():
    s = requests.Session()
    s.headers.update(HEADERS)

    # 1. GET REGISTRATION PAGE (Grab CSRF & Cookies)
    log(f"Fetching {SIGNUP_URL}...", "INFO")
    try:
        r = s.get(SIGNUP_URL, timeout=15)
        if r.status_code != 200:
            log(f"Failed to load page: {r.status_code}", "FAIL")
            return
    except Exception as e:
        log(f"Connection error: {e}", "FAIL")
        return

    # Parse CSRF
    soup = BeautifulSoup(r.text, 'html.parser')
    csrf_input = soup.find('input', {'name': '_csrf'})
    
    if not csrf_input:
        log("CSRF token not found! Parsing manually...", "WARN")
        # Fallback regex
        match = re.search(r'name="_csrf" value="(.*?)"', r.text)
        if match:
            csrf_token = match.group(1)
        else:
            log("Could not extract CSRF token. Aborting.", "FAIL")
            return
    else:
        csrf_token = csrf_input['value']

    log(f"Got CSRF Token: {csrf_token[:10]}...", "SUCCESS")

    # 2. REGISTER USER
    payload = {
        "_csrf": csrf_token,
        "user_name": USERNAME,
        "email": EMAIL,
        "password": PASSWORD,
        "retype": PASSWORD
    }

    log(f"Attempting to register: {USERNAME} / {PASSWORD}", "INFO")
    r = s.post(SIGNUP_URL, data=payload)

    # 3. VERIFY REGISTRATION
    if r.status_code == 200:
        # Forgejo usually redirects to home or login on success. 
        # If we are still 200 OK on sign_up, check for errors.
        if "user_name" in r.text and "class=\"error\"" in r.text:
            log("Registration failed (Form Error). Parsing...", "FAIL")
            soup = BeautifulSoup(r.text, 'html.parser')
            error_msg = soup.find('div', {'class': 'ui message error'})
            if error_msg:
                print(f"Server says: {error_msg.text.strip()}")
            return
        
        log("Registration successful! (Or at least no error)", "SUCCESS")
        
        # 4. EXPLORE
        log("Scanning for r00ter33's repositories...", "INFO")
        r_search = s.get(SEARCH_URL + "?q=r00ter33&topic=&language=&sort=recentupdate")
        if "r00ter33" in r_search.text:
            log("Found r00ter33 content!", "SUCCESS")
            # Extract links
            links = re.findall(r'href="/r00ter33/([\w\-_]+)"', r_search.text)
            unique_repos = list(set(links))
            for repo in unique_repos:
                print(f"  [+] Found Repo: {repo}")
        else:
            log("No direct hits for r00ter33 in public search. Checking users...", "WARN")
            
        # Check User Profile
        r_user = s.get(f"{BASE_URL}/r00ter33")
        if r_user.status_code == 200:
            log("User 'r00ter33' exists!", "SUCCESS")
        elif r_user.status_code == 404:
            log("User 'r00ter33' not found (404).", "FAIL")
        else:
            log(f"User check status: {r_user.status_code}", "WARN")

    elif r.status_code == 403:
        log("403 Forbidden. IP banned or Email blocked.", "FAIL")
    else:
        log(f"Unexpected status: {r.status_code}", "WARN")

if __name__ == "__main__":
    main()
