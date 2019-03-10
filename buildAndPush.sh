#!/bin/bash

# This script will build the docker image and push it to DockerHub.
# Usage: buildAndPush.sh imageName

# Dockerhub image names look like "username/appname" and must be all lower case.
# For example, "janesmith/calculator"

IMAGE_NAME=$1
echo "Using $IMAGE_NAME as the image name"
docker build -t $IMAGE_NAME .
if [ $? -ne 0 ]; then
    echo "Docker build failed"
fi
docker push $IMAGE_NAME
if [ $? -ne 0 ]; then
    echo "Docker push failed"
fi
