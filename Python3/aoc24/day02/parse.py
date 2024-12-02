"""Module containing the parsing of the input data."""

from pathlib import Path


def parse_input_file(path: Path) -> list[list[int]]:
    """Parse the input file in two lists of integers."""
    result: list[list[int]] = []

    with path.open("r", encoding="utf-8") as f_input:
        for line in f_input:
            result.append([int(val) for val in line.strip().split(" ")])

    return result
