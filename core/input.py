from pathlib import Path

ROSALIND_DIR = Path(__file__).parents[1]
INPUTS_DIR = "inputs"


def read_input(filename: str) -> str:
    path = ROSALIND_DIR / INPUTS_DIR / filename

    with open(path.resolve()) as f:
        input = f.read()
    return input
