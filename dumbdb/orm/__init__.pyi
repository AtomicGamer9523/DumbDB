"""
ORM for DumbDB
"""

import typing as _t

S = _t.TypeVar('S')

class DumbDB(_t.Generic[S]):
    """
    An ORM for DumbDB
    """
    def __init__(self, name: str) -> None:
        """
        Create a new DumbDB instance with the provided name
        """
    def set(self, key: int, value: S) -> None:
        """
        Set a value to a key (key has to be an unsigned integer)

        If the key already exists, the value will be overwritten
        If it does not exist, a new key-value pair will be created
        """
        ...
    def get(self, key: int) -> S | None:
        """
        Get a value from a key (key has to be an unsigned integer)
        May return None if the key does not exist
        """
        ...
    def delete(self, key: int) -> None:
        """
        Delete a key-value pair from the database
        """
        ...
    def contains(self, key: int) -> bool:
        """
        Check if the database contains a key
        """
        ...
    def clear(self) -> None:
        """
        Clears the entire database
        !!! WARNING, THIS IS PERMANENT !!!
        """
        ...
