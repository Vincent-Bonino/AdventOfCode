"""Module containing the parsing of the input data."""

from pathlib import Path


def parse_input_file(path: Path) -> tuple[list[int], list[int]]:
    """Parse the input file in two lists of integers."""
    list1: list[int] = []
    list2: list[int] = []

    with path.open("r", encoding="utf-8") as f_input:
        for line in f_input:
            val1, _, val2 = line.strip().partition(" ")
            list1.append(int(val1))
            list2.append(int(val2))

    return list1, list2
