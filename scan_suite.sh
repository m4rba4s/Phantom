#!/bin/bash
# Phantom Scan Validation Suite

TARGET=${1:-185.158.133.1}
BIN="target/debug/phantom"

echo "============================================="
echo "   PHANTOM: Stealth Scan Validation Suite    "
echo "============================================="
echo "Target: $TARGET"
echo ""

echo "[*] Building Phantom (if needed)..."
echo "[*] Assuming Phantom is already built at $BIN"
if [ ! -f "$BIN" ]; then
    echo "[-] Binary not found at $BIN. Run 'cargo build' first."
    exit 1
fi

echo "[*] Testing sudo access for raw socket capabilities..."
sudo -v

echo ""
echo "---------------------------------------------"
echo "[1/4] Basic Stealth Scan (Default MTU 24, Delay 100ms)"
echo "---------------------------------------------"
sudo $BIN --i-am-authorized scan $TARGET -p 80,443,22

echo ""
echo "---------------------------------------------"
echo "[2/4] Vanilla Scan (No Fragmentation)"
echo "---------------------------------------------"
sudo $BIN --i-am-authorized scan $TARGET -p 80,443,22 --no-fragment

echo ""
echo "---------------------------------------------"
echo "[3/4] Advanced Evasion (Decoys & High Delay)"
echo "---------------------------------------------"
sudo $BIN --i-am-authorized scan $TARGET -p 80,443,22 --decoys 5 --delay 500

echo ""
echo "---------------------------------------------"
echo "[4/4] Aggressive Discovery (Fast)"
echo "---------------------------------------------"
# Using a subset of ports for the demo script so it doesn't take forever, but demonstrating aggressive timing
sudo $BIN --i-am-authorized scan $TARGET -p 21,22,23,53,80,443,8080,3389,8443 --no-fragment --delay 10 -a

echo ""
echo "============================================="
echo " [+] All validation scans completed!         "
echo "============================================="
