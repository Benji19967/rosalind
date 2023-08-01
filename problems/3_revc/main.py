from core.input import read_input

INPUT_FILENAME = "rosalind_revc.txt"

COMPLEMENTS = {
    "A": "T",
    "T": "A",
    "C": "G",
    "G": "C",
}


def solve(input: str) -> str:
    reverse_complement_list = []

    for c in reversed(input):
        reverse_complement_list.append(COMPLEMENTS[c])

    reverse_complement = "".join(reverse_complement_list)

    return reverse_complement


if __name__ == "__main__":
    input = read_input(INPUT_FILENAME)

    result = solve(input=input)

    print(result)
