#!/bin/bash
echo "🔐 Generating self-signed root CA for Overwatch..."
# Sample OpenSSL command (customize as needed)
openssl req -x509 -newkey rsa:4096 -sha256 -days 365 -nodes \
    -keyout overwatch_ca.key -out overwatch_ca.crt \
    -subj "/CN=Overwatch Local Interceptor" -addext "basicConstraints=CA:TRUE"
echo "✅ Done! CA cert: overwatch_ca.crt"
