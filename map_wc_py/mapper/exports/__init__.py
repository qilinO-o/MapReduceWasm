from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from types import TracebackType
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some


class Map(Protocol):

    @abstractmethod
    def map(self, key: str, value: str) -> List[Tuple[str, str]]:
        raise NotImplementedError


