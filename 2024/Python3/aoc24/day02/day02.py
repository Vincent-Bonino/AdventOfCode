"""Solution of day02."""

from typing import Any, ClassVar

from aoc24.solution import Solution

from .parse import parse_input_file
from .solve import solve_part_one, solve_part_two


class Day02(Solution):
    """Solution of day02."""

    day: ClassVar[int] = 2

    reports: list[list[int]]

    def __init__(self, *args: Any, **kwargs: Any) -> None:
        """Initialize a new instance."""
        super().__init__(*args, **kwargs)

        self.reports = parse_input_file(self.file)

    def solve_part_one(self) -> int:
        """Solution for part one."""
        return solve_part_one(self.reports)

    def solve_part_two(self) -> int:
        """Solution for part two."""
        return solve_part_two(self.reports)
