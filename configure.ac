AC_PREREQ([2.60])

AC_INIT([whenenv], [0.0.1])
AM_INIT_AUTOMAKE([1.6 foreign subdir-objects])
m4_ifdef([AM_SILENT_RULES], [
    AM_SILENT_RULES([yes])
])

AC_CANONICAL_HOST

AC_PROG_CC_C99
AM_PROG_CC_C_O

AC_PATH_PROG([CARGO], [cargo], [notfound])
AS_IF([test "$CARGO" = "notfound"], [AC_MSG_ERROR([cargo is required])])

AC_PATH_PROG([RUSTC], [rustc], [notfound])
AS_IF([test "$RUSTC" = "notfound"], [AC_MSG_ERROR([rustc is required])])

dnl =============
dnl Check RPM dir
dnl =============
RPMD=`pwd`/rpm
AC_MSG_CHECKING([for directory where to create rpms])
AC_ARG_WITH(rpm-dir,[  --with-rpm-dir=<dir> Default is \$RPMD/],
            rpm_dir="$withval", rpm_dir="$RPMD")
if test -d "$rpm_dir" ; then
    AC_MSG_RESULT([found $rpm_dir])
    RPM_DIR=$rpm_dir
else
    AC_MSG_RESULT([no such directory $rpm_dir])
    AC_MSG_RESULT([the directory $rpm_dir will be created for you if possible])
    RPM_DIR=$rpm_dir
fi

AC_SUBST(RPM_DIR)



dnl Specify --enable-debug to make a development release.  By default,
dnl we build in public release mode.

AC_ARG_ENABLE(debug,
              AC_HELP_STRING([--enable-debug],
                             [Build Rust code with debugging information [default=no]]),
              [debug_release=$enableval],
              [debug_release=no])

AC_MSG_CHECKING(whether to build Rust code with debugging information)
if test "x$debug_release" = "xyes" ; then
    AC_MSG_RESULT(yes)
    CARGO_TARGET_DIR=debug
    CARGO_RELEASE_ARGS=""
    SPEC_CONFIG="--enable-debug"
else
    AC_MSG_RESULT(no)
    CARGO_TARGET_DIR=release
    CARGO_RELEASE_ARGS="--release"
    SPEC_CONFIG=""
fi
AM_CONDITIONAL([DEBUG_RELEASE], [test "x$debug_release" = "xyes"])

AC_SUBST([CARGO_RELEASE_ARGS])
AC_SUBST([SPEC_CONFIG])


SPEC_PREFIX="/usr"
AC_SUBST(SPEC_PREFIX)


AC_SUBST(PACKAGE_TARNAME)


LT_INIT



AC_SUBST(CARGO_TARGET_DIR)

AC_SUBST(VERSION)

AC_SUBST(ac_aux_dir)
AC_CONFIG_MACRO_DIRS([m4])

AC_CONFIG_FILES([
  Makefile
])

AC_CONFIG_FILES([
  whenenv.spec
])


AC_CONFIG_FILES([
  src/Makefile
])
AC_CONFIG_FILES([
  src/rust/Makefile
])
AC_CONFIG_FILES([
  src/rust/Cargo.toml
])
AC_OUTPUT
