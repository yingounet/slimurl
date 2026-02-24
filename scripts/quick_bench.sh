#!/bin/bash
set -e

BASE_URL="${BASE_URL:-http://localhost:3000}"

echo "=== URLSlim Quick Benchmark ==="

if ! curl -sf "$BASE_URL/" > /dev/null 2>&1; then
    echo "Server not responding at $BASE_URL"
    echo "Start with: docker compose up -d"
    exit 1
fi

echo "Creating test link..."
CODE=$(curl -s -X POST "$BASE_URL/api/v1/links" \
    -H "Content-Type: application/json" \
    -d '{"url":"https://example.com/quick-test"}' | jq -r '.code')

echo "Test code: $CODE"
echo ""

if command -v hey &> /dev/null; then
    echo "Running benchmark with hey..."
    hey -z 10s -q 1000 -c 100 "$BASE_URL/$CODE"
elif command -v wrk &> /dev/null; then
    echo "Running benchmark with wrk..."
    wrk -t4 -c100 -d10s "$BASE_URL/$CODE"
elif command -v ab &> /dev/null; then
    echo "Running benchmark with ab..."
    ab -n 10000 -c 100 "$BASE_URL/$CODE"
else
    echo "No benchmark tool found. Install one of: hey, wrk, ab"
    echo "  brew install hey"
    exit 1
fi
