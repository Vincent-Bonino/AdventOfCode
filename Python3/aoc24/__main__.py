"""Project entry-point."""

from argparse import ArgumentParser, Namespace

from aoc24.solution import Solution

# Solutions
import aoc24.day00
import aoc24.day01
import aoc24.day02
import aoc24.day03


def main() -> None:
    """Project entry-point function."""
    args: Namespace = build_cli_arguments_parser().parse_args()

    solution: Solution = Solution.get_from_day(args.day)(args.test)

    # Part 1
    print("Part 1:", solution.solve_part_one())

    # Part 2
    print("Part 2:", solution.solve_part_two())


def build_cli_arguments_parser() -> ArgumentParser:
    """Build the CLI arguments parser."""
    parser: ArgumentParser = ArgumentParser(
        prog="aoc24",
        description="Advent of Code 2024 solutions",
    )
    parser.add_argument("day", type=int, help="Day to run")
    parser.add_argument("--test", action="store_true", help="Toggle to run on test input")

    return parser


if __name__ == '__main__':
    main()
