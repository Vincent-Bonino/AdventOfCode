"""Module containing computation of solutions."""

from collections.abc import Iterable
from functools import cache
from itertools import permutations, product

from .direction import Direction4, direction_from_arrow

KeyPad = dict[str, tuple[int, int]]

# Positions of tiles on the numeric keypad
NUMERIC_KEYPAD: KeyPad = {
    "A": (3, 2),
    "0": (3, 1),
    "1": (2, 0),
    "2": (2, 1),
    "3": (2, 2),
    "4": (1, 0),
    "5": (1, 1),
    "6": (1, 2),
    "7": (0, 0),
    "8": (0, 1),
    "9": (0, 2),
}

# Positions of tiles on the directional keypads
DIRECTIONAL_KEYPAD: KeyPad = {
    "A": (0, 2),
    "^": (0, 1),
    ">": (1, 2),
    "v": (1, 1),
    "<": (1, 0),
}


def get_code_int_value(code: str) -> int:
    """Get the integer value of a code."""
    return int(code[:3])


@cache
def generate_paths_on_keypad(from_symbol: str, to_symbol: str, is_dir_keypad: bool) -> set[str]:
    """Build all possible path (set of instruction) to move and type.

    Parameters
    ----------
    from_symbol : str
        Symbol starting at.
    to_symbol : str
        Symbol going to.
    is_dir_keypad : bool
        Whether to use directional or numeric keypad.

    Returns
    -------
    set[str]
        All possible instruction that type the symbol on the keypad.

    """
    keypad = DIRECTIONAL_KEYPAD if is_dir_keypad is True else NUMERIC_KEYPAD

    current_tile_coord: tuple[int, int] = keypad[from_symbol]

    # Move to the new tile
    new_tile_coord: tuple[int, int] = keypad[to_symbol]

    coord_diff: tuple[int, int] = (
        new_tile_coord[0] - current_tile_coord[0],
        new_tile_coord[1] - current_tile_coord[1],
    )

    moves_from_curr_to_new: str = ""

    # Add moves on X
    if coord_diff[0] > 0:
        moves_from_curr_to_new += "v" * coord_diff[0]
    elif coord_diff[0] < 0:
        moves_from_curr_to_new += "^" * abs(coord_diff[0])

    # Add moves on Y
    if coord_diff[1] > 0:
        moves_from_curr_to_new += ">" * coord_diff[1]
    elif coord_diff[1] < 0:
        moves_from_curr_to_new += "<" * abs(coord_diff[1])

    all_moves: set[str] = set("".join(x) for x in permutations(moves_from_curr_to_new))

    # Now filter out paths going on the empty tile
    valid_moves: set[str] = set()

    for moves in all_moves:
        current_pos: tuple[int, int] = current_tile_coord
        is_move_valid: bool = True

        for m in moves:
            direction: Direction4 = direction_from_arrow(m)
            current_pos = direction + current_pos

            if current_pos not in keypad.values():
                # We are in outside the keypad (on the empty tile)
                is_move_valid = False
                break

        if is_move_valid:
            valid_moves.add(f"{moves}A")  # Because we must finish on A to actually press the button

    # We have all valid moves
    return valid_moves


def best_path(paths: Iterable[str]) -> int:
    """Retur the best (shorted) path of the provided sequence."""
    return min(map(len, paths))


@cache
def compute_typing_complexity(from_symbol: str, to_symbol: str, is_dir_keypad: bool, depth: int) -> int:
    """Compute the cost of going from {from_symbol} to {to_symbol} (and pressing) it, {depths} keypads deep.

    Parameters
    ----------
    from_symbol : str
        Symbol starting at.
    to_symbol : str
        Symbol going to.
    is_dir_keypad : bool
        Whether to use directional or numeric keypad.
    depth : int
        Number of nested keypads.

    """
    # Recursion limit
    if depth == 0:
        return best_path(generate_paths_on_keypad(from_symbol, to_symbol, is_dir_keypad=True))

    # Compute recursively the result
    all_paths: set[str] = generate_paths_on_keypad(from_symbol, to_symbol, is_dir_keypad)
    best_complexity: int = 2 << 60  # Almost infity

    for path in all_paths:
        path = "A" + path  # Always start on A  # noqa: PLW2901 (loop var overwritten)

        current_complexity: int = 0
        for i in range(len(path) - 1):
            from_symb = path[i]
            to_symb = path[i + 1]
            current_complexity += compute_typing_complexity(from_symb, to_symb, is_dir_keypad=True, depth=depth - 1)

        best_complexity = min(best_complexity, current_complexity)

    return best_complexity


def complexity(code: str, nested_keypads: int) -> int:
    """Compute the whole complexity (code + typeing) of a code, with nested keypads."""
    integer_value: int = get_code_int_value(code)

    code = "A" + code  # Always start on A
    typing_complexity: int = 0
    for i in range(len(code) - 1):
        typing_complexity += compute_typing_complexity(code[i], code[i + 1], is_dir_keypad=False, depth=nested_keypads)

    return integer_value * typing_complexity


def solve_part_one(codes: list[str]) -> int:
    """Compute solution of part one."""
    return sum(complexity(code, 2) for code in codes)


def solve_part_two(codes: list[str]) -> int:
    """Compute solution of part two."""
    return sum(complexity(code, 25) for code in codes)
