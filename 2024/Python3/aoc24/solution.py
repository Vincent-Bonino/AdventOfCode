"""Base class for Solution."""

from abc import ABC, abstractmethod
from pathlib import Path
from typing import Any, ClassVar

DATA_PATH: Path = Path("./data")

INPUT_DATA_PATH: Path = DATA_PATH / "inputs"
TESTS_DATA_PATH: Path = DATA_PATH / "tests"


class Solution(ABC):
    """Base class for a solution.

    ClassAttributes (Solution)
    ---------------
    _solutions : Dict[int, Type[Solution]]
        Day number - solution class mapping.

    Attributes
    ----------
    file : Path
        Path to the file containing the input data.
    day : ClassVar[int]
        Number of the day. (1-25)
    test_name : ClassVar[str | None]
        Extra value to add to test file's name.
        Useful in case of several test files.

    """

    _solutions: ClassVar[dict[int, type['Solution']]] = {}

    file: Path

    day: ClassVar[int]
    test_name: ClassVar[str | None] = None

    @abstractmethod
    def solve_part_one(self) -> int:
        """Solution for part one."""

    @abstractmethod
    def solve_part_two(self) -> int:
        """Solution for part two."""

    def __init_subclass__(cls, *args: Any, **kwargs: Any) -> None:
        """Store subclasses."""
        super().__init_subclass__(**kwargs)
        cls._solutions[cls.day] = cls

    def __init__(self, is_test: bool) -> None:
        """Initialize a new Solution object.

        Raises
        ------
        FileNotFoundError
            Raised then target file does not exist.

        """
        self.file: Path

        if is_test is False:
            self.file = INPUT_DATA_PATH / f"day{self.day:02d}.txt"
        else:
            name_extra: str = "" if self.test_name is None else f"-{self.test_name}"
            self.file = TESTS_DATA_PATH / f"day{self.day:02d}{name_extra}.txt"

        if not self.file.is_file():
            raise FileNotFoundError(f"File {self.file} does not exist yet")

    @classmethod
    def get_from_day(cls, day: int) -> type['Solution']:
        """Get the day's solution class."""
        return cls._solutions[day]
