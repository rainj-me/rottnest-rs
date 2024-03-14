import rottnest as rottnest
from .pele import  search_index_bm25, search_index_substring, index_file_bm25, merge_index_bm25, merge_index_substring, index_file_substring

__doc__ = rottnest.__doc__
if hasattr(rottnest, "__all__"):
    __all__ = rottnest.__all__