from typing import Generator

from core.input import read_input_lines

INPUT_FILENAME = "rosalind_gc.txt"


def get_string_id(line: str) -> str:
    """
    We strip the '>' symbol
    """
    return line[1:].strip()


def string_generator(
    input: Generator[str, None, None]
) -> Generator[tuple[str, str], None, None]:
    """
    Returns string id as well as string itself
    in one Python string.

    Since the strings are of length 1kbp max each,
    we can just concatenate lines of one string into one string.
    """
    first = True
    string_id = ""
    string_list = []
    for line in input:
        if line.startswith(">"):
            if first:
                string_id = get_string_id(line=line)
                first = False
            else:
                yield string_id, "".join(string_list)
                string_id = get_string_id(line=line)
                string_list = []
        else:
            string_list.append(line.strip())
    yield string_id, "".join(string_list)


def get_gc_content(string: str) -> float:
    """
    string_list is a list of strings where each element
    corresponds to one line in the input file.
    """
    gc_count = 0
    total_count = 0
    for char in string:
        if char in ["G", "C"]:
            gc_count += 1
        total_count += 1
    return gc_count / total_count * 100


def solve(input: Generator[str, None, None]) -> tuple[str, float]:
    max_gc_content = 0
    max_gc_content_id = ""
    for string_id, string in string_generator(input=input):
        gc_content = get_gc_content(string=string)

        if gc_content > max_gc_content:
            max_gc_content = gc_content
            max_gc_content_id = string_id

    return max_gc_content_id, max_gc_content


if __name__ == "__main__":
    input = read_input_lines(INPUT_FILENAME)

    result = solve(input=input)

    print(result)
