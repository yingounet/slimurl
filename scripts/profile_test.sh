#!/bin/bash
set -e

BASE_URL="${BASE_URL:-http://localhost:3000}"
OUTPUT_DIR="${OUTPUT_DIR:-./benchmark_results/profile}"
DURATION="${DURATION:-60s}"

mkdir -p "$OUTPUT_DIR"

echo "=== URLSlim Profile Test ==="
echo "Target: $BASE_URL"
echo "Duration: $DURATION"
echo ""

curl -sf "$BASE_URL/" > /dev/null || { echo "Server not responding"; exit 1; }

TEST_CODE=$(curl -s -X POST "$BASE_URL/api/v1/links" \
    -H "Content-Type: application/json" \
    -d '{"url":"https://example.com/profile-test"}' | jq -r '.code')

echo "Test link: $TEST_CODE"

if command -v hey &> /dev/null; then
    echo ""
    echo "[1] Redirect Performance (cached)..."
    hey -z 10s -q 5000 -c 200 "$BASE_URL/$TEST_CODE" 2>&1 | tee "$OUTPUT_DIR/redirect_cached.txt"
    
    echo ""
    echo "[2] Redirect Performance (cold cache simulation)..."
    for i in {1..100}; do
        curl -s -X POST "$BASE_URL/api/v1/links" \
            -H "Content-Type: application/json" \
            -d "{\"url\":\"https://example.com/cold-$i\"}" > /dev/null
    done
    
    hey -z 10s -q 1000 -c 100 -m GET "$BASE_URL/[a-zA-Z0-9]{6}" 2>&1 | tee "$OUTPUT_DIR/redirect_cold.txt" || true
    
    echo ""
    echo "[3] Link Creation Performance..."
    hey -z 10s -q 100 -c 50 -m POST \
        -H "Content-Type: application/json" \
        -d '{"url":"https://example.com/create-test"}' \
        "$BASE_URL/api/v1/links" 2>&1 | tee "$OUTPUT_DIR/create.txt"
    
    echo ""
    echo "[4] QR Code Performance..."
    hey -z 10s -q 500 -c 50 "$BASE_URL/$TEST_CODE/qrcode?size=200" 2>&1 | tee "$OUTPUT_DIR/qrcode.txt"
    
    echo ""
    echo "[5] Sustained Load Test..."
    hey -z "$DURATION" -q 2000 -c 100 "$BASE_URL/$TEST_CODE" 2>&1 | tee "$OUTPUT_DIR/sustained.txt"
else
    echo "hey is not installed. Install with: brew install hey"
    exit 1
fi

echo ""
echo "=== Profile Complete ==="
echo "Results saved to $OUTPUT_DIR/"
