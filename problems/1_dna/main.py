from collections import Counter
from core.input import read_input

INPUT_FILENAME = "rosalind_dna.txt"


def solve(input: str) -> dict[str, int]:
    counts = Counter(input)

    return counts


if __name__ == "__main__":
    input = read_input(INPUT_FILENAME)

    result = solve(input=input)

    print(result)
