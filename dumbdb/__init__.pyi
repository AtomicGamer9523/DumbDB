"""
Database for Dummies.
A simple, easy-to-use, and lightweight database for small projects.
It is so simple a child could use it.
"""

import typing as _t

S = _t.TypeVar('S')
"""
The type of the value stored in the database
"""

class DumbDB(_t.Generic[S]):
    """
    An ORM for DumbDB
    """
    def __init__(self, name: str) -> None:
        """
        Create a new DumbDB instance with the provided name.
        And optionally, the permissions level. (Default: READ)
        """
    def set(self, key: int, value: S) -> None:
        """
        Set a value to a key (key has to be an unsigned integer)

        If the key already exists, the value will be overwritten
        If it does not exist, a new key-value pair will be created

        Permission Level Required: WRITE
        """
        ...
    def get(self, key: int) -> S | None:
        """
        Get a value from a key (key has to be an unsigned integer)
        May return None if the key does not exist

        Permission Level Required: READ
        """
        ...
    def delete(self, key: int) -> None:
        """
        Delete a key-value pair from the database

        Permission Level Required: WRITE
        """
        ...
    def contains(self, key: int) -> bool:
        """
        Check if the database contains a key
        Returns True if the key exists, False otherwise

        Permission Level Required: READ
        """
        ...
    def clear(self) -> None:
        """
        Clears the entire database.
        !!! WARNING, THIS IS PERMANENT !!!

        Permission Level Required: ADMIN
        """
        ...
