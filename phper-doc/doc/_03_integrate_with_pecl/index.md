# Integrate with PECL

As can be seen from the `quick start` example, using phper to develop a PHP extension doesn't require the `phpize` and `make` processes, as you would when developing with C/C++.

However, if you intend to publish the extension on PECL (the official repository of PHP extensions), you will need to integrate the phper project with `phpize` and `make` since the PECL install command will call them.

This chapter will guide you on how to integrate the phper project with `pecl` and `phpize`.

## Steps

### Adapt to `phpize`

1. At first, imagine you have finished the hello project follow [write your first extension](_02_quick_start::_01_write_your_first_extension), can build the php extension `.so` file successfully.

1. And then, create the `config.m4` file using by `phpize` (In theory,
   `config.w32` is also required for compatibility with Windows, but now phper
   don't support Windows).

   ```autoconf
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

   ```

   Here we add the configure option `--enable-cargo-debug`, whether to enable
   cargo debug mode.

   *I think this isn't the best way of writing, because I am not very familiar*
   *with the syntax of `autoconf`, but it can work, if you have a better way of*
   *writing, welcome to PR. :-)*

1. Don't forget to add the `.gitignore`, because `phpize` and `./configure` will
   generate too much temporary files.

   ```git
   hello-*.tgz
   *.lo
   *.la
   .libs
   acinclude.m4
   aclocal.m4
   autom4te.cache
   build
   config.guess
   config.h
   config.h.in
   config.h.in~
   config.log
   config.nice
   config.status
   config.sub
   configure
   configure.ac
   configure.in
   include
   install-sh
   libtool
   ltmain.sh
   Makefile
   Makefile.fragments
   Makefile.global
   Makefile.objects
   missing
   mkinstalldirs
   modules
   php_test_results_*.txt
   phpt.*
   run-test-info.php
   run-tests.php
   tests/**/*.diff
   tests/**/*.out
   tests/**/*.exp
   tests/**/*.log
   tests/**/*.db
   tests/**/*.mem
   tmp-php.ini
   ```

Now `phpize` is ready to run, let's try!

```shell
phpize
./configure --enable-cargo-debug
make
make install
```

Alright, the hello extension will be installed in php extension directory.

### Adapt to PECL

1. To integrate with PECL, we have to create the `package.xml` file.

   ```xml
    <?xml version="1.0"?>
    
    <package version="2.0" 
    	xmlns="http://pear.php.net/dtd/package-2.0" 
    	xmlns:tasks="http://pear.php.net/dtd/tasks-1.0" 
    	xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://pear.php.net/dtd/tasks-1.0 http://pear.php.net/dtd/tasks-1.0.xsd http://pear.php.net/dtd/package-2.0 http://pear.php.net/dtd/package-2.0.xsd">
    	<name>hello</name>
    	<channel>pecl.php.net</channel>
    	<summary>Hello world example.</summary>
    	<description>The Hello world example of phper.</description>
    	<lead>
    		<name>jmjoy</name>
    		<user>jmjoy</user>
    		<email>jmjoy@apache.org</email>
    		<active>yes</active>
    	</lead>
    	<date>1970-01-01</date>
    	<version>
    		<release>0.0.0</release>
    		<api>0.0.0</api>
    	</version>
    	<stability>
    		<release>stable</release>
    		<api>stable</api>
    	</stability>
    	<license uri="http://license.coscl.org.cn/MulanPSL2/">MulanPSL-2.0</license>
    	<notes>        Release notes.	</notes>
    	<contents>
    		<dir name="/">
    			<file name="Cargo.lock" role="src" />
    			<file name="Cargo.toml" role="src" />
    			<file name="README.md" role="doc" />
    			<file name="build.rs" role="src" />
    			<file name="config.m4" role="src" />
    			<file name="src/lib.rs" role="src" />
    		</dir>
    	</contents>
    	<dependencies>
    		<required>
    			<php>
    				<min>7.2.0</min>
    			</php>
    			<pearinstaller>
    				<min>1.4.0</min>
    			</pearinstaller>
    		</required>
    	</dependencies>
    	<providesextension>hello</providesextension>
    	<extsrcrelease>
    		<configureoption default="no" name="enable-cargo-debug" prompt="enable cargo debug?" />
    	</extsrcrelease>
    </package>
   ```

   **The example [hello](https://github.com/phper-framework/phper/tree/master/examples/hello) in phper github repository not run `pecl` well, because
   it's the sub crate of phper, not a separate project.*

All is OK, try to build the `pecl` package and install it.

```shell
pecl package
pecl install hello-*.tgz
```

We will receive output like this at the end if the whole is successful.

```text
configuration option "php_ini" is not set to php.ini location
You should add "extension=hello.so" to php.ini
```
