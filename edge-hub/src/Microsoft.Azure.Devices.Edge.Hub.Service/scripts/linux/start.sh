#!/bin/sh

# This scrips starts the IoT hub service

# generate SSL self signed certificate
./scripts/linux/generate-cert.sh

# start service
exec dotnet Microsoft.Azure.Devices.Edge.Hub.Service.dll