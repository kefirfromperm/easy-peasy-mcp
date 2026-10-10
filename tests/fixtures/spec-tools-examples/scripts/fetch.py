# SPDX-License-Identifier: MIT
import json
import requests


def fetch(url):
    return requests.get(url, timeout=10).json()


if __name__ == "__main__":
    print(json.dumps(fetch("https://example.com/api/users")))
