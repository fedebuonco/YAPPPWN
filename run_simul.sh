#!/bin/bash

# Check if a pcap file was provided
if [ -z "$1" ]; then
  echo "Usage: $0 <path_to_pcap_file>"
  exit 1
fi

PCAP_FILE="$1"

# Function to open a new terminal and run a command
run_in_new_terminal() {
    osascript <<EOF
    tell application "Terminal"
        do script "$1"
    end tell
EOF
}

# Command 1: Replay the pcap file on interface en1
run_in_new_terminal "tcpreplay -i en1 -l 2 '$PCAP_FILE'"

# Command 2: Replay the pcap file on interface en2
run_in_new_terminal "tcpreplay -i en2 '$PCAP_FILE'"

# Command 3: Activate virtual environment and run PPPwn script
run_in_new_terminal "cd ~/Desktop/PPPwn && source .venv/bin/activate && python3 pppwn.py --interface=en1 --fw=1100"

# Command 4: Run YAPPwn with stage binaries
run_in_new_terminal "cd ~/Desktop/yapppwn && ./target/release/yapppwn -i en2 --fw 1100 --stage-1 ./stage1/stage1.bin --stage-2 ./stage2/stage2.bin"

# Wait for user input to terminate terminals
read -p "Press any key to kill all terminals..."

# Kill all Terminal windows (optional: only if you want to close them)
osascript -e 'tell application "Terminal" to close every window' > /dev/null 2>&1

echo "All terminal windows have been closed."
