import json
import sys


def main():
    users = json.load(sys.stdin)
    admins = [u for u in users if u["role"] == "admin"]
    print(f"{len(admins)} admins of {len(users)} users")


main()
