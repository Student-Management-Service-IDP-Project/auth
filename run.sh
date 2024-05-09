#!/bin/bash

docker run -it -e MONGO_SERVER="mongodb" \
    -e MONGO_PORT=27017 \
    -e MONGO_USERNAME="school" \
    -e MONGO_PASSWORD="schoolpassword" \
    -e MONGO_DATABASE="school" \
    auth-service:0.3.0
