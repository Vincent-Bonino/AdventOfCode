"""Module containing computation of solutions."""

from .instruction import Do, Dont, Instruction, Mul


def solve_part_one(instructions: list[Instruction]) -> int:
    """Compute solution of part one."""
    return sum(instr.left * instr.right for instr in instructions if isinstance(instr, Mul))


def solve_part_two(instructions: list[Instruction]) -> int:
    """Compute solution of part two."""
    result: int = 0
    enabled: bool = True

    for instr in instructions:
        match instr:
            case Do():
                enabled = True
            case Dont():
                enabled = False
            case Mul(left=left, right=right):
                if enabled:
                    result += left * right

    return result
