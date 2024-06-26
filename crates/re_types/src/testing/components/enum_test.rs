// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/testing/components/enum_test.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Component**: A test of the enum type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EnumTest {
    /// Great film.
    Up = 1,

    /// Feeling blue.
    Down = 2,

    /// Correct.
    #[default]
    Right = 3,

    /// It's what's remaining.
    Left = 4,

    /// It's the only way to go.
    Forward = 5,

    /// Baby's got it.
    Back = 6,
}

impl EnumTest {
    /// All the different enum variants.
    pub const ALL: [Self; 6] = [
        Self::Up,
        Self::Down,
        Self::Right,
        Self::Left,
        Self::Forward,
        Self::Back,
    ];
}

impl ::re_types_core::SizeBytes for EnumTest {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        0
    }

    #[inline]
    fn is_pod() -> bool {
        true
    }
}

impl std::fmt::Display for EnumTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "Up"),
            Self::Down => write!(f, "Down"),
            Self::Right => write!(f, "Right"),
            Self::Left => write!(f, "Left"),
            Self::Forward => write!(f, "Forward"),
            Self::Back => write!(f, "Back"),
        }
    }
}

::re_types_core::macros::impl_into_cow!(EnumTest);

impl ::re_types_core::Loggable for EnumTest {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.testing.components.EnumTest".into()
    }

    #[allow(clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use arrow2::datatypes::*;
        DataType::Union(
            std::sync::Arc::new(vec![
                Field::new("_null_markers", DataType::Null, true),
                Field::new("Up", DataType::Null, true),
                Field::new("Down", DataType::Null, true),
                Field::new("Right", DataType::Null, true),
                Field::new("Left", DataType::Null, true),
                Field::new("Forward", DataType::Null, true),
                Field::new("Back", DataType::Null, true),
            ]),
            Some(std::sync::Arc::new(vec![
                0i32, 1i32, 2i32, 3i32, 4i32, 5i32, 6i32,
            ])),
            UnionMode::Sparse,
        )
    }

    #[allow(clippy::wildcard_imports)]
    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            let data: Vec<_> = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    datum
                })
                .collect();
            let num_variants = 6usize;
            let types = data
                .iter()
                .map(|a| match a.as_deref() {
                    None => 0,
                    Some(value) => *value as i8,
                })
                .collect();
            let fields: Vec<_> =
                std::iter::repeat(NullArray::new(DataType::Null, data.len()).boxed())
                    .take(1 + num_variants)
                    .collect();
            UnionArray::new(
                <crate::testing::components::EnumTest>::arrow_datatype(),
                types,
                fields,
                None,
            )
            .boxed()
        })
    }

    #[allow(clippy::wildcard_imports)]
    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow2::array::UnionArray>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.testing.components.EnumTest")?;
            let arrow_data_types = arrow_data.types();
            arrow_data_types
                .iter()
                .map(|typ| match typ {
                    0 => Ok(None),
                    1 => Ok(Some(EnumTest::Up)),
                    2 => Ok(Some(EnumTest::Down)),
                    3 => Ok(Some(EnumTest::Right)),
                    4 => Ok(Some(EnumTest::Left)),
                    5 => Ok(Some(EnumTest::Forward)),
                    6 => Ok(Some(EnumTest::Back)),
                    _ => Err(DeserializationError::missing_union_arm(
                        Self::arrow_datatype(),
                        "<invalid>",
                        *typ as _,
                    )),
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.components.EnumTest")?
        })
    }
}
