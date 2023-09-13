# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/datatypes/translation_and_mat3x3.fbs".

# You can extend this class by creating a "TranslationAndMat3x3Ext" class in "translation_and_mat3x3_ext.py".

from __future__ import annotations

from typing import Sequence, Union

import pyarrow as pa
from attrs import define, field

from .. import datatypes
from .._baseclasses import (
    BaseExtensionArray,
    BaseExtensionType,
)
from ._overrides import translation_and_mat3x3__init_override  # noqa: F401

__all__ = [
    "TranslationAndMat3x3",
    "TranslationAndMat3x3Array",
    "TranslationAndMat3x3ArrayLike",
    "TranslationAndMat3x3Like",
    "TranslationAndMat3x3Type",
]


def _translation_and_mat3x3__translation__special_field_converter_override(
    x: datatypes.Vec3DLike | None,
) -> datatypes.Vec3D | None:
    if x is None:
        return None
    elif isinstance(x, datatypes.Vec3D):
        return x
    else:
        return datatypes.Vec3D(x)


def _translation_and_mat3x3__matrix__special_field_converter_override(
    x: datatypes.Mat3x3Like | None,
) -> datatypes.Mat3x3 | None:
    if x is None:
        return None
    elif isinstance(x, datatypes.Mat3x3):
        return x
    else:
        return datatypes.Mat3x3(x)


@define(init=False)
class TranslationAndMat3x3:
    """
    Representation of an affine transform via a 3x3 affine matrix paired with a translation.

    First applies the matrix, then the translation.
    """

    def __init__(self, *args, **kwargs):  # type: ignore[no-untyped-def]
        translation_and_mat3x3__init_override(self, *args, **kwargs)

    from_parent: bool = field(converter=bool)
    """
    If true, the transform maps from the parent space to the space where the transform was logged.
    Otherwise, the transform maps from the space to its parent.
    """

    translation: datatypes.Vec3D | None = field(
        default=None, converter=_translation_and_mat3x3__translation__special_field_converter_override
    )
    """
    3D translation, applied after the matrix.
    """

    matrix: datatypes.Mat3x3 | None = field(
        default=None, converter=_translation_and_mat3x3__matrix__special_field_converter_override
    )
    """
    3x3 matrix for scale, rotation & shear.
    """


TranslationAndMat3x3Like = TranslationAndMat3x3
TranslationAndMat3x3ArrayLike = Union[
    TranslationAndMat3x3,
    Sequence[TranslationAndMat3x3Like],
]


# --- Arrow support ---


class TranslationAndMat3x3Type(BaseExtensionType):
    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.struct(
                [
                    pa.field(
                        "translation",
                        pa.list_(pa.field("item", pa.float32(), nullable=False, metadata={}), 3),
                        nullable=True,
                        metadata={},
                    ),
                    pa.field(
                        "matrix",
                        pa.list_(pa.field("item", pa.float32(), nullable=False, metadata={}), 9),
                        nullable=True,
                        metadata={},
                    ),
                    pa.field("from_parent", pa.bool_(), nullable=False, metadata={}),
                ]
            ),
            "rerun.datatypes.TranslationAndMat3x3",
        )


class TranslationAndMat3x3Array(BaseExtensionArray[TranslationAndMat3x3ArrayLike]):
    _EXTENSION_NAME = "rerun.datatypes.TranslationAndMat3x3"
    _EXTENSION_TYPE = TranslationAndMat3x3Type

    @staticmethod
    def _native_to_pa_array(data: TranslationAndMat3x3ArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError  # You need to implement native_to_pa_array_override in translation_and_mat3x3_ext.py


TranslationAndMat3x3Type._ARRAY_TYPE = TranslationAndMat3x3Array

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(TranslationAndMat3x3Type())
