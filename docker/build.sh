#! /usr/bin/env bash
# build crust sworker docker image

usage() {
    echo "Usage:"
        echo "    $0 -h                      Display this help message."
        echo "    $0 [options]"
    echo "Options:"
    echo "     -p publish image"
    echo "     -m build mode(dev or prod)"

      exit 1;
}

PUBLISH=0

while getopts ":hpm:" opt; do
    case ${opt} in
        h)
            usage
            ;;
        p)
            PUBLISH=1
            ;;
        m)
            SWORKER_MODE=$OPTARG
            ;;
        ?)
            echo "Invalid Option: -$OPTARG" 1>&2
            exit 1
            ;;
    esac
done

basedir=$(cd `dirname $0`; pwd)
instdir=$basedir/..
VER=$(cat VERSION | head -n 1)
IMAGEID="crustio/crust-sworker-teaclave:$VER"

echo "building crust sworker runner image $IMAGEID"
if [ "$PUBLISH" -eq "1" ]; then
    echo "will publish after build"
fi

cd $instdir
if [ x"$SWORKER_MODE" != x"prod" ]; then
    SWORKER_MODE="dev"
fi

docker build -f docker/runner/Dockerfile -t $IMAGEID .
if [ "$?" -ne "0" ]; then
    echo "crust-sworker build failed!"
    exit 1
fi

echo "crustio/crust-sworker-teaclave build success"
if [ "$PUBLISH" -eq "1" ]; then
    echo "will publish image to $IMAGEID"
    docker push $IMAGEID
fi
