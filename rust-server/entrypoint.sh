#!/bin/bash

# * Fix permissions for mounted volume
chown -R appuser:appuser /sclera_builds

# * Run the app
exec "$@"
