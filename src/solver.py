import json
import sys
from pathlib import Path

from sympy import Eq, Symbol, solve, sympify


def main() -> None:
    if len(sys.argv) != 3:
        raise SystemExit("usage: solver.py <input.json> <output.json>")

    input_path = Path(sys.argv[1])
    output_path = Path(sys.argv[2])

    payload = json.loads(input_path.read_text(encoding="utf-8"))
    equation_text = payload["equation"]
    variable = Symbol(payload["variable"])

    left_text, right_text = equation_text.split("=", 1)
    equation = Eq(sympify(left_text), sympify(right_text))
    solutions = solve(equation, variable)

    output_path.write_text(
        json.dumps({"solutions": [str(item) for item in solutions]}, indent=2),
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()