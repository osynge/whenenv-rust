#!/bin/sh

set -x
(git rev-parse HEAD ; git describe) 2> /dev/null > src/githash
aclocal -I config
#autoheader
libtoolize --automake
touch AUTHORS ChangeLog
automake --add-missing --copy
autoconf

