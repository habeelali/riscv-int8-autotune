#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
python3 hardware/esp32c3-v5/audit.py \
 results/hardware-v5/physical-return/captures/boot-1 \
 results/hardware-v5/physical-return/captures/boot-2 \
 results/hardware-v5/physical-return/captures/boot-3 \
 --oracle hardware/esp32c3-v5/main/generated/oracle.json \
 --output results/hardware-v5/physical-audit
