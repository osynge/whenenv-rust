#!/bin/sh

set -x
(git rev-parse HEAD ; git describe) 2> /dev/null > src/githash
if [ ! -d "config" ] ; then
mkdir config
fi
aclocal -I config
#autoheader
libtoolize --automake
touch AUTHORS
touch ChangeLog
automake --add-missing --copy
autoconf

