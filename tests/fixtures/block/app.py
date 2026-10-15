import os


def load(path):
    with open(path) as f:
        data = f.read()

    return data.strip()


def main():
    print(load("x"))
