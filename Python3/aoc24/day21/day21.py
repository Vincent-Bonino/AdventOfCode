"""Example solution."""

from typing import Any, ClassVar

from aoc24.solution import Solution

from .solve import solve_part_one, solve_part_two


class Day21(Solution):
    """Example solution."""

    day: ClassVar[int] = 21
    test_name: ClassVar[str | None] = None

    codes: list[str]

    def __init__(self, *args: Any, **kwargs: Any) -> None:
        """Initialize a new instance."""
        super().__init__(*args, **kwargs)

        self.codes = list(map(str.strip, self.file.open('r', encoding="utf-8").readlines()))

        print(self.codes)

    def solve_part_one(self) -> int:
        """Solution for part one."""
        return solve_part_one(self.codes)

    def solve_part_two(self) -> int:
        """Solution for part two."""
        return solve_part_two(self.codes)
