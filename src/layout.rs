
use bevy_reflect::Reflect;
use pi_enum_default_macro::EnumDefault;

#[derive(Reflect, EnumDefault, Copy, Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

#[derive(Reflect, EnumDefault, Copy, Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum AlignSelf {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

#[derive(Reflect, Copy, Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum AlignContent {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

impl Default for AlignContent {
    fn default() -> AlignContent {
        AlignContent::Stretch
    }
}

#[derive(Reflect, Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Direction {
    Inherit,
    LTR,
    RTL,
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Inherit
    }
}

#[derive(Reflect, Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Display {
    Flex,
    None,
}

impl Default for Display {
    fn default() -> Display {
        Display::Flex
    }
}

#[derive(Reflect, Copy, Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

impl Default for FlexDirection {
    fn default() -> FlexDirection {
        FlexDirection::Row
    }
}

#[derive(Reflect, Copy, Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Default for JustifyContent {
    fn default() -> JustifyContent {
        JustifyContent::FlexStart
    }
}

// #[derive(Reflect, Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
// pub enum Overflow {
//     Visible,
//     Hidden,
//     Scroll,
// }

// impl Default for Overflow {
//     fn default() -> Overflow {
//         Overflow::Visible
//     }
// }

#[derive(Reflect, Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum PositionType {
    Relative,
    Absolute,
}

impl Default for PositionType {
    fn default() -> PositionType {
        PositionType::Relative
    }
}

#[derive(Reflect, Copy, Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Default for FlexWrap {
    fn default() -> FlexWrap {
        FlexWrap::Wrap
    }
}

// bitflags::bitflags! {
//     /// Bitflags info about the material a shader is currently rendering.
//     /// This is accessible in the shader in the [`StandardMaterialUniform`]
//     #[repr(transparent)]
//     pub struct NodeType: u32 {
//         const VNode         = (1 << 0);
//         // const EMISSIVE_TEXTURE           = (1 << 1);
//         // const METALLIC_ROUGHNESS_TEXTURE = (1 << 2);
//         // const OCCLUSION_TEXTURE          = (1 << 3);
//         // const DOUBLE_SIDED               = (1 << 4);
//         // const UNLIT                      = (1 << 5);
//         // const ALPHA_MODE_OPAQUE          = (1 << 6);
//         // const ALPHA_MODE_MASK            = (1 << 7);
//         // const ALPHA_MODE_BLEND           = (1 << 8);
//         // const TWO_COMPONENT_NORMAL_MAP   = (1 << 9);
//         // const FLIP_NORMAL_MAP_Y          = (1 << 10);
//         const NONE                       = 0;
//         const UNINITIALIZED              = 0xFFFF;
//     }
// }
