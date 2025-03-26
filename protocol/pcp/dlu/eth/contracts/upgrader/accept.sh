# accept.sh

#!/bin/bash

# Initialize contract variable
contract=""
url=""

# Parse options using getopts
while getopts "c:u:" opt; do
  case $opt in
    c) contract="$OPTARG"
    ;;
    u) url="$OPTARG"
    ;;
    \?) echo "Invalid option: -$OPTARG" >&2
        exit 1
    ;;
  esac
done

echo "Contract: $contract"
echo "URL: $url"

# Ensure the contract flag is provided
if [ -z "$contract" ]; then
  echo "Error: -c flag for contract is required."
  exit 1
fi

# Ensure the url flag is provided
if [ -z "$url" ]; then
  echo "Error: -u flag for url is required."
  exit 1
fi

# Convert contract name to lowercase
lowercase_contract=$(echo "$contract" | tr '[:upper:]' '[:lower:]')

# Run the upgrader script
echo "Running upgrader/accept.ts"
npx tsx ./accept.ts -c "$lowercase_contract" -u $url
