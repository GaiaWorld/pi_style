/// 用户操作的组件定义
/// 遵循css对应属性的语意
/// 扩展属性：
/// 1. as-image（force、advise、none）： 作为图像缓存， force表示强制缓存为图像；advise表示建议缓存为图像，当缓存空间不足时，不缓存；none表示不缓存为图像，该属性默认为none
/// 
use std::default::Default;
use std::{
    hash::{Hash, Hasher},
    mem::transmute,
};

use ordered_float::NotNan;
use pi_curves::easing::EEasingMode;
use pi_curves::steps::EStepMode;
use pi_flex_layout::style::{
    AlignContent, AlignItems, AlignSelf, Dimension, Direction, Display, FlexDirection, FlexWrap, JustifyContent, PositionType, OverflowWrap,
};
use smallvec::SmallVec;

use pi_atom::Atom;
use pi_flex_layout::prelude::{INode, Number, Rect, Size as FlexSize};

pub type Point2 = nalgebra::Point2<f32>;


pub type Aabb2 = parry2d::bounding_volume::Aabb;
#[derive(Debug, Deref, DerefMut, Clone, Serialize, Deserialize, Hash)]
pub struct NotNanRect(pub Rect<NotNan<f32>>);

impl Default for NotNanRect {
    fn default() -> Self {
        Self(unsafe {
            Rect {
                top: NotNan::new_unchecked(0.0),
                right: NotNan::new_unchecked(1.0),
                bottom: NotNan::new_unchecked(1.0),
                left: NotNan::new_unchecked(0.0),
            }
        })
    }
}

impl NotNanRect {
    pub fn new(top: NotNan<f32>, right: NotNan<f32>, bottom: NotNan<f32>, left: NotNan<f32>) -> Self { Self(Rect { top, right, bottom, left }) }

    /// 是否为单位rect（0~1）
    pub fn is_unit(&self) -> bool {
        if *self.left == 0.0 && *self.top == 0.0 && *self.right == 1.0 && *self.bottom == 1.0 {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BaseShape {
	Circle {
		radius: LengthUnit,
		center: Center,
	},
	Ellipse {
		rx: LengthUnit,
		ry: LengthUnit,
		center: Center,
	},
	Inset {
		rect_box: [LengthUnit;4],
		border_radius: BorderRadius,
	},
	Sector {
		rotate: f32, // 旋转 （单位： 弧度）
		angle: f32, // 弧度角
		radius: LengthUnit, // 半径
		center: Center
	}
}

impl Default for BaseShape {
    fn default() -> Self {
        BaseShape::Circle {
			radius: Default::default(),
			center: Default::default(),
		}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Center {
	pub x: LengthUnit,
	pub y: LengthUnit
}

#[derive(Debug, Deref, DerefMut, Clone, Serialize, Deserialize)]
pub struct CgColor(nalgebra::Vector4<f32>);
impl Hash for CgColor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            NotNan::new_unchecked(self.0.x).hash(state);
            NotNan::new_unchecked(self.0.y).hash(state);
            NotNan::new_unchecked(self.0.z).hash(state);
            NotNan::new_unchecked(self.0.w).hash(state);
        }
    }
}

impl CgColor{
    pub fn is_opacity(&self) -> bool {
        self.0.w == 1.0
    }
}

impl From<&str> for CgColor{
    fn from(value: &str) -> Self {
        let rgba = if value.starts_with("rgba"){
            let rgba_values: Vec<u8> = value
            .trim_start_matches("rgba(")
            .trim_end_matches(")")
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
            rgba_values
        } else if value.starts_with("rgb"){
            let mut rgba_values: Vec<u8> = value
            .trim_start_matches("rgb(")
            .trim_end_matches(")")
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
            rgba_values.push(255);
            rgba_values
        } else {
            let rgba_values = match value {
                "red" => vec![255,0,0,1],
                "black" => vec![0,0,0,1],
                _=> panic!("svg not surpport color {}", value),
            };

            rgba_values
        };

        CgColor::new(rgba[0]as f32 / 255.0 , rgba[1]as f32 / 255.0, rgba[2]as f32 / 255.0, rgba[3]as f32 / 255.0)
        
    }
}

impl CgColor {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Self(nalgebra::Vector4::new(x, y, z, w)) }
}

impl Default for CgColor {
    fn default() -> Self { Self(nalgebra::Vector4::new(0.0, 0.0, 0.0, 0.0)) }
}

#[derive(Default)]
pub struct Node;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
	pub property: SmallVec<[usize; 1]>, // 指定过度影响的属性
	pub duration: SmallVec<[Time; 1]>,                           // 指定需要多少毫秒完成过度
	pub delay: SmallVec<[Time; 1]>,                    // 启动过度前的延迟间隔。
    pub timing_function: SmallVec<[AnimationTimingFunction; 1]>, // 插值函数
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub name: AnimationName,                               // 指定要绑定到选择器的关键帧的名称
    pub duration: SmallVec<[Time; 1]>,                           // 动画指定需要多少毫秒完成
    pub timing_function: SmallVec<[AnimationTimingFunction; 1]>, // 设置动画将如何完成一个周期(插值函数)
    pub iteration_count: SmallVec<[IterationCount; 1]>,
    pub delay: SmallVec<[Time; 1]>,                    // 设置动画在启动前的延迟间隔。
    pub direction: SmallVec<[AnimationDirection; 1]>,  // 指定是否应该轮流反向播放动画。
    pub fill_mode: SmallVec<[AnimationFillMode; 1]>,   // 规定当动画不播放时（当动画完成时，或当动画有一个延迟未开始播放时），要应用到元素的样式。
    pub play_state: SmallVec<[AnimationPlayState; 1]>, // 指定动画是否正在运行或已暂停
}


#[derive(Debug, Default, Serialize, Clone, Deserialize)]
pub struct AnimationName {
	pub value: SmallVec<[Atom; 1]>,
	pub scope_hash: usize,
}

/// 动画循环次数
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Deref, DerefMut)]
pub struct IterationCount(pub f32);

// 动画默认播放一次
impl Default for IterationCount {
    fn default() -> Self { Self(1.0) }
}

/// 时间 ，单位 ms
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Deref, DerefMut, Default)]
pub struct Time(pub usize);

/// 动画循环方向
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, EnumDefault)]
pub enum AnimationDirection {
    /// 每个循环内动画向前循环，换言之，每个动画循环结束，动画重置到起点重新开始，这是默认属性。
    Normal,
    /// 动画交替反向运行，反向运行时，动画按步后退，同时，带时间功能的函数也反向，比如，ease-in 在反向时成为 ease-out。计数取决于开始时是奇数迭代还是偶数迭代
    Reverse,
    /// 反向运行动画，每周期结束动画由尾到头运行。
    Alternate,
    /// 反向交替，反向开始交替。动画第一次运行时是反向的，然后下一次是正向，后面依次循环。决定奇数次或偶数次的计数从 1 开始。
    AlternateReverse,
}

/// 动画播放状态
#[derive(Debug, Clone, Serialize, Deserialize, EnumDefault)]
pub enum AnimationPlayState {
    /// 正在播放
    Running,
    /// 暂停
    Paused,
}

/// 设置 CSS 动画在执行之前和之后如何将样式应用于其目标
#[derive(Debug, Clone, Serialize, Deserialize, EnumDefault)]
pub enum AnimationFillMode {
    /// 当动画未执行时，动画将不会将任何样式应用于目标，而是已经赋予给该元素的 CSS 规则来显示该元素。这是默认值
    None,
    /// 目标将保留由执行期间遇到的最后一个关键帧 (en-US)计算值。最后一个关键帧取决于animation-direction和animation-iteration-count的值
    Forwards,
    /// 动画将在应用于目标时立即应用第一个关键帧中定义的值，并在animation-delay期间保留此值。第一个关键帧取决于animation-direction的值
    Backwards,
    /// 动画将遵循forwards和backwards的规则，从而在两个方向上扩展动画属性
    Both,
}

// 淡入淡出方式
#[derive(EnumDefault, Debug, Clone, Serialize, Deserialize)]
pub enum EaseFunction {
    Back,
    Circle,
    Cubic,
    Sine,
    Quad,
    Quart,
    Quint,
    Expo,
    Elastic,
    Bounce,
}

/// 插值函数
#[derive(EnumDefault, Debug, Clone, Serialize, Deserialize)]
pub enum AnimationTimingFunction {
    /// 匀速
    Linear,
    /// 淡入淡出
    Ease(EEasingMode),
    /// 跳跃
    Step(usize, EStepMode),
    /// 贝塞尔曲线
    CubicBezier(f32, f32, f32, f32),
}


/// 布局大小
#[derive(Default, Deref, DerefMut, Clone, Serialize, Deserialize, Debug)]
pub struct Size(pub FlexSize<Dimension>);

/// 布局外边距
#[derive(Default, Deref, DerefMut, Clone, Serialize, Deserialize, Debug)]
pub struct Margin(pub Rect<Dimension>);

/// 布局内边距
#[derive(Default, Deref, DerefMut, Clone, Serialize, Deserialize, Debug)]
pub struct Padding(pub Rect<Dimension>);

/// 布局边框尺寸
#[derive(Default, Deref, DerefMut, Clone, Serialize, Deserialize, Debug)]
pub struct Border(pub Rect<Dimension>);

#[derive(Deref, DerefMut, Clone, Serialize, Deserialize, Debug)]
pub struct Position(pub Rect<Dimension>);

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct MinMax {
    pub min: FlexSize<Dimension>,
    pub max: FlexSize<Dimension>,
}

// 描述子节点行为的flex布局属性
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FlexContainer {
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_content: AlignContent,
    pub direction: Direction,
	pub overflow_wrap: OverflowWrap,
    pub row_gap: f32,
    pub column_gap: f32,
    pub auto_reduce: bool,
}

// 描述节点自身行为的flex布局属性
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FlexNormal {
    pub order: isize,
    pub flex_basis: Dimension,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub align_self: AlignSelf,
    pub position_type: PositionType,
    pub aspect_ratio: Number,
}

impl Default for Position {
    fn default() -> Self {
        Position(Rect {
            left: Dimension::Undefined,
            right: Dimension::Undefined,
            top: Dimension::Undefined,
            bottom: Dimension::Undefined,
        })
    }
}

impl Default for FlexContainer {
    fn default() -> Self {
        FlexContainer {
            flex_direction: Default::default(),
            flex_wrap: Default::default(),
            justify_content: Default::default(),
            align_items: Default::default(),
            align_content: AlignContent::FlexStart,
            direction: Default::default(),
			overflow_wrap: Default::default(),
            row_gap: 0.0,
            column_gap: 0.0,
            auto_reduce: false,
        }
    }
}

impl Default for FlexNormal {
    fn default() -> Self {
        Self {
            order: 0,
            flex_basis: Dimension::Auto,
            flex_grow: Default::default(),
            flex_shrink: Default::default(),
            align_self: Default::default(),
            position_type: Default::default(),
            aspect_ratio: Default::default(),
        }
    }
}

//================================== 组件
#[derive(Deref, DerefMut, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Debug)]
pub struct ZIndex(pub isize);

//超出部分的裁剪方式
#[derive(Deref, DerefMut, Clone, Default, Serialize, Deserialize, Debug)]
pub struct Overflow(pub bool);
//不透明度
#[derive(Deref, DerefMut, Clone, Debug, Serialize, Deserialize)]
pub struct Opacity(pub f32);

/// 渲染模式
#[derive(Clone, Copy, Debug, Serialize, Deserialize, EnumDefault)]
pub enum BlendMode {
    Normal,
    AlphaAdd,
    Subtract,
    Multiply,
    OneOne,
}

// 将display、visibility、enable合并为show组件
#[derive(Deref, DerefMut, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Show(pub usize);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AllTransform {
    pub transform: Vec<TransformFunc>,
	pub translate: Option<[LengthUnit;2]>, // 平移，单位：px
	pub scale: Option<[f32;2]>, // 缩放 0~1
	pub rotate: Option<f32>, // 旋转，单位： 弧度
}

pub type TransformFuncs = Vec<TransformFunc>;
// 背景色和class
#[derive(Debug, Clone, Default, Serialize, Deserialize, Deref)]
pub struct BackgroundColor(pub Color);

// class名称， 支持多个class， 当只有一个或两个class时， 有优化
#[derive(Debug, Clone, Default, Serialize, Deserialize, Deref, DerefMut)]
pub struct ClassName(pub SmallVec<[usize; 1]>);

// 边框颜色
#[derive(Debug, Clone, Default, Serialize, Deserialize, Deref, DerefMut)]
pub struct BorderColor(pub CgColor);

// 图片路劲及纹理
#[derive(Debug, Deref, DerefMut, Clone, Serialize, Deserialize, Default, Hash)]
pub struct BackgroundImage(pub Atom);

// 遮罩图片是图片路径或线性渐变色
#[derive(Clone, Debug, Serialize, Deserialize, EnumDefault)]
pub enum MaskImage {
    Path(Atom),
    LinearGradient(LinearGradientColor),
}


#[derive(Debug, Deref, DerefMut, Clone, Serialize, Deserialize, Default)]
pub struct MaskImageClip(pub NotNanRect);

// 滤镜， 与CSS的Filter不同， 该滤镜不依赖Filter 函数的先后顺序， 且同种滤镜设置多次，会覆盖前面的设置（css是一种叠加效果）
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Hsi {
    pub hue_rotate: f32,  //色相转换  -0.5 ~ 0.5 , 对应ps的-180 ~180
    pub saturate: f32,    // 饱和度  -1。0 ~1.0 ， 对应ps的 -100 ~ 100
    pub bright_ness: f32, //亮度 -1。0 ~1.0 ， 对应ps的 -100 ~ 100
}

// image图像的uv（仅支持百分比， 不支持像素值）
#[derive(Debug, Deref, DerefMut, Clone, Serialize, Deserialize, Hash, Default)]
pub struct BackgroundImageClip(pub NotNanRect);


// 边框图片
#[derive(Debug, Deref, DerefMut, Clone, Serialize, Deserialize, Default, Hash)]
pub struct BorderImage(pub Atom);

// borderImage图像的uv（仅支持百分比， 不支持像素值）
#[derive(Debug, Deref, DerefMut, Clone, Serialize, Deserialize, Hash, Default)]
pub struct BorderImageClip(pub NotNanRect);

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct BorderImageSlice {
    pub top: NotNan<f32>,
    pub right: NotNan<f32>,
    pub bottom: NotNan<f32>,
    pub left: NotNan<f32>,
    pub fill: bool,
}

impl Default for BorderImageSlice {
    fn default() -> Self {
        Self {
            left: unsafe { NotNan::new_unchecked(0.0) },
            top: unsafe { NotNan::new_unchecked(0.0) },
            right: unsafe { NotNan::new_unchecked(0.0) },
            bottom: unsafe { NotNan::new_unchecked(0.0) },
            fill: true,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Hash, Deref, DerefMut)]
pub struct BorderImageRepeat(pub ImageRepeat);

#[derive(Debug, Clone, Default, Serialize, Deserialize, Hash)]
pub struct ImageRepeat {
    pub x: ImageRepeatOption,
    pub y: ImageRepeatOption,
}

// 圆角， 目前仅支持x分量
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BorderRadius {
    pub x: [LengthUnit; 4], // 从左上角开始， 顺时针经过的每个角的圆角的x半径
	pub y: [LengthUnit; 4], // 从左上角开始， 顺时针经过的每个角的圆角的y半径
}

// 参考CSS的box-shadow的语法
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BoxShadow {
    pub h: f32,         // 水平偏移，正右负左
    pub v: f32,         // 垂直偏移，正下负上
    pub blur: f32,      // 模糊半径，0代表不模糊，
    pub spread: f32,    // 阴影扩展，上下左右各加上这个值
    pub color: CgColor, // 阴影颜色
}

// 文字样式
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Text {
    pub letter_spacing: f32,     //字符间距， 单位：像素
    pub word_spacing: f32,       //字符间距， 单位：像素
    pub line_height: LineHeight, //设置行高
    pub indent: f32,             // 缩进， 单位： 像素
    pub white_space: WhiteSpace, //空白处理
    pub color: Color,            //颜色
    pub stroke: Stroke,
    pub text_align: TextAlign,
    pub vertical_align: VerticalAlign,
}

// 文本内容
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextContent(pub String, pub Atom);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
	pub font_style: FontStyle, //	规定字体样式。参阅：font-style 中可能的值。
    pub font_weight: usize,    //	规定字体粗细。参阅：font-weight 中可能的值。
    pub font_size: FontSize,   //
    pub font_family: Atom,     //	规定字体系列。参阅：font-family 中可能的值。

    pub line_height: LineHeight,  //设置行高
	pub letter_spacing: f32,      //字符间距， 单位：像素
    pub word_spacing: f32,        //字符间距， 单位：像素
    pub white_space: WhiteSpace,  //空白处理

	pub text_overflow: TextOverflow,

    pub text_indent: f32,
	pub text_stroke: Stroke,
	pub vertical_align: VerticalAlign,
	pub text_align: TextAlign,

	pub color: Color, //颜色
    pub text_shadow: TextShadows, // 缩进， 单位： 像素

    pub text_outer_glow: OuterGlow, // 文字外发光
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            color: Default::default(),
            text_indent: Default::default(),
            text_stroke: Default::default(),
            text_align: Default::default(),
            text_shadow: Default::default(),
            letter_spacing: Default::default(),
            word_spacing: Default::default(),
            white_space: Default::default(),
            line_height: Default::default(),
            vertical_align: Default::default(),
            font_style: Default::default(),
            font_weight: 500,
            font_size: Default::default(),
            font_family: Default::default(),
			text_overflow: Default::default(),

            text_outer_glow: Default::default(),
        }
    }
}

pub type TextShadows = SmallVec<[TextShadow; 1]>;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextShadow {
    pub h: f32,         //	必需。水平阴影的位置。允许负值。	测试
    pub v: f32,         //	必需。垂直阴影的位置。允许负值。	测试
    pub blur: f32,      //	可选。模糊的距离。	测试
    pub color: CgColor, //	可选。阴影的颜色。参阅 CSS 颜色值。
}

/// 外发光
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OuterGlow {
    /// 外发光颜色
    pub color: CgColor,
    /// 外发光距离
    pub distance: f32,
    /// 外法光强度
    pub intensity: f32,
}

#[derive(Debug, Clone, EnumDefault, Serialize, Deserialize)]
pub enum TextOverflow {
	None,
	Clip,
	Ellipsis,
	Custom(String),
}

// TransformWillChange， 用于优化频繁变化的Transform
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TransformWillChange(pub TransformFuncs);

// #[derive(Debug)]
// pub struct Quad(pub Point2, pub Point2, pub Point2, pub Point2);

#[derive(Clone, Copy, Debug, EnumDefault, Serialize, Deserialize)]
pub enum LengthUnit {
    Pixel(f32),
    Percent(f32),
}

impl LengthUnit {
	#[inline]
	pub fn get_absolute_value(&self, refer: f32) -> f32 {
		match self {
			LengthUnit::Pixel(r) => *r,
			LengthUnit::Percent(r) => refer * r,
		}
	}
}

// #[derive(Clone, Copy, Debug, EnumDefault, Serialize, Deserialize)]
// pub enum Display {
//     Flex,
//     None,
// }

#[derive(Debug, Clone, EnumDefault, Serialize, Deserialize)]
pub enum Color {
    // RGB(CgColor),
    RGBA(CgColor),
    LinearGradient(LinearGradientColor),
    // RadialGradient(RadialGradientColor),
    // SvgLinearGradient(u64),
}
impl Hash for Color {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
		match self {
			Color::RGBA(color) => {
				NotNan::new(color.x).unwrap().hash(hasher);
				NotNan::new(color.y).unwrap().hash(hasher);
				NotNan::new(color.z).unwrap().hash(hasher);
				NotNan::new(color.w).unwrap().hash(hasher);
			},
			Color::LinearGradient(color) =>  {
				color.hash(hasher);
			},
		}
    }
}

impl Color {
    #[inline]
    pub fn is_opaque(&self) -> bool {
        match self {
            Color::RGBA(c) => c.w >= 1.0,
            Color::LinearGradient(l) => {
                for c in l.list.iter() {
                    if c.rgba.w < 1.0 {
                        return false;
                    }
                }
                true
            } // Color::RadialGradient(g) => {
              //     for c in g.list.iter() {
              //         if c.rgba.a < 1.0 {
              //             return false
              //         }
              //     }
              //     return true;
              // }
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinearGradientColor {
    pub direction: f32,
    pub list: Vec<ColorAndPosition>,
}

impl LinearGradientColor {
    pub fn is_opacity(&self) -> bool {
        let mut is_opacity = true;
        for i in self.list.iter() {
            is_opacity = is_opacity & i.rgba.is_opacity();
        }
        is_opacity
    }
}

impl Hash for LinearGradientColor {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        NotNan::new(self.direction).unwrap().hash(hasher);
        for l in self.list.iter() {
            NotNan::new(l.position).unwrap().hash(hasher);
            NotNan::new(l.rgba.x).unwrap().hash(hasher);
            NotNan::new(l.rgba.y).unwrap().hash(hasher);
            NotNan::new(l.rgba.z).unwrap().hash(hasher);
            NotNan::new(l.rgba.w).unwrap().hash(hasher);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadialGradientColor {
    pub center: (f32, f32),
    pub shape: RadialGradientShape,
    pub size: RadialGradientSize,
    pub list: Vec<ColorAndPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ColorAndPosition {
    // 位置百分比0~1
    pub position: f32,
    pub rgba: CgColor,
}

#[derive(Debug, Clone, Copy, EnumDefault, Serialize, Deserialize)]
pub enum RadialGradientSize {
    ClosestSide,
    FarthesSide,
    ClosestCorner,
    Farthescorner,
}

#[derive(Debug, Clone, Copy, EnumDefault, Serialize, Deserialize)]
pub enum RadialGradientShape {
    Ellipse,
    Circle,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Stroke {
    pub width: NotNan<f32>, //	描边宽度
    pub color: CgColor,     //	描边颜色
}

// 虚线
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrokeDasharray {
    pub real: f32,     //	实线部分长度
    pub empty: f32,    //	空白处长度
}
impl Default for StrokeDasharray {
    fn default() -> Self {
        Self { real: 100000000.0, empty: 0.0, }
    }
}

impl From<&str> for StrokeDasharray {
    fn from(value: &str) -> Self {
        let args: Vec<f32> = value
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        Self { real: args[0], empty: args[1],  }
    }
}

// 虚线
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shadow {
    pub color: CgColor, // 颜色
    pub offset_x: f32, // 偏移
    pub offset_y: f32,    
    pub blur_level: f32,  //模糊等级
}
impl Default for Shadow {
    fn default() -> Self {
        Self { color: CgColor::new(0.0,0.0,0.0,0.0), offset_x: 0.0, offset_y: 0.0, blur_level: 3.0 }
    }
}

// 111111111nnnnnnnnn
// 222222222mmmmmmmmm

// #[derive(Default, Debug, Clone, Serialize, Deserialize)]
// pub struct Outline {
//     pub width: NotNan<f32>, //	描边宽度
//     pub color: CgColor,     //	外发光颜色
// }

// 图像填充的方式
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FitType {
	/// 保留原有元素内容的长度和宽度，也就是说内容不会被重置。
    None,
	// 	默认，不保证保持原有的比例，内容拉伸填充整个内容容器。
    Fill,
	/// 保持原有尺寸比例。内容被缩放。
    Contain,
	/// 保持原有尺寸比例。但部分内容可能被剪切
    Cover,
	/// 保持原有尺寸比例。内容的尺寸与 none 或 contain 中的一个相同，取决于它们两个之间谁得到的对象尺寸会更小一些
    ScaleDown,
    // Repeat,
    // RepeatX,
    // RepeatY,
}

impl Default for FitType {
    fn default() -> Self { FitType::Fill }
}

#[derive(Debug, Clone, Copy, EnumDefault, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum ImageRepeatOption {
    /// 拉伸源图像的边缘区域以填充每个边界之间的间隙。
    Stretch,
    /// 源图像的边缘区域被平铺（重复）以填充每个边界之间的间隙。可以修剪瓷砖以实现适当的配合。
    Repeat,
    /// 类似 repeat 值。如果无法完整平铺所有图像，则对图像进行缩放以适应区域。。
    Round,
    /// 类似 repeat 值。如果无法完整平铺所有图像，扩展空间会分布在图像周围
    Space,
}

#[derive(Debug, Clone, Copy, EnumDefault, Serialize, Deserialize)]
pub enum FontSize {
    None,          // 默认尺寸。
    Length(usize), //把 font-size 设置为一个固定的值。
    Percent(f32),  //把 font-size 设置为基于父元素的一个百分比值。
}

//设置行高
#[derive(Debug, Clone, Copy, EnumDefault, Serialize, Deserialize)]
pub enum LineHeight {
    Normal,       //设置合理的行间距（等于font-size）
    Length(f32),  //固定像素
    Number(f32),  //设置数字，此数字会与当前的字体尺寸相乘来设置行间距。
    Percent(f32), //	基于当前字体尺寸的百分比行间距.
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumDefault)]
pub enum TransformFunc {
    TranslateX(LengthUnit),
	TranslateY(LengthUnit),
	Translate(LengthUnit, LengthUnit),

    ScaleX(f32),
    ScaleY(f32),
    Scale(f32, f32),

    RotateX(f32),
    RotateY(f32),
    RotateZ(f32),

    SkewX(f32),
    SkewY(f32),
}

#[derive(Debug, Clone, EnumDefault, Serialize, Deserialize)]
pub enum TransformOrigin {
    Center,
    XY(LengthUnit, LengthUnit),
}

impl TransformOrigin {
    pub fn to_value(&self, width: f32, height: f32) -> Point2 {
        match self {
            TransformOrigin::Center => Point2::new(0.5 * width, 0.5 * height),
            TransformOrigin::XY(x, y) => Point2::new(
                match x {
                    LengthUnit::Pixel(v) => v.clone(),
                    LengthUnit::Percent(v) => v * width,
                },
                match y {
                    LengthUnit::Pixel(v) => v.clone(),
                    LengthUnit::Percent(v) => v * height,
                },
            ),
        }
    }
}

#[derive(Debug)]
pub enum ShowType {
    Display = 1,    // 0表示 Flex
    Visibility = 2, // 0表示no Visible
    Enable = 12,    // 0表示no Enable
}

#[derive(Debug, Clone, EnumDefault, Copy, Serialize, Deserialize)]
pub enum Enable {
    Auto = 0,
    None = 1,
    Visible = 2,
}

// pub fn tanslate_to_matrix(x: f32, y: f32, z: f32) -> Matrix4{
// 	Matrix4::new(
// 		1.0, 0.0, 0.0, 0.0,
// 		0.0, 1.0, 0.0, 0.0,
// 		0.0, 0.0, 1.0, 0.0,
// 		x, y, z, 1.0,
// 	)
// }

// pub fn sacle_to_matrix(x: f32, y: f32, z: f32) -> Matrix4{
// 	Matrix4::new(
// 		x, 0.0, 0.0, 0.0,
// 		0.0, y, 0.0, 0.0,
// 		0.0, 0.0, z, 0.0,
// 		0.0, 0.0, 0.0, 1.0,
// 	)
// }

// pub fn angle_z_to_matrix(theta: f32) -> Matrix4 {
// 	let r = theta/180.0*(std::f32::consts::PI);
// 	let (s, c) = r.sin_cos();
// 	// http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
// 	// let (s, c) = Rad::sin_cos(theta.into());

// 	Matrix4::new(
// 		c, s, 0.0, 0.0,
// 		-s, c, 0.0, 0.0,
// 		0.0, 0.0, 1.0, 0.0,
// 		0.0, 0.0, 0.0, 1.0,
// 	)
// }

//对齐元素中的文本
#[derive(Debug, Clone, Copy, EnumDefault, Hash, Serialize, Deserialize)]
pub enum TextAlign {
    Left,    //把文本排列到左边。默认值：由浏览器决定。
    Right,   //把文本排列到右边。
    Center,  //把文本排列到中间。
    Justify, //实现两端对齐文本效果。
}

//设置元素中空白的处理方式
#[derive(Debug, Clone, Copy, EnumDefault, Hash, Serialize, Deserialize)]
pub enum WhiteSpace {
    Normal,  //	默认。空白会被浏览器忽略(其实是所有的空白被合并成一个空格), 超出范围会换行。
    Nowrap,  //	空白会被浏览器忽略(其实是所有的空白被合并成一个空格), 超出范围文本也不会换行，文本会在在同一行上继续，直到遇到 <br> 标签为止。
    PreWrap, //	保留所有空白符序列，超出范围会换行。
    Pre,     //	保留空白符，超出范围不会换行(利用yoga无法支持， 暂不支持)
    PreLine, //	合并空白符序列，如果存在换行符，优先保留换行符， 超出范围会换行。
}

impl WhiteSpace {
    pub fn allow_wrap(&self) -> bool {
        // match *self {
        //     WhiteSpace::Nowrap | WhiteSpace::Pre => false,
        //     WhiteSpace::Normal | WhiteSpace::PreWrap | WhiteSpace::PreLine => true,
        // }
        match *self {
            WhiteSpace::Nowrap => false,
            _ => true,
        }
    }

    pub fn preserve_newlines(&self) -> bool {
        match *self {
            WhiteSpace::Normal | WhiteSpace::Nowrap => false,
            WhiteSpace::Pre | WhiteSpace::PreWrap | WhiteSpace::PreLine => true,
        }
    }

    pub fn preserve_spaces(&self) -> bool {
        match *self {
            WhiteSpace::Normal | WhiteSpace::Nowrap | WhiteSpace::PreLine => true,
            WhiteSpace::Pre | WhiteSpace::PreWrap => false,
        }
    }
}

#[derive(Debug, Clone, Copy, EnumDefault, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum FontStyle {
    Normal,  //	默认值。标准的字体样式。
    Ttalic,  //	斜体的字体样式。
    Oblique, //	倾斜的字体样式。
}
#[derive(Debug, Clone, Copy, EnumDefault, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

impl Default for Opacity {
    fn default() -> Opacity { Opacity(1.0) }
}

impl Show {
    #[inline]
    pub fn get_display(&self) -> Display { unsafe { transmute((self.0 & (ShowType::Display as usize)) as u8) } }

    #[inline]
    pub fn set_display(&mut self, display: Display) {
        match display {
            Display::Flex => self.0 &= !(ShowType::Display as usize),
            Display::Grid => self.0 &= !(ShowType::Display as usize),
            Display::None => self.0 |= ShowType::Display as usize,
        }
    }

    #[inline]
    pub fn get_visibility(&self) -> bool { (self.0 & (ShowType::Visibility as usize)) != 0 }

    #[inline]
    pub fn set_visibility(&mut self, visibility: bool) {
        if visibility {
            self.0 |= ShowType::Visibility as usize;
        } else {
            self.0 &= !(ShowType::Visibility as usize);
        }
    }

    #[inline]
    pub fn get_enable(&self) -> Enable {
        let r = unsafe { transmute(((self.0 & (ShowType::Enable as usize)) >> 2) as u8) };
        r
    }

    #[inline]
    pub fn set_enable(&mut self, enable: Enable) { self.0 = self.0 & !(ShowType::Enable as usize) | ((enable as usize) << 2); }
}

impl Default for Show {
    fn default() -> Show { Show(ShowType::Visibility as usize) }
}

#[derive(Clone, Default, Deref, DerefMut, Debug, Serialize, Deserialize)]
pub struct NodeState(pub INode);

/// 作为图像缓存
#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumDefault, PartialEq, Eq, Hash)]
pub enum AsImage {
	/// 不缓存为图像
	None,
	/// 建议缓存为图像，当缓存空间不足时，不缓存
	Advise,
	/// 强制缓存为图像
	Force,
}
pub const GUI_STYLE_COUNT: u16 =  99;
// 枚举样式的类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[repr(u16)]
pub enum StyleType {
    BackgroundRepeat = 0,
    FontStyle = 1,
    FontWeight = 2,
    FontSize = 3,
    FontFamily = 4,
    LetterSpacing = 5,
    WordSpacing = 6,
    LineHeight = 7,
    TextIndent = 8,
    WhiteSpace = 9,
    TextAlign = 10,
    VerticalAlign = 11,
    Color = 12,
    TextStroke = 13,
    TextShadow = 14,

    BackgroundImage = 15,
    BackgroundImageClip = 16,
    ObjectFit = 17,

    BackgroundColor = 18,
    BoxShadow = 19,

    BorderImage = 20,
    BorderImageClip = 21,
    BorderImageSlice = 22,
    BorderImageRepeat = 23,

    BorderColor = 24,

    Hsi = 25,
    Blur = 26,
    MaskImage = 27,
    MaskImageClip = 28,

    Transform = 29,
    TransformOrigin = 30,
    TransformWillChange = 31,

    BorderRadius = 32,
    ZIndex = 33,
    Overflow = 34,
    BlendMode = 35,

    Display = 36,
    Visibility = 37,
    Enable = 38,

    Width = 39,
    Height = 40,

    MarginTop = 41,
    MarginRight = 42,
    MarginBottom = 43,
    MarginLeft = 44,

    PaddingTop = 45,
    PaddingRight = 46,
    PaddingBottom = 47,
    PaddingLeft = 48,

    BorderTop = 49,
    BorderRight = 50,
    BorderBottom = 51,
    BorderLeft = 52,

    PositionTop = 53,
    PositionRight = 54,
    PositionBottom = 55,
    PositionLeft = 56,

    MinWidth = 57,
    MinHeight = 58,
    MaxHeight = 59,
    MaxWidth = 60,

    Direction = 61,
    FlexDirection = 62,
    FlexWrap = 63,
    JustifyContent = 64,
    AlignContent = 65,
    AlignItems = 66,

    PositionType = 67,
    AlignSelf = 68,
    FlexShrink = 69,
    FlexGrow = 70,
    AspectRatio = 71,
    Order = 72,
    FlexBasis = 73,

    Opacity = 74,
    TextContent = 75,
    NodeState = 76,

    // TransformFunc = 78,

    AnimationName = 77,
    AnimationDuration = 78,
    AnimationTimingFunction = 79,
    AnimationDelay = 80,
    AnimationIterationCount = 81,
    AnimationDirection = 82,
    AnimationFillMode = 83,
    AnimationPlayState = 84,

	ClipPath = 85,
	Translate = 86,
	Scale = 87,
	Rotate = 88,

	AsImage = 89,

	TextOverflow = 90,

	OverflowWrap = 91,

	TransitionProperty = 92,
	TransitionDuration= 93,
	TransitionTimingFunction = 94,
	TransitionDelay = 95,

    TextOuterGlow = 96,

    RowGap = 97,
    ColumnGap = 98,
    AutoReduce = 99,
}

// // 可插值属性
// pub const INTERPOLABLE_PROPERTY: usize = StyleType::BackgroundRepeat as usize | 
// 	StyleType::Color |
// 	StyleType::BackgroundImageClip |
// 	StyleType::BackgroundColor |
// 	StyleType::BorderColor |
// 	StyleType::Hsi |
// 	StyleType::Blur |
// 	StyleType::Transform |
// 	StyleType::BorderRadius |

//     StyleType::Width |
//     StyleType::Height |

//     StyleType::MarginTop |
//     StyleType::MarginRight |
//     StyleType::MarginBottom |
//     StyleType::MarginLeft |

//     StyleType::PaddingTop |
//     StyleType::PaddingRight |
//     StyleType::PaddingBottom |
//     StyleType::PaddingLeft |

//     StyleType::BorderTop |
//     StyleType::BorderRight |
//     StyleType::BorderBottom |
//     StyleType::BorderLeft |

//     StyleType::PositionTop |
//     StyleType::PositionRight |
//     StyleType::PositionBottom |
//     StyleType::PositionLeft |

//     StyleType::MinWidth |
//     StyleType::MinHeight |
//     StyleType::MaxHeight |
//     StyleType::MaxWidth |

//     StyleType::Opacity |

//     StyleType::TransformFunc |
// 	StyleType::Translate |
// 	StyleType::Scale |
// 	StyleType::Rotate;



