#!/bin/bash
#
# This script will build the docker image and push it to dockerhub.
#
# Usage: buildAndPush.sh imageName
#
# Dockerhub image names look like "username/appname" and must be all lower case.
# For example, "janesmith/calculator"

IMAGE_NAME=$1
echo "Using $IMAGE_NAME as the image name"

# Make the docker image
docker build -t $IMAGE_NAME .
if [ $? -ne 0 ]; then
    echo "Docker build failed"
#    exit
fi
docker push $IMAGE_NAME
if [ $? -ne 0 ]; then
    echo "Docker push failed"
#    exit
fi



#### Steps
### Rust
# Just save the updated src and run the docker build

### Docker
# cd ./cargo-project-directory
# docker build -t vmpereiraf/rust-serverless-skel:v0.1 .
# docker push vmpereiraf/rust-serverless-skel:v0.1

### OpenWhisk CLI
# ibmcloud fn action create <action_name> --docker vmpereiraf/rust-serverless-skel ### to create new action
# ibmcloud fn action invoke --result <action_name> --param <json_formatted_parameters>
# ibmcloud fn action update <action_name> --docker vmpereiraf/rust-serverless-skel ### to update existing action
