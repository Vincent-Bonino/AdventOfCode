"""Module containing computation of solutions."""


def solve_part_one(left_list: list[int], right_list: list[int]) -> int:
    """Compute solution of part one."""
    left_list.sort()
    right_list.sort()

    return sum(abs(left - right) for left, right in zip(left_list, right_list, strict=True))


def solve_part_two(left_list: list[int], right_list: list[int]) -> int:
    """Compute solution of part two."""
    return sum(left * right_list.count(left) for left in left_list)
