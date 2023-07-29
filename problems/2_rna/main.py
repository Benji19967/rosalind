from core.input import read_input

INPUT_FILENAME = "rosalind_rna.txt"


def solve(input: str) -> str:
    transcribed_string_list = []

    for c in input:
        if c == "T":
            transcribed_string_list.append("U")
        else:
            transcribed_string_list.append(c)

    transcribed_string = "".join(transcribed_string_list)

    return transcribed_string


if __name__ == "__main__":
    input = read_input(INPUT_FILENAME)

    result = solve(input=input)

    print(result)
