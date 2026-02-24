#!/bin/bash
set -e

BASE_URL="${BASE_URL:-http://localhost:3000}"
DURATION="${DURATION:-30s}"
RATE="${RATE:-1000}"
OUTPUT_DIR="${OUTPUT_DIR:-./benchmark_results}"

mkdir -p "$OUTPUT_DIR"

echo "=== URLSlim Performance Benchmark ==="
echo "Target: $BASE_URL"
echo "Duration: $DURATION"
echo "Rate: $RATE requests/sec"
echo "Output: $OUTPUT_DIR"
echo ""

check_command() {
    if ! command -v $1 &> /dev/null; then
        echo "Error: $1 is not installed"
        echo "Install with: brew install $1"
        exit 1
    fi
}

echo "[1/5] Health Check..."
curl -sf "$BASE_URL/" > /dev/null && echo "Server is running" || { echo "Server not responding"; exit 1; }

echo ""
echo "[2/5] Creating test links..."
TEST_CODE=$(curl -s -X POST "$BASE_URL/api/v1/links" \
    -H "Content-Type: application/json" \
    -d '{"url":"https://example.com/perf-test"}' | jq -r '.code')

if [ -z "$TEST_CODE" ] || [ "$TEST_CODE" = "null" ]; then
    echo "Failed to create test link"
    exit 1
fi
echo "Created test link: $TEST_CODE"

echo ""
echo "[3/5] Running redirect benchmark (read-heavy)..."
if command -v hey &> /dev/null; then
    hey -z "$DURATION" -q "$RATE" -c 100 "$BASE_URL/$TEST_CODE" > "$OUTPUT_DIR/redirect_hey.txt" 2>&1
    echo "Results saved to $OUTPUT_DIR/redirect_hey.txt"
elif command -v wrk &> /dev/null; then
    wrk -t4 -c100 -d"$DURATION" "$BASE_URL/$TEST_CODE" > "$OUTPUT_DIR/redirect_wrk.txt" 2>&1
    echo "Results saved to $OUTPUT_DIR/redirect_wrk.txt"
else
    echo "Neither hey nor wrk installed, skipping"
fi

echo ""
echo "[4/5] Running API creation benchmark (write)..."
if command -v hey &> /dev/null; then
    hey -z "$DURATION" -q "$RATE" -c 50 -m POST \
        -H "Content-Type: application/json" \
        -d '{"url":"https://example.com/test"}' \
        "$BASE_URL/api/v1/links" > "$OUTPUT_DIR/create_hey.txt" 2>&1
    echo "Results saved to $OUTPUT_DIR/create_hey.txt"
else
    echo "hey not installed, skipping"
fi

echo ""
echo "[5/5] Running vegeta benchmark (if available)..."
if command -v vegeta &> /dev/null; then
    echo "GET $BASE_URL/$TEST_CODE" | vegeta attack -duration="$DURATION" -rate="$RATE" | \
        vegeta report -type=text > "$OUTPUT_DIR/vegeta_report.txt" 2>&1
    echo "Results saved to $OUTPUT_DIR/vegeta_report.txt"
    
    echo "GET $BASE_URL/$TEST_CODE" | vegeta attack -duration="$DURATION" -rate="$RATE" | \
        vegeta plot > "$OUTPUT_DIR/plot.html" 2>&1
    echo "Plot saved to $OUTPUT_DIR/plot.html"
else
    echo "vegeta not installed, skipping"
fi

echo ""
echo "=== Benchmark Complete ==="
echo "Results saved to $OUTPUT_DIR/"

if [ -f "$OUTPUT_DIR/redirect_hey.txt" ]; then
    echo ""
    echo "--- Redirect Summary (hey) ---"
    grep -E "(Requests/sec|Average|99%)" "$OUTPUT_DIR/redirect_hey.txt" || true
fi

if [ -f "$OUTPUT_DIR/vegeta_report.txt" ]; then
    echo ""
    echo "--- Redirect Summary (vegeta) ---"
    cat "$OUTPUT_DIR/vegeta_report.txt"
fi
