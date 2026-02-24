#!/bin/bash
set -e

BASE_URL="${BASE_URL:-http://localhost:3000}"
STEPS="${STEPS:-5}"
STEP_DURATION="${STEP_DURATION:-10s}"
MAX_RATE="${MAX_RATE:-10000}"
OUTPUT_DIR="${OUTPUT_DIR:-./benchmark_results/stress}"

mkdir -p "$OUTPUT_DIR"

echo "=== URLSlim Stress Test ==="
echo "Target: $BASE_URL"
echo "Steps: $STEPS x $STEP_DURATION"
echo "Max Rate: $MAX_RATE req/s"
echo ""

curl -sf "$BASE_URL/" > /dev/null || { echo "Server not responding"; exit 1; }

TEST_CODE=$(curl -s -X POST "$BASE_URL/api/v1/links" \
    -H "Content-Type: application/json" \
    -d '{"url":"https://example.com/stress-test"}' | jq -r '.code")

echo "Test link: $TEST_CODE"
echo ""

if ! command -v vegeta &> /dev/null; then
    echo "Error: vegeta is required for stress test"
    echo "Install with: brew install vegeta"
    exit 1
fi

echo "| Rate (req/s) | Latency P50 | P99 | Success |"
echo "|--------------|-------------|-----|---------|"

for ((i=1; i<=STEPS; i++)); do
    RATE=$((MAX_RATE * i / STEPS))
    
    RESULT=$(echo "GET $BASE_URL/$TEST_CODE" | \
        vegeta attack -duration="$STEP_DURATION" -rate="$RATE" | \
        vegeta report -type=json)
    
    P50=$(echo "$RESULT" | jq -r '.latencies.P50 / 1000000')
    P99=$(echo "$RESULT" | jq -r '.latencies.P99 / 1000000')
    SUCCESS=$(echo "$RESULT" | jq -r '.success_ratio * 100')
    
    printf "| %12d | %10.2fms | %5.2fms | %6.1f%% |\n" \
        "$RATE" "$P50" "$P99" "$SUCCESS"
    
    echo "$RESULT" > "$OUTPUT_DIR/stress_${RATE}.json"
    
    sleep 2
done

echo ""
echo "Full results saved to $OUTPUT_DIR/"
