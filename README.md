# ips-patch

Command-line tool to apply IPS patches.

## Synopsis

To build and install:

    $ cargo install --git https://github.com/cmbrandenburg/ips-patch.git

To run:

    $ ips-patch example.ips <in_file >out_file

The `ips-patch` program accepts one parameter: the name of the IPS patch
file to apply. The program then reads the data-to-patch from `stdin` and
writes the patched data to `stdout`.

## More info

The IPS file format is used to patch binaries files, usually ROMs.

The `ips-patch` program follows the IPS specification described here:
http://www.zerosoft.zophar.net/ips.php.
