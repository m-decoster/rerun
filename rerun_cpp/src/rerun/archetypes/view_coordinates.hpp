// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/view_coordinates.fbs".

#pragma once

#include "../collection.hpp"
#include "../components/view_coordinates.hpp"
#include "../data_cell.hpp"
#include "../indicator_component.hpp"
#include "../rerun_sdk_export.hpp"
#include "../result.hpp"

#include <cstdint>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: How we interpret the coordinate system of an entity/space.
    ///
    /// For instance: What is "up"? What does the Z axis mean? Is this right-handed or left-handed?
    ///
    /// The three coordinates are always ordered as [x, y, z].
    ///
    /// For example [Right, Down, Forward] means that the X axis points to the right, the Y axis points
    /// down, and the Z axis points forward.
    ///
    /// ## Example
    ///
    /// ### View coordinates for adjusting the eye camera
    /// ![image](https://static.rerun.io/viewcoordinates/0833f0dc8616a676b7b2c566f2a6f613363680c5/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_view_coordinates");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     rec.log_static("world", rerun::ViewCoordinates::RIGHT_HAND_Z_UP); // Set an up-axis
    ///     rec.log(
    ///         "world/xyz",
    ///         rerun::Arrows3D::from_vectors({{1.0, 0.0, 0.0}, {0.0, 1.0, 0.0}, {0.0, 0.0, 1.0}}
    ///         ).with_colors({{255, 0, 0}, {0, 255, 0}, {0, 0, 255}})
    ///     );
    /// }
    /// ```
    struct ViewCoordinates {
        rerun::components::ViewCoordinates xyz;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.components.ViewCoordinatesIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        // Extensions to generated type defined in 'view_coordinates_ext.cpp'

        /// Construct Vec3D from x/y/z values.
        constexpr ViewCoordinates(uint8_t axis0, uint8_t axis1, uint8_t axis2)
            : xyz(rerun::components::ViewCoordinates(axis0, axis1, axis2)) {}

        // <BEGIN_GENERATED:declarations>
        // This section is generated by running `scripts/generate_view_coordinate_defs.py --cpp`
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates ULF;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates UFL;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LUF;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LFU;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates FUL;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates FLU;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates ULB;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates UBL;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LUB;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LBU;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates BUL;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates BLU;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates URF;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates UFR;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RUF;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RFU;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates FUR;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates FRU;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates URB;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates UBR;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RUB;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RBU;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates BUR;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates BRU;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates DLF;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates DFL;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LDF;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LFD;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates FDL;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates FLD;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates DLB;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates DBL;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LDB;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LBD;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates BDL;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates BLD;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates DRF;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates DFR;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RDF;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RFD;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates FDR;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates FRD;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates DRB;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates DBR;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RDB;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RBD;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates BDR;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates BRD;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RIGHT_HAND_X_UP;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RIGHT_HAND_X_DOWN;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RIGHT_HAND_Y_UP;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RIGHT_HAND_Y_DOWN;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RIGHT_HAND_Z_UP;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RIGHT_HAND_Z_DOWN;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LEFT_HAND_X_UP;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LEFT_HAND_X_DOWN;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LEFT_HAND_Y_UP;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LEFT_HAND_Y_DOWN;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LEFT_HAND_Z_UP;
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LEFT_HAND_Z_DOWN;
        // <END_GENERATED:declarations>

      public:
        ViewCoordinates() = default;
        ViewCoordinates(ViewCoordinates&& other) = default;

        explicit ViewCoordinates(rerun::components::ViewCoordinates _xyz) : xyz(std::move(_xyz)) {}

        /// Returns the number of primary instances of this archetype.
        size_t num_instances() const {
            return 1;
        }
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::ViewCoordinates> {
        /// Serialize all set component batches.
        static Result<std::vector<DataCell>> serialize(const archetypes::ViewCoordinates& archetype
        );
    };
} // namespace rerun
