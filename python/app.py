from flask import Flask, request, jsonify
from sympy import Eq, Symbol, solve, sympify

app = Flask(__name__, static_folder='static')


@app.route('/')
def index():
    return app.send_static_file('index.html')


@app.route('/solve', methods=['POST'])
def solve_route():
    data = request.get_json()
    if not data:
        return jsonify({'error': 'invalid json'}), 400

    equation_text = data.get('equation', '')
    variable_name = data.get('variable', 'x')

    try:
        variable = Symbol(variable_name)
        left_text, right_text = equation_text.split('=', 1)
        equation = Eq(sympify(left_text), sympify(right_text))
        solutions = solve(equation, variable)
        return jsonify({'solutions': [str(s) for s in solutions]})
    except Exception as e:
        return jsonify({'error': str(e)}), 400


if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
