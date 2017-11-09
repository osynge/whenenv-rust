#!/bin/sh

set -x
(git rev-parse HEAD ; git describe) 2> /dev/null > src/githash
if [ ! -d "config" ] ; then
mkdir config
fi
if [ ! -d "m4" ] ; then
mkdir m4
fi
aclocal -I config
#autoheader
libtoolize --automake
touch AUTHORS
touch ChangeLog
touch NEWS
touch README
touch COPYING
automake --add-missing --copy
autoconf

