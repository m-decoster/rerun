# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/components/text_log_level.fbs".

# You can extend this class by creating a "TextLogLevelExt" class in "text_log_level_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    BaseDelegatingExtensionArray,
    BaseDelegatingExtensionType,
)

__all__ = ["TextLogLevelArray", "TextLogLevelType"]


class TextLogLevelType(BaseDelegatingExtensionType):
    _TYPE_NAME = "rerun.components.TextLogLevel"
    _DELEGATED_EXTENSION_TYPE = datatypes.Utf8Type


class TextLogLevelArray(BaseDelegatingExtensionArray[datatypes.Utf8ArrayLike]):
    _EXTENSION_NAME = "rerun.components.TextLogLevel"
    _EXTENSION_TYPE = TextLogLevelType
    _DELEGATED_ARRAY_TYPE = datatypes.Utf8Array


TextLogLevelType._ARRAY_TYPE = TextLogLevelArray

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(TextLogLevelType())
