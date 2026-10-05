#!/bin/sh
set -eu
: "${MODELS_DIR:?Set MODELS_DIR to the absolute directory containing classic, community-64 and community-128}"
case "$MODELS_DIR" in /*) ;; *) echo 'MODELS_DIR must be absolute' >&2; exit 1;; esac
exec docker run --detach --name "${CONTAINER_NAME:-kroko-stt-german-wyoming-local}" \
 --platform linux/amd64 --restart=no --read-only --cap-drop=ALL \
 --security-opt=no-new-privileges --stop-timeout=25 \
 --publish "${BIND_ADDRESS:-127.0.0.1}:${HOST_PORT:-10321}:10321" \
 --mount "type=bind,src=$MODELS_DIR,dst=/models,readonly" \
 --env "KROKO_MODEL=${KROKO_MODEL:-classic}" --env NUM_THREADS=1 \
 "${IMAGE:-kroko-stt-german-wyoming:local}"
