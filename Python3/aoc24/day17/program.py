"""Class for the execution of a program."""

from enum import Enum


class Instruction(Enum):
    """Enumeration of defined instructions."""

    Adv = 0
    Bxl = 1
    Bst = 2
    Jnz = 3
    Bxc = 4
    Out = 5
    Bdv = 6
    Cdv = 7


class Computer:
    """Class running a program."""

    # Registers
    a: int
    b: int
    c: int

    # Stack
    jump_flag: bool
    stack: list[int]
    stack_pointer: int

    stdout: list[int]

    do_shift: bool

    def __init__(self, program: list[int], a: int, do_shift: bool = True):
        """Initialize a new object."""
        self.a: int = a
        self.b: int = 0
        self.c: int = 0
        self.jump_flag: bool = True
        self.stack: list[int] = program
        self.stack_pointer: int = 0
        self.stdout: list[int] = []
        self.do_shift = do_shift

    def run(self) -> list[int]:
        """Run the loaded program until it exits."""
        while self.stack_pointer < len(self.stack):
            self._run_instruction()

        # Return stdout
        return self.stdout

    def run_until_jump(self) -> None:
        """Run the loaded program until it jumps."""
        while self.stack_pointer < len(self.stack) and self.jump_flag is True:
            self._run_instruction()

    def _run_instruction(self) -> None:
        """Run an instruction."""
        # Reset jump flag
        self.jump_flag = True

        # Run next instruction
        instruction: Instruction = Instruction(self.stack[self.stack_pointer])
        operand: int = self.stack[self.stack_pointer + 1]

        self._execute_instruction(instruction, operand)

        # Update stack pointer
        if self.jump_flag is True:
            self.stack_pointer += 2

    def resolve_combo(self, combo: int) -> int:
        """Resolve the combo value of the operand."""
        match combo:
            case 0 | 1 | 2 | 3:
                return combo
            case 4:
                return self.a
            case 5:
                return self.b
            case 6:
                return self.c
            case 7:
                raise NotImplementedError("Combo 7 is reserved")
            case _:
                raise ValueError(f"Invalid value for combo: {combo}")

    def _execute_instruction(self, instruction: Instruction, operand: int) -> None:
        """Run a single operation."""
        match instruction:
            case Instruction.Adv:
                if self.do_shift:
                    self.a >>= self.resolve_combo(operand)
            case Instruction.Bxl:
                self.b ^= operand
            case Instruction.Bst:
                self.b = self.resolve_combo(operand) & 7
            case Instruction.Jnz:
                if self.a != 0:
                    self.stack_pointer = operand
                    self.jump_flag = False  # Do not move stack pointer after jump
            case Instruction.Bxc:
                self.b ^= self.c
            case Instruction.Out:
                self.stdout.append(self.resolve_combo(operand) & 7)
            case Instruction.Bdv:
                self.b = self.a >> self.resolve_combo(operand)
            case Instruction.Cdv:
                self.c = self.a >> self.resolve_combo(operand)

    # Getters

    @property
    def registers(self) -> tuple[int, int, int]:
        """Return the tuple of registers."""
        return (self.a, self.b, self.c)
