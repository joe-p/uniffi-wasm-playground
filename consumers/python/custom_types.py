from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from playground import StrOrIntEnum

def lift_int_str_union(value: "StrOrIntEnum") -> int | str:
    return value[0]

def lower_int_str_union(value: int | str, enum):

    if isinstance(value, int):
        return enum.INT(value)
    elif isinstance(value, str):
        return enum.STR(value)
    else:
        raise ValueError("Value must be an int or str")

