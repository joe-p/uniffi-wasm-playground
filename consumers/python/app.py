from arithmetic import add, div, ArithmeticError, InternalError

MAX_U64 = 18446744073709551615

try:
    div(1, 0)
except InternalError as e:
    print(f"We caught a panic! {e.__class__.__name__}: {e}")

try:
    add(MAX_U64, 1)
except ArithmeticError.IntegerOverflow as e:
    print(f"We caught an error! {e.__class__.__name__}: {e}")

try:
    add(MAX_U64 + 1, 0)
except ValueError as e:
    print(f"We caught an error thrown by the binding! {e.__class__.__name__}: {e}")
