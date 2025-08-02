import json
from pathlib import Path

import requests


class Endpoint:
    def __init__(self, url):
        self.url = url


def get_url() -> str:
    file = Path("../endpoint.json")
    with file.open("r", encoding="utf-8") as file:
        content = json.loads(file.read())
        endpoint = Endpoint(**content)
        return endpoint.url


def fetch(url: str) -> str:
    response = requests.get(url=url)
    response.raise_for_status()

    return str(response.text)


def write_file(content: str) -> None:
    file = Path("response.json")
    with file.open("a", encoding="utf-8") as file:
        file.write(content + "\n")


if __name__ == "__main__":
    url = get_url()
    content = fetch(url)
    write_file(content)
