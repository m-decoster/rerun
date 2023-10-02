# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/archetypes/view_coordinates.fbs".

# You can extend this class by creating a "ViewCoordinatesExt" class in "view_coordinates_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field

from .. import components
from .._baseclasses import Archetype
from ..error_utils import catch_and_log_exceptions
from .view_coordinates_ext import ViewCoordinatesExt

__all__ = ["ViewCoordinates"]


@define(str=False, repr=False, init=False)
class ViewCoordinates(ViewCoordinatesExt, Archetype):
    """
    How we interpret the coordinate system of an entity/space.

    For instance: What is "up"? What does the Z axis mean? Is this right-handed or left-handed?

    The three coordinates are always ordered as [x, y, z].

    For example [Right, Down, Forward] means that the X axis points to the right, the Y axis points
    down, and the Z axis points forward.

    Example
    -------
    ```python

    import rerun as rr

    rr.init("rerun_example_view_coordinates", spawn=True)

    rr.log("world", rr.ViewCoordinates.RIGHT_HAND_Z_UP, timeless=True)  # Set an up-axis
    rr.log(
        "world/xyz",
        rr.Arrows3D(
            vectors=[[1, 0, 0], [0, 1, 0], [0, 0, 1]],
            colors=[[255, 0, 0], [0, 255, 0], [0, 0, 255]],
        ),
    )
    ```
    """

    def __init__(self: Any, xyz: components.ViewCoordinatesLike):
        """Create a new instance of the ViewCoordinates archetype."""

        # You can define your own __init__ function as a member of ViewCoordinatesExt in view_coordinates_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(xyz=xyz)
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            xyz=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> ViewCoordinates:
        """Produce an empty ViewCoordinates, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    xyz: components.ViewCoordinatesBatch = field(
        metadata={"component": "required"},
        converter=components.ViewCoordinatesBatch._required,  # type: ignore[misc]
    )
    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__


if hasattr(ViewCoordinatesExt, "deferred_patch_class"):
    ViewCoordinatesExt.deferred_patch_class(ViewCoordinates)