# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/archetypes/disconnected_space.fbs".

# You can extend this class by creating a "DisconnectedSpaceExt" class in "disconnected_space_ext.py".

from __future__ import annotations

from attrs import define, field

from .. import components
from .._baseclasses import (
    Archetype,
)

__all__ = ["DisconnectedSpace"]


@define(str=False, repr=False)
class DisconnectedSpace(Archetype):
    """
    Specifies that the entity path at which this is logged is disconnected from its parent.

    This is useful for specifying that a subgraph is independent of the rest of the scene.

    If a transform or pinhole is logged on the same path, this archetype's components
    will be ignored.

    Example
    -------
    ```python
    import rerun as rr
    import rerun.experimental as rr2

    rr.init("rerun_example_disconnect_space", spawn=True)

    # These two points can be projected into the same space..
    rr2.log("world/room1/point", rr2.Points3D([[0, 0, 0]]))
    rr2.log("world/room2/point", rr2.Points3D([[1, 1, 1]]))

    # ..but this one lives in a completely separate space!
    rr2.log("world/wormhole", rr2.DisconnectedSpace(True))
    rr2.log("world/wormhole/point", rr2.Points3D([[2, 2, 2]]))
    ```
    """

    # You can define your own __init__ function as a member of DisconnectedSpaceExt in disconnected_space_ext.py

    disconnected_space: components.DisconnectedSpaceArray = field(
        metadata={"component": "primary"},
        converter=components.DisconnectedSpaceArray.from_similar,  # type: ignore[misc]
    )
    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__
