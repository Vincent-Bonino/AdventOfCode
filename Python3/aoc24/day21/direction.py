"""Direction enum."""

from enum import Enum
from typing import Any


class Direction4(Enum):
    """Enumeration of all four cardinal directions.

    Values are the delta on (x, y) to move in this direction.

    """

    N = (-1, 0)
    E = (0, +1)
    S = (+1, 0)
    W = (0, -1)

    def __add__(self, other: Any) -> tuple[int, int]:
        """Add a direction with a tuple of integers."""
        if not isinstance(other, tuple):
            raise NotImplementedError

        return (
            self.value[0] + other[0],
            self.value[1] + other[1],
        )


def direction_from_arrow(arrow: str) -> Direction4:
    """Return the Direction4 associated with the given arrow symbol."""
    match arrow:
        case '^':
            return Direction4.N
        case '>':
            return Direction4.E
        case 'v':
            return Direction4.S
        case '<':
            return Direction4.W
        case _:
            raise ValueError
