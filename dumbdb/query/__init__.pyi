"""
This module contains the Query interface for DumbDB
"""
class DumbDB:
    """
    An Query interface for DumbDB
    """
    def __init__(self, name: str) -> None:
        """
        Create a new DumbDB instance with the provided name
        """
        ...
    def query(self, input: str) -> str:
        """
        Runs a query on the Database
        """
        ...
