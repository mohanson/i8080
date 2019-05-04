import subprocess
import sys


def call(command):
    print(command)
    r = subprocess.call(command, shell=True)
    if r != 0:
        sys.exit(r)


def main():
    call('cargo test test_rom -- --nocapture --test-threads 1')


if __name__ == '__main__':
    main()
