"""Solution of day01."""

from typing import Any, ClassVar

from aoc24.solution import Solution

from .parse import parse_input_file
from .solve import solve_part_one, solve_part_two


class Day01(Solution):
    """Solution of day01."""

    day: ClassVar[int] = 1

    left_list: list[int]
    right_list: list[int]

    def __init__(self, *args: Any, **kwargs: Any) -> None:
        """Initialize a new instance."""
        super().__init__(*args, **kwargs)

        self.left_list, self.right_list = parse_input_file(self.file)

    def solve_part_one(self) -> int:
        """Solution for part one."""
        return solve_part_one(self.left_list, self.right_list)

    def solve_part_two(self) -> int:
        """Solution for part two."""
        return solve_part_two(self.left_list, self.right_list)
