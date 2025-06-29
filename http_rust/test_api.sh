#!/bin/bash

echo "=== Solana HTTP API Server Test ==="
echo

echo "1. Testing root endpoint:"
curl -s http://localhost:8080 | jq '.'
echo

echo "2. Testing health endpoint:"
curl -s http://localhost:8080/health | jq '.'
echo

echo "3. Testing balance endpoint (System Program):"
curl -s http://localhost:8080/balance/11111111111111111111111111111111 | jq '.'
echo

echo "4. Testing account info endpoint:"
curl -s http://localhost:8080/account/11111111111111111111111111111111 | jq '.'
echo

echo "5. Testing invalid address (should return error):"
curl -s http://localhost:8080/balance/invalid_address | jq '.'
echo

echo "=== All tests completed ===" 