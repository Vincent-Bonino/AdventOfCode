"""Module containing computation of solutions."""

MIN_DELTA: int = 1
MAX_DELTA: int = 3


def solve_part_one(reports: list[list[int]]) -> int:
    """Compute solution of part one."""
    return sum(is_safe(report) for report in reports)


def is_safe(report: list[int]) -> bool:
    """Determine if a report is safe."""
    is_increasing: bool = report[1] >= report[0]

    for index in range(1, len(report)):
        variation: bool = report[index] >= report[index - 1]
        if variation != is_increasing:
            return False

        delta: int = abs(report[index] - report[index - 1])
        if not (MIN_DELTA <= delta <= MAX_DELTA):
            return False

    return True


def solve_part_two(reports: list[list[int]]) -> int:
    """Compute solution of part two."""
    return sum(is_safe(report) or is_pseudo_safe(report) for report in reports)


def is_pseudo_safe(report: list[int]) -> bool:
    """Determine if a report is pseudo-safe.

    I.e. if the report is safe with an element removed.

    """
    for index in range(len(report)):
        subreport: list[int] = list(report)  # Clone
        subreport.pop(index)
        if is_safe(subreport):
            return True

    return False
