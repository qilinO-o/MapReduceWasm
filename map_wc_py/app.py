from typing import List
from typing import Tuple
import mapper

class Map(mapper.Mapper):
    def map(self, key: str, value: str) -> List[Tuple[str, str]]:
        terminators = ['.', ',', ' ', '\t', '\n', ';', ':', '"', '-', '\'', '(', ')', '[', ']', '?', '!', '_']
        for terminator in terminators:
            value = value.replace(terminator, ' ')
        value = value.split()
        ret = []
        for word in value:
            if len(word) > 0:
                ret.append((word, '1'))
        return ret
    