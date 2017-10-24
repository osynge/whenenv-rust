#!/bin/sh

set -x
aclocal -I config
#autoheader
libtoolize --automake
touch AUTHORS ChangeLog
automake --add-missing --copy
autoconf

