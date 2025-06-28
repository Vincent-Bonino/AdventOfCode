#!/usr/bin/env bash

mypy "aoc24"

ruff check "aoc24"

yapf --recursive --diff "aoc24" | ../scripts/colordiff.ps1
