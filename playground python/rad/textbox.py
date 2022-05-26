from time import sleep


def run(inp, delaySeconds):
    for i in inp:
        print(i, end='')
        sleep(delaySeconds)
    print()