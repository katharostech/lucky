#/bin/bash
../lucky daemon start --ignore-already-running && \
../lucky daemon trigger-hook {hook_name}