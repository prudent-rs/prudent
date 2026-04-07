#!/usr/bin/sh

# violations_coverage/verify_error_messages uses `trybuild` crate`, which uses Serde (proc macros),
# which needs to be run as a dynamic library, so on Alpine Linux it needs musl-dev
apk add --no-cache musl-dev
