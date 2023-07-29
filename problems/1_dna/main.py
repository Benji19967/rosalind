from collections import Counter
from core.input import read_input

INPUT_FILENAME = "rosalind_dna.txt"

if __name__ == "__main__":
    input = read_input(INPUT_FILENAME)

    counts = Counter(input)

    print(counts)
