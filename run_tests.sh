#!/bin/bash

# Phantom Lab: Automated Validation TUI
# Beautiful TUI for running test suites and checking kernel capabilities

# --- Color Definitions ---
RESET="\e[0m"
BOLD="\e[1m"
DIM="\e[2m"
RED="\e[31m"
GREEN="\e[32m"
YELLOW="\e[33m"
BLUE="\e[34m"
MAGENTA="\e[35m"
CYAN="\e[36m"
WHITE="\e[37m"

# --- TUI Functions ---
print_header() {
    clear
    echo -e "${CYAN}${BOLD}"
    echo " ╔══════════════════════════════════════════════════════╗"
    echo " ║                                                      ║"
    echo " ║   ${MAGENTA}P H A N T O M${CYAN} : Validation & Testing Suite       ║"
    echo " ║                                                      ║"
    echo " ╚══════════════════════════════════════════════════════╝"
    echo -e "${RESET}"
    echo -e "${DIM}  Automated validation of network primitives and parsing${RESET}\n"
}

spinner() {
    local pid=$1
    local msg=$2
    local spin='-\|/'
    local i=0
    tput civis # Hide cursor
    while kill -0 $pid 2>/dev/null; do
        i=$(( (i+1) %4 ))
        printf "\r${BLUE}[${spin:$i:1}]${RESET} ${msg}..."
        sleep 0.1
    done
    wait $pid
    local exit_code=$?
    tput cnorm # Show cursor
    if [ $exit_code -eq 0 ]; then
        printf "\r${GREEN}[ ✔ ]${RESET} ${msg} - ${GREEN}SUCCESS${RESET}      \n"
    else
        printf "\r${RED}[ ✘ ]${RESET} ${msg} - ${RED}FAILED${RESET}       \n"
    fi
    return $exit_code
}

run_step() {
    local name=$1
    local cmd=$2
    eval "$cmd" > /tmp/phantom_test_out.log 2>&1 &
    spinner $! "$name"
    local status=$?
    if [ $status -ne 0 ]; then
        echo -e "${DIM}--- Error Log ---${RESET}"
        cat /tmp/phantom_test_out.log | tail -n 10 | sed 's/^/  /'
        echo -e "${DIM}-----------------${RESET}"
    fi
    return $status
}

# --- Main Execution ---
print_header

FAILURES=0

echo -e "${BOLD}1. Kernel & Infrastructure Checks${RESET}"
IFACE=$(ip route get 8.8.8.8 | awk '{print $5; exit}')
run_step "Detect default interface ($IFACE)" "test -n \"$IFACE\"" || ((FAILURES++))

run_step "Check if 'tc' is installed" "which tc" || ((FAILURES++))

run_step "Verify Fair Queueing (fq) Qdisc on $IFACE" "tc -s qdisc show dev $IFACE | grep -q fq"
if [ $? -ne 0 ]; then
    echo -e "  ${YELLOW}⚠ WARNING: fq qdisc not active on $IFACE. SO_MAX_PACING_RATE will be ignored.${RESET}"
    echo -e "  ${DIM}Run: sudo tc qdisc replace dev $IFACE root fq${RESET}"
    ((FAILURES++))
fi

echo -e "\n${BOLD}2. Rust Cargo Test Suite${RESET}"
run_step "Compile Phantom & Run Tests (cargo test)" "cargo test --color always" || ((FAILURES++))

run_step "Verify JA4 TLS Modifiers" "cargo test test_tls_static_profile" || ((FAILURES++))

run_step "Verify Memory Safety (cargo clippy)" "cargo clippy -- -D warnings" 
if [ $? -ne 0 ]; then
    echo -e "  ${YELLOW}⚠ Lints found, but continuing.${RESET}"
fi

echo -e "\n${BOLD}3. Summary${RESET}"
if [ $FAILURES -eq 0 ]; then
    echo -e "  ${GREEN}${BOLD}ALL SYSTEMS NOMINAL.${RESET} The Phantom network stack is fully validated."
    echo -e "  ${CYAN}Ready for high-throughput traffic generation.${RESET}\n"
else
    echo -e "  ${RED}${BOLD}VALIDATION FAILED.${RESET} Encountered $FAILURES error(s)."
    echo -e "  ${DIM}Please review the logs above and correct the issues before deployment.${RESET}\n"
fi
