use std::hash::Hash;
use bevy_reflect::Reflect;

/// 矩形， 采用start end top bottom定义矩形
#[derive(Reflect, Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct Rect<T: Reflect> {
    pub left: T,
    pub right: T,
    pub top: T,
    pub bottom: T,
}

impl<T: Reflect> Rect<T> {
	pub fn new(
		left: T,
		right: T,
		top: T,
		bottom: T,) -> Self {
		Rect{
			left,
			right,
			top,
			bottom
		}

	}
}

/// 大小
#[derive(Reflect, Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Hash, Default)]
pub struct Size<T: Reflect> {
    pub width: T,
    pub height: T,
}

impl<T: Reflect> Size<T> {
    pub fn undefined() -> Size<Number> {
        Size {
            width: Number::Undefined,
            height: Number::Undefined,
        }
    }
}

/// number
#[derive(Reflect, Copy, Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize, Default)]
pub enum Number {
    /// 用f32定义
    Defined(f32),
	/// 未定义
	#[default]
    Undefined,
}

impl Number {
    /// 判断是否定义
    pub fn is_defined(self) -> bool {
        self != Number::Undefined
    }
}

/// 点
#[derive(Reflect, Debug, Copy, Clone, PartialEq)]
pub struct Point<T: Reflect> {
    pub x: T,
    pub y: T,
}

#[derive(Reflect, Copy, Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize, EnumDefault)]
pub enum Dimension {
    Undefined,
    Auto,
    Points(f32),
    Percent(f32),
}

#[derive(Reflect, Clone, Copy, Debug, EnumDefault, Serialize, Deserialize)]
pub enum LengthUnit {
    Pixel(f32),
    Percent(f32),
}

#[derive(Reflect, Debug, Clone, Copy, EnumDefault, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum ImageRepeatOption {
    /// 拉伸源图像的边缘区域以填充每个边界之间的间隙。
    Stretch,
    /// 源图像的边缘区域被平铺（重复）以填充每个边界之间的间隙。可以修剪瓷砖以实现适当的配合。
    Repeat,
    /// 源图像的边缘区域被平铺（重复）以填充每个边界之间的间隙。可以拉伸瓷砖以实现适当的配合。
    Round,
    /// 源图像的边缘区域被平铺（重复）以填充每个边界之间的间隙。可以缩小瓷砖以实现适当的配合。
    Space,
}

// 图像填充的方式
#[derive(Reflect, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum FitType {
    None,
	#[default]
    Fill,
    Contain,
    Cover,
    ScaleDown,
    // Repeat,
    // RepeatX,
    // RepeatY,
}

const fn aa(x: u32) -> u32 {
	132 + x
}

fn cc() {
	let mut arr = Vec::new();
	for i in 0..1000 {
		arr.push(i);
	}

	for i in 0..1000 {
		aa(arr[i]);
	}
}