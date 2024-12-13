import re
import sys
import unittest

import numpy as np
from scipy.optimize import LinearConstraint, milp


def parse_machine(machine: str) -> list[tuple[int, int]]:
    coordinate_re = re.compile(r"X[+=](?P<X>\d+), Y[+=](?P<Y>\d+)")
    matches = [coordinate_re.search(line).groupdict() for line in machine.splitlines()]
    return [(int(match["X"]), int(match["Y"])) for match in matches]


def solve_machine(a_vec, b_vec, prize_vec):
    """
    Formulate the problem as an integer linear programming problem and solve it
    """
    c = np.array([3, 1])
    integrality = np.ones_like(c)

    # Constraint matrix
    A = np.array([a_vec, b_vec]).T

    # Our constraint is an exact equality, so we set it for both lower and upper bounds
    b_l = b_u = np.array(prize_vec)

    constraints = LinearConstraint(A, b_l, b_u)

    res = milp(c=c, constraints=constraints, integrality=integrality)
    return res.fun


def solution(input: str, offset: int = 0) -> int:
    machines = input.split("\n\n")
    total_tokens = 0

    for i, machine in enumerate(machines):
        a, b, prize = parse_machine(machine)

        # Apply offset if needed
        prize = tuple(p + offset for p in prize)

        res = solve_machine(a, b, prize)

        if res is not None:
            print(f"{i} is solvable")
            total_tokens += int(res)

    return total_tokens


class TestSolution(unittest.TestCase):
    def test_example(self):
        with open("example.txt", "r") as fd:
            self.assertEqual(solution(fd.read()), 480)

    def test_input(self):
        with open("input.txt", "r") as fd:
            self.assertEqual(solution(fd.read()), 26299)


if __name__ == "__main__":
    if sys.argv[1] == "2":
        offset = 10_000_000_000_000
    else:
        offset = 0

    with open(f"{sys.argv[2]}.txt", "r") as fd:
        print(solution(fd.read(), offset))
