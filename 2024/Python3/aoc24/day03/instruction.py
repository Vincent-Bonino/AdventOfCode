"""Module containing the definition of an instruction."""

from dataclasses import dataclass


class Instruction:
    """Base instruction class."""


class Do(Instruction):
    """Do instruction."""


class Dont(Instruction):
    """Don't instruction."""


@dataclass
class Mul(Instruction):
    """Mul instruction."""

    left: int
    right: int
