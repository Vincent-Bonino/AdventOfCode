"""Module containing the parsing of the input data."""

from pathlib import Path
from re import Pattern, compile

from .instruction import Do, Dont, Instruction, Mul

MUL_INSTRUCTION_REGEX: Pattern[str] = compile(r"mul\((\d+),(\d+)\)")
INSTRUCTION_REGEX: Pattern[str] = compile(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)")


def parse_input_file(path: Path) -> list[Instruction]:
    """Parse the input file in two lists of integers."""
    instructions: list[Instruction] = []

    for instr in INSTRUCTION_REGEX.finditer(path.read_text()):
        match instr.group(0):
            case "do()":
                instructions.append(Do())
            case "don't()":
                instructions.append(Dont())
            case _:
                instructions.append(Mul(int(instr.group(1)), int(instr.group(2))))

    return instructions
