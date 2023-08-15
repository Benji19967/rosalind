from pathlib import Path
from typing import Generator

ROSALIND_DIR = Path(__file__).parents[1]
INPUTS_DIR = "inputs"


def read_input(filename: str) -> str:
    path = ROSALIND_DIR / INPUTS_DIR / filename

    with open(path.resolve()) as f:
        input = f.read().strip()
    return input


def read_input_lines(filename: str) -> Generator[str, None, None]:
    path = ROSALIND_DIR / INPUTS_DIR / filename

    with open(path.resolve()) as f:
        yield from f
