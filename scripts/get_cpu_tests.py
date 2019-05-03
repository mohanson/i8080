import contextlib
import os.path

import requests

prefix = 'http://altairclone.com/downloads/cpu_tests/'
suffix = [
    '+README.TXT',
    '8080EXER.COM',
    '8080EXER.MAC',
    '8080EXER.PNG',
    '8080EXER.PRN',
    '8080EXM.COM',
    '8080EXM.MAC',
    '8080EXM.PRN',
    '8080PRE.COM',
    '8080PRE.MAC',
    '8080PRE.PRN',
    '8080_8085 CPU Exerciser.pdf',
    'CPUTEST.COM',
    'TST8080.ASM',
    'TST8080.COM',
    'TST8080.PRN',
]
saveto = './res/cpu_tests'


def wget(url: str, dst: str):
    resp = requests.get(url, stream=True)
    with open(dst, 'wb') as f:
        with contextlib.closing(resp) as r:
            for data in r.iter_content(chunk_size=8 * 1024):
                f.write(data)


def main():
    if not os.path.exists(saveto):
        print(f'Make dir {saveto}')
        os.makedirs(saveto, 0o755)
    for e in suffix:
        src = prefix + e
        dst = os.path.join(saveto, e)
        print(f'Get {src}')
        wget(src, dst)


if __name__ == '__main__':
    main()
