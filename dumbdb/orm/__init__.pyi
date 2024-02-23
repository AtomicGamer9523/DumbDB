import typing as _t

S = _t.TypeVar('S')

class DumbDB(_t.Generic[S]):
    def __init__(self, name: str) -> None:
        """
        Create a new DumbDB instance with the provided name
        """
    def set(self, key: int, value: S) -> None:
        """
        Set a value to a key (key has to be an unsigned integer)
        """
        ...
    def get(self, key: int) -> S | None: ...
    def delete(self, key: int) -> None: ...
    def contains(self, key: int) -> bool: ...
    def clear(self) -> None: ...
