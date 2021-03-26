#!/bin/bash
basedir=$(cd `dirname $0`;pwd)
instdir=$basedir/..
srcdir=$instdir/src
crustdir=/opt/crust
sworkerdir=$crustdir/crust-sworker
uid=$(stat -c '%U' $instdir)

cd $instdir
tryout=30
while true; do
    make
    if [ $? -eq 0 ]; then
        break
    fi
    ((tryout--))
    if [ $tryout -eq 0 ]; then
        exit 1
    fi
done

mkdir -p $sworkerdir
if [ $? -ne 0 ]; then
    echo "[ERROR] Create directory $sworkerdir failed!"
    exit 1
fi
cd $sworkerdir
echo "[INFO] Remove old version"
rm -rf *
mkdir bin
mkdir etc
cp $srcdir/crust-sworker-t ./bin
cp $srcdir/enclave.signed.so ./etc
cp $instdir/VERSION ./

chown -R $uid:$uid $sworkerdir
if [ $? -ne 0 ]; then
    echo "[ERROR] Change $sworkerdir owner to $uid failed!"
    exit 1
fi

echo "[INFO] Install crust-sworker-teaclave successfully!"
