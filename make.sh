#! /bin/bash


out="main"

usage() {
    cat << EOF
Compile your Rust script.

Usage:
    $(basename "${BASH_SOURCE[0]}") [OPTIONS] [SUBCOMMAND]

OPTIONS:

    -o, --out       Name output file
    -h, --help      Print this help and exit
EOF
    exit
}

while [ "$1" != "" ]; do
    case $1 in
        -o | --out)
            shift
            out=$1
            ;;
        -h | --help)
            usage
            exit 0
            ;;
        * )
            usage
            exit 0
            ;;
    esac
    shift
done

rustc src/main.rs -o $out

exit 0