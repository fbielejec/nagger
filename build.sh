#!/bin/bash

#--- ARGS
NAME=nagger
IMAGE=fbielejec/$NAME

#--- FUNCTIONS

function build {
  {
    TAG=$(git log -1 --pretty=%h)
    IMG=$IMAGE:$TAG

    echo "============================================="
    echo  "Buidling: "$IMG""
    echo "============================================="

    cargo build --release
    docker build -t $IMG -f Dockerfile .
    docker tag $IMG $IMAGE:latest

  } || {
    echo "EXCEPTION WHEN BUIDLING "$IMG""
    exit 1
  }

}

function push {
  echo "Pushing: " $IMAGE
  docker push $IMAGE
}

function login {
  echo "$DOCKER_PASSWORD" | docker login -u "$DOCKER_USERNAME" --password-stdin
}

#--- EXECUTE

NAME=$1

login
build $NAME
push $NAME

exit $?
