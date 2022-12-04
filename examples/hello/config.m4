dnl Copyright (c) 2022 PHPER Framework Team
dnl PHPER is licensed under Mulan PSL v2.
dnl You can use this software according to the terms and conditions of the Mulan
dnl PSL v2. You may obtain a copy of Mulan PSL v2 at:
dnl          http://license.coscl.org.cn/MulanPSL2
dnl THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
dnl KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
dnl NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
dnl See the Mulan PSL v2 for more details.

PHP_ARG_ENABLE([hello],
  [whether to enable hello support],
  [AS_HELP_STRING([--enable-hello],
    [Enable hello support])],
  [no])

dnl If not enable, `cargo build` run with argument `--release`.
PHP_ARG_ENABLE([cargo_debug], [whether to enable cargo debug mode],
[  --enable-cargo-debug           Enable cargo debug], no, no)

if test "$PHP_hello" != "no"; then
  dnl Check cargo command exists or not.
  AC_PATH_PROG(CARGO, cargo, no)
  if ! test -x "$CARGO"; then
    AC_MSG_ERROR([cargo command missing, please reinstall the cargo distribution])
  fi

  AC_DEFINE(HAVE_hello, 1, [ Have hello support ])

  PHP_NEW_EXTENSION(hello, [ ], $ext_shared)

  CARGO_MODE_FLAGS="--release"
  CARGO_MODE_DIR="release"

  if test "$PHP_CARGO_DEBUG" != "no"; then
    CARGO_MODE_FLAGS=""
    CARGO_MODE_DIR="debug"
  fi

  cat >>Makefile.objects<< EOF
all: cargo_build

clean: cargo_clean

cargo_build:
    # Build the extension file
	PHP_CONFIG=$PHP_PHP_CONFIG cargo build $CARGO_MODE_FLAGS

    # Copy the extension file from target dir to modules
	if [[ -f ./target/$CARGO_MODE_DIR/libhello.dylib ]] ; then \\
		cp ./target/$CARGO_MODE_DIR/libhello.dylib ./modules/hello.so ; fi
	if [[ -f ./target/$CARGO_MODE_DIR/libhello.so ]] ; then \\
		cp ./target/$CARGO_MODE_DIR/libhello.so ./modules/hello.so ; fi

cargo_clean:
	cargo clean

.PHONY: cargo_build cargo_clean
EOF

  dnl Symbolic link the files for `cargo build`
  AC_CONFIG_LINKS([ \
    Cargo.lock:Cargo.lock \
    Cargo.toml:Cargo.toml \
    build.rs:build.rs \
    src:src \
    ])
fi
