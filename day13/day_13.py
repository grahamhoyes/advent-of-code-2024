import re
import sys
import unittest
from typing import Callable

import numpy as np
from scipy.optimize import LinearConstraint, milp


def parse_machine(machine: str) -> list[tuple[int, int]]:
    coordinate_re = re.compile(r"X[+=](?P<X>\d+), Y[+=](?P<Y>\d+)")
    matches = [coordinate_re.search(line).groupdict() for line in machine.splitlines()]
    return [(int(match["X"]), int(match["Y"])) for match in matches]


def all_integers(vec, tol=1e-3) -> bool:
    """
    Check if all elements of a vector are integers, within a tolerance
    """
    return np.all(np.abs(vec - np.round(vec)) < tol)


def solve_machine_ilp(a_vec: tuple, b_vec: tuple, prize_vec: tuple) -> int | None:
    """
    Formulate the problem as in integer linear programming problem and solve it

    :returns: The cost of the solution if there is one, else None
    """
    # Cost vector
    c = np.array([3, 1])
    integrality = np.ones_like(c)

    # Constraint matrix
    A = np.array([a_vec, b_vec]).astype(int).T

    # Our constraint is an exact equality, so we set it for both lower and upper bounds
    b_l = b_u = np.array(prize_vec).astype(int)

    constraints = LinearConstraint(A, b_l, b_u)

    res = milp(c=c, constraints=constraints, integrality=integrality)
    return int(res.fun) if res.success else None


def solve_machine_linear_system(
    a_vec: tuple, b_vec: tuple, prize_vec: tuple
) -> int | None:
    """
    Solve the problem as a system of linear equations.

    This doesn't work in the general case, but does for the particular inputs
    provided here.

    This avoids the numerical problems of the ILP solver for large values.
    """
    # Cost vector
    c = np.array([3, 1])

    # Constraint matrix
    A = np.array([a_vec, b_vec]).astype(int).T
    b = np.array(prize_vec).astype(int)

    sol = np.linalg.solve(A, b)

    # Only after integer solutions
    if not all_integers(sol):
        return None

    cost = c.T @ sol

    # Rounding to help with numerical instability
    return round(cost)


def solution(
    input: str, solver: Callable[[tuple, tuple, tuple], int | None], offset: int = 0
) -> int:
    machines = input.split("\n\n")
    total_tokens = 0

    for machine in machines:
        a, b, prize = parse_machine(machine)

        # Apply offset if needed
        prize = tuple(p + offset for p in prize)

        res = solver(a, b, prize)

        if res is not None:
            total_tokens += int(res)

    return total_tokens


PART_2_OFFSET = 10_000_000_000_000


class TestSolution(unittest.TestCase):
    def test_example_1(self):
        with open("example.txt", "r") as fd:
            self.assertEqual(solution(fd.read(), solve_machine_ilp), 480)

    def test_input_1(self):
        with open("input.txt", "r") as fd:
            self.assertEqual(solution(fd.read(), solve_machine_ilp), 26299)

    def test_example_2(self):
        with open("example.txt", "r") as fd:
            self.assertEqual(
                solution(fd.read(), solve_machine_linear_system, PART_2_OFFSET),
                875318608908,
            )

    def test_input_2(self):
        with open("input.txt", "r") as fd:
            self.assertEqual(
                solution(fd.read(), solve_machine_linear_system, PART_2_OFFSET),
                107824497933339,
            )


if __name__ == "__main__":
    if sys.argv[1] == "2":
        offset = PART_2_OFFSET
        solver = solve_machine_linear_system
    else:
        offset = 0
        solver = solve_machine_ilp

    with open(f"{sys.argv[2]}.txt", "r") as fd:
        print(solution(fd.read(), solver, offset))
