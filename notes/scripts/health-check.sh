#!/bin/bash

# Health check script for the Note API
# This script is used by Docker to verify the application is running correctly

set -e

# Configuration
HOST=${HEALTH_CHECK_HOST:-localhost}
PORT=${HEALTH_CHECK_PORT:-8000}
ENDPOINT=${HEALTH_CHECK_ENDPOINT:-/health}
TIMEOUT=${HEALTH_CHECK_TIMEOUT:-5}
RETRIES=${HEALTH_CHECK_RETRIES:-3}
SLEEP=${HEALTH_CHECK_SLEEP:-2}

attempt=1
while [ $attempt -le $RETRIES ]; do
    echo "Health check attempt $attempt: http://$HOST:$PORT$ENDPOINT"
    http_response=$(curl -s -w "HTTPSTATUS:%{http_code}" --max-time "$TIMEOUT" "http://$HOST:$PORT$ENDPOINT" || echo "FAILED")
    body=$(echo "$http_response" | sed -e 's/HTTPSTATUS\:.*//g')
    status=$(echo "$http_response" | tr -d '\n' | sed -e 's/.*HTTPSTATUS://')
    if [ "$body" = "OK" ] && [ "$status" = "200" ]; then
        echo "Health check passed: $body (HTTP $status)"
        exit 0
    else
        echo "Health check failed: HTTP $status, body: $body"
        if [ $attempt -lt $RETRIES ]; then
            echo "Retrying in $SLEEP seconds..."
            sleep $SLEEP
        fi
    fi
    attempt=$((attempt+1))
done

echo "Health check failed after $RETRIES attempts."
echo "Environment diagnostics:"
echo "  HOST=$HOST"
echo "  PORT=$PORT"
echo "  ENDPOINT=$ENDPOINT"
echo "  DATABASE_URL (redacted)=$(echo $DATABASE_URL | sed -E 's#(://[^:]+):[^@]+@#\1:***@#')"
exit 1
fi