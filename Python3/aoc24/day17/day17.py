"""Solution of day17."""

from typing import Any, ClassVar

from aoc24.solution import Solution

from .program import Computer
from .solve import solve_part_two


class Day17(Solution):
    """Solution of day17."""

    day: ClassVar[int] = 17
    test_name: ClassVar[str | None] = "exen0r"

    a_register: int
    program: list[int]

    def __init__(self, *args: Any, **kwargs: Any) -> None:
        """Initialize a new instance."""
        super().__init__(*args, **kwargs)

        self._parse_input()

    def _parse_input(self) -> None:
        """Parse the input file."""
        register_line: str = self.file.open('r', encoding="utf-8").readlines()[0]
        self.a = int(register_line.removeprefix("Register A: ").strip())

        program_line: str = self.file.open('r', encoding="utf-8").readlines()[4]
        self.program = list(map(int, program_line.removeprefix("Program: ").strip().split(",")))

    def solve_part_one(self) -> int:
        """Solution for part one."""
        stdout: list[int] = Computer(self.program, self.a).run()
        print(",".join(map(str, stdout)))
        return 1234

    def solve_part_two(self) -> int:
        """Solution for part two."""
        return solve_part_two(self.program)
