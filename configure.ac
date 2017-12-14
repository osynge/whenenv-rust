AC_PREREQ([2.60])

AC_INIT([whenenv], [0.0.5])
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

dnl Set PACKAGE PREFIX
if test "x${prefix}" = "xNONE"; then
  packageprefix=${ac_default_prefix}
else
  packageprefix=${prefix}
fi

packagedatadir=share/${PACKAGE}

dnl Subst PACKAGE_DATA_DIR.
NO_PREFIX_PACKAGE_DATA_DIR="${packagedatadir}"
AC_SUBST(NO_PREFIX_PACKAGE_DATA_DIR)
PACKAGE_DATA_DIR="${packagedatadir}"
AC_SUBST(PACKAGE_DATA_DIR)


GIT_HASH=(git rev-parse HEAD)
AC_MSG_CHECKING([for git hash])
AC_ARG_WITH(git-hash,[  --with-git-hash=<hash> Default is \$GITD/],
            git_hash="$withval", git_hash="$GITD")
if test -f "src/githash" ; then
    git_hash=`cat src/githash`
    AC_MSG_RESULT([found $git_hash])
    GIT_HASH=$git_hash
else
    git_hash=`git rev-parse HEAD`
    AC_MSG_RESULT([no such hash $git_hash])
    AC_MSG_RESULT([the hash $git_hash will be created for you if possible])
fi

AC_SUBST(GIT_HASH)


GIT_TAG_HASH=`git rev-list --tags --max-count=1`
if test "$GIT_TAG_HASH" = "$GIT_HASH" ; then
VERSION_DETAIL=""
else
GIT_BRANCH=`git rev-parse --abbrev-ref HEAD`
VERSION_DETAIL=_rc_${GIT_BRANCH}_`date +%s`
fi
AC_SUBST(VERSION_DETAIL)


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



dnl =============
dnl Check jobs dir
dnl =============
JOBD=${packageprefix}/$PACKAGE_DATA_DIR/job
AC_MSG_CHECKING([for directory where to create jobs])
AC_ARG_WITH(job-dir,[  --with-job-dir=<dir> Default is \$JOBD/],
            job_dir="$withval", job_dir="$JOBD")
if test -d "$job_dir" ; then
    AC_MSG_RESULT([found $job_dir])
    jobdir=$job_dir
else
    AC_MSG_RESULT([no such directory $job_dir])
    AC_MSG_RESULT([the directory $job_dir will be created for you if possible])
    jobdir=$job_dir
fi
AC_SUBST(jobdir)

dnl =============
dnl Check shells dir
dnl =============
SHELLD=${packageprefix}/$PACKAGE_DATA_DIR/shell
AC_MSG_CHECKING([for directory where to create shells])
AC_ARG_WITH(shell-dir,[  --with-shell-dir=<dir> Default is \$SHELLD/],
            shell_dir="$withval", shell_dir="$SHELLD")
if test -d "$shell_dir" ; then
    AC_MSG_RESULT([found $shell_dir])
    shelldir=$shell_dir
else
    AC_MSG_RESULT([no such directory $shell_dir])
    AC_MSG_RESULT([the directory $shell_dir will be created for you if possible])
    shelldir=$shell_dir
fi
AC_SUBST(shelldir)

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
  whenenv.spec
  src/rust/Cargo.toml
  src/rust/src/autoconf.rs
])


AC_CONFIG_FILES([
  Makefile
  src/Makefile
  src/rust/Makefile
  src/data/Makefile
  src/data/ad404baf/Makefile
])
AC_OUTPUT

