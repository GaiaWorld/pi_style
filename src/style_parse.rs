//! 解析字符串格式的样式

use std::collections::hash_map::Entry;
use std::str::FromStr;
use std::{collections::VecDeque, intrinsics::transmute};

use bitvec::prelude::BitArray;
use cssparser::{CowRcStr, Delimiter, ParseError, Parser, ParserInput, Token, BasicParseError, SourceLocation, ParseErrorKind, BasicParseErrorKind};
use ordered_float::NotNan;
use pi_atom::Atom;
use pi_curves::steps::EStepMode;
use pi_flex_layout::{
    prelude::Rect,
    style::{AlignContent, AlignItems, AlignSelf, Dimension, Display, FlexDirection, FlexWrap, JustifyContent, PositionType, OverflowWrap},
};
use pi_hash::XHashMap;
use smallvec::SmallVec;
use thiserror::Error;

use crate::style::{
    Animation, AnimationDirection, AnimationFillMode, AnimationName, AnimationPlayState, AnimationTimingFunction, AsImage, BaseShape, BlendMode, BorderImageSlice, BorderRadius, BoxShadow, Center, CgColor, Color, ColorAndPosition, Enable, FitType, FontSize, Hsi, ImageRepeat, ImageRepeatOption, IterationCount, LengthUnit, LineHeight, LinearGradientColor, MaskImage, NotNanRect, OuterGlow, Stroke, TextAlign, TextContent, TextOverflow, TextShadow, Time, TransformFunc, TransformOrigin, Transition, WhiteSpace
};
use crate::style::StyleType;

use super::style_type::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Attribute {
    BackgroundRepeat(BackgroundRepeatType), // 0 
    FontStyle(FontStyleType),               // 1
    FontWeight(FontWeightType),             // 2
    FontSize(FontSizeType),                 // 3
    FontFamily(FontFamilyType),             // 4
    LetterSpacing(LetterSpacingType),       // 5
    WordSpacing(WordSpacingType),           // 6
    LineHeight(LineHeightType),             // 7
    TextIndent(TextIndentType),             // 8
    WhiteSpace(WhiteSpaceType),             // 9

    TextAlign(TextAlignType),         // 10
    VerticalAlign(VerticalAlignType), // 11
    Color(ColorType),                 // 12
    TextStroke(TextStrokeType),       // 13
    TextShadow(TextShadowType),       // 14

    BackgroundImage(BackgroundImageType),         // 15
    BackgroundImageClip(BackgroundImageClipType), // 16
    ObjectFit(ObjectFitType),                     // 17
    BackgroundColor(BackgroundColorType),         // 18
    BoxShadow(BoxShadowType),                     // 19
    BorderImage(BorderImageType),                 // 20
    BorderImageClip(BorderImageClipType),         // 21
    BorderImageSlice(BorderImageSliceType),       // 22
    BorderImageRepeat(BorderImageRepeatType),     // 23

    BorderColor(BorderColorType), // 24

    Hsi(HsiType),                                 // 25
    Blur(BlurType),                               // 26
    MaskImage(MaskImageType),                     // 27
    MaskImageClip(MaskImageClipType),             // 28
    Transform(TransformType),                     // 29
    TransformOrigin(TransformOriginType),         // 30
    TransformWillChange(TransformWillChangeType), // 31
    BorderRadius(BorderRadiusType),               // 32
    ZIndex(ZIndexType),                           // 33
    Overflow(OverflowType),                       // 34

    BlendMode(BlendModeType),   // 35
    Display(DisplayType),       // 36
    Visibility(VisibilityType), // 37
    Enable(EnableType),         // 38

    Width(WidthType),   // 39
    Height(HeightType), // 40

    MarginTop(MarginTopType),       // 41
    MarginRight(MarginRightType),   // 42
    MarginBottom(MarginBottomType), // 43
    MarginLeft(MarginLeftType),     // 44

    PaddingTop(PaddingTopType),       // 45
    PaddingRight(PaddingRightType),   // 46
    PaddingBottom(PaddingBottomType), // 47
    PaddingLeft(PaddingLeftType),     // 48

    BorderTop(BorderTopType),       // 49
    BorderRight(BorderRightType),   // 50
    BorderBottom(BorderBottomType), // 51
    BorderLeft(BorderLeftType),     // 52

    PositionTop(PositionTopType),       // 53
    PositionRight(PositionRightType),   // 54
    PositionBottom(PositionBottomType), // 55
    PositionLeft(PositionLeftType),     // 56

    MinWidth(MinWidthType),             // 57
    MinHeight(MinHeightType),           // 58
    MaxHeight(MaxHeightType),           // 58
    MaxWidth(MaxWidthType),             // 60
    Direction(DirectionType),           // 61
    FlexDirection(FlexDirectionType),   // 62
    FlexWrap(FlexWrapType),             // 63
    JustifyContent(JustifyContentType), // 64
    AlignContent(AlignContentType),     // 65
    AlignItems(AlignItemsType),         // 66

    PositionType(PositionTypeType), // 67
    AlignSelf(AlignSelfType),       // 69
    FlexShrink(FlexShrinkType),     // 69
    FlexGrow(FlexGrowType),         // 70
    AspectRatio(AspectRatioType),   // 71
    Order(OrderType),               // 72
    FlexBasis(FlexBasisType),       // 73
    Opacity(OpacityType),           // 74

    TextContent(TextContentType), // 75

    VNode(VNodeType), // 76

    AnimationName(AnimationNameType),                     // 77
    AnimationDuration(AnimationDurationType),             // 78
    AnimationTimingFunction(AnimationTimingFunctionType), // 79
    AnimationDelay(AnimationDelayType),                   // 80
    AnimationIterationCount(AnimationIterationCountType), // 81
    AnimationDirection(AnimationDirectionType),           // 82
    AnimationFillMode(AnimationFillModeType),             // 83
    AnimationPlayState(AnimationPlayStateType),           // 84
	ClipPath(ClipPathType),   // 85
	Translate(TranslateType),                     // 86
	Scale(ScaleType),                     // 87
	Rotate(RotateType),                     // 88

	AsImage(AsImageType),                     // 89

	TextOverflow(TextOverflowType), // 90
	OverflowWrap(OverflowWrapType), // 91

	TransitionProperty(TransitionPropertyType),                     // 92
    TransitionDuration(TransitionDurationType),             // 93
    TransitionTimingFunction(TransitionTimingFunctionType), // 94
    TransitionDelay(TransitionDelayType),                   // 95

    TextOuterGlow(TextOuterGlowType),       // 96

    RowGap(RowGapType),     // 97
    ColumnGap(ColumnGapType),     // 98
    AutoReduce(AutoReduceType), // 99
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct KeyFrameList {
    pub frames: XHashMap<Atom, XHashMap<NotNan<f32>, VecDeque<Attribute>>>,
	pub scope_hash: usize,
}

// #[derive(Debug, Serialize, Deserialize, Default)]
// pub struct KeyFrame {
// 	// progress: f32, // 0~1, 进度
// 	attrs: VecDeque<Attribute>, // 属性, key为属性索引
// }

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ClassMap {
    pub attrs: VecDeque<Attribute>,
    pub classes: Vec<ClassItem>,
    pub key_frames: KeyFrameList,
}

impl ClassMap {
    pub fn to_class_sheet(mut self, class_sheet: &mut ClassSheet) {
        for class in self.classes.iter() {
            let class_meta = style_list_to_buffer(&mut class_sheet.style_buffer, &mut self.attrs, class.count);
            class_sheet.class_map.insert(class.class_name, class_meta);
        }
    }
}

pub fn style_list_to_buffer(style_buffer: &mut Vec<u8>, style_list: &mut VecDeque<Attribute>, mut count: usize) -> ClassMeta {
	let start = style_buffer.len();
	let mut class_meta = ClassMeta {
		start,
		end: start,
		class_style_mark: BitArray::default(),
	};

	loop {
		if count == 0 {
			break;
		}
		let r = style_list.pop_front().unwrap();
		style_to_buffer(style_buffer, r, &mut class_meta);
		count -= 1;
	}
	class_meta.end = style_buffer.len();

	class_meta
}

pub fn style_to_buffer(style_buffer: &mut Vec<u8>, mut style: Attribute,  class_meta: &mut ClassMeta) {

	match &mut style {
		Attribute::BackgroundRepeat(r) => unsafe {
			class_meta.class_style_mark.set(BackgroundRepeatType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::PositionType(r) => unsafe {
			class_meta.class_style_mark.set(PositionTypeType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::FlexWrap(r) => unsafe {
			class_meta.class_style_mark.set(FlexWrapType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::FlexDirection(r) => unsafe {
			class_meta.class_style_mark.set(FlexDirectionType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::AlignContent(r) => unsafe {
			class_meta.class_style_mark.set(AlignContentType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::AlignItems(r) => unsafe {
			class_meta.class_style_mark.set(AlignItemsType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::AlignSelf(r) => unsafe {
			class_meta.class_style_mark.set(AlignSelfType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::JustifyContent(r) => unsafe {
			class_meta.class_style_mark.set(JustifyContentType::get_type() as usize, true);
			r.write(style_buffer);
		},

		Attribute::ObjectFit(r) => unsafe {
			class_meta.class_style_mark.set(ObjectFitType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::TextAlign(r) => unsafe {
			class_meta.class_style_mark.set(TextAlignType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::VerticalAlign(r) => unsafe {
			class_meta.class_style_mark.set(VerticalAlignType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::WhiteSpace(r) => unsafe {
			class_meta.class_style_mark.set(WhiteSpaceType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::FontStyle(r) => unsafe {
			class_meta.class_style_mark.set(FontStyleType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::Enable(r) => unsafe {
			class_meta.class_style_mark.set(EnableType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::Display(r) => unsafe {
			class_meta.class_style_mark.set(DisplayType::get_type() as usize, true);
			r.write(style_buffer);
		},

		Attribute::Visibility(r) => unsafe {
			class_meta.class_style_mark.set(VisibilityType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::Overflow(r) => unsafe {
			class_meta.class_style_mark.set(OverflowType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::LetterSpacing(r) => unsafe {
			class_meta.class_style_mark.set(LetterSpacingType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::TextIndent(r) => unsafe {
			class_meta.class_style_mark.set(TextIndentType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::WordSpacing(r) => unsafe {
			class_meta.class_style_mark.set(WordSpacingType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::FontWeight(r) => unsafe {
			class_meta.class_style_mark.set(FontWeightType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::FontFamily(r) => unsafe {
			class_meta.class_style_mark.set(FontFamilyType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::ZIndex(r) => unsafe {
			class_meta.class_style_mark.set(ZIndexType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::BackgroundImage(r) => unsafe {
			class_meta.class_style_mark.set(BackgroundImageType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::BorderImage(r) => unsafe {
			class_meta.class_style_mark.set(BorderImageType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::FlexShrink(r) => unsafe {
			class_meta.class_style_mark.set(FlexShrinkType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::FlexGrow(r) => unsafe {
			class_meta.class_style_mark.set(FlexGrowType::get_type() as usize, true);
			r.write(style_buffer);
		},

		Attribute::Opacity(r) => unsafe {
			class_meta.class_style_mark.set(OpacityType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::BorderImageRepeat(r) => unsafe {
			class_meta.class_style_mark.set(BorderImageRepeatType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::FontSize(r) => unsafe {
			class_meta.class_style_mark.set(FontSizeType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::Blur(r) => unsafe {
			class_meta.class_style_mark.set(BlurType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::LineHeight(r) => unsafe {
			class_meta.class_style_mark.set(LineHeightType::get_type() as usize, true);
			r.write(style_buffer);
		},

		Attribute::Width(r) => unsafe {
			class_meta.class_style_mark.set(WidthType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::Height(r) => unsafe {
			class_meta.class_style_mark.set(HeightType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::MarginLeft(r) => unsafe {
			class_meta.class_style_mark.set(MarginLeftType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::MarginTop(r) => unsafe {
			class_meta.class_style_mark.set(MarginTopType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::MarginBottom(r) => unsafe {
			class_meta.class_style_mark.set(MarginBottomType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::MarginRight(r) => unsafe {
			class_meta.class_style_mark.set(MarginRightType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::PaddingLeft(r) => unsafe {
			class_meta.class_style_mark.set(PaddingLeftType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::PaddingTop(r) => unsafe {
			class_meta.class_style_mark.set(PaddingTopType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::PaddingBottom(r) => unsafe {
			class_meta.class_style_mark.set(PaddingBottomType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::PaddingRight(r) => unsafe {
			class_meta.class_style_mark.set(PaddingRightType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::BorderLeft(r) => unsafe {
			class_meta.class_style_mark.set(BorderLeftType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::BorderTop(r) => unsafe {
			class_meta.class_style_mark.set(BorderTopType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::BorderBottom(r) => unsafe {
			class_meta.class_style_mark.set(BorderBottomType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::BorderRight(r) => unsafe {
			class_meta.class_style_mark.set(BorderRightType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::MinWidth(r) => unsafe {
			class_meta.class_style_mark.set(MinWidthType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::MinHeight(r) => unsafe {
			class_meta.class_style_mark.set(MinHeightType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::MaxHeight(r) => unsafe {
			class_meta.class_style_mark.set(MaxHeightType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::MaxWidth(r) => unsafe {
			class_meta.class_style_mark.set(MaxWidthType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::FlexBasis(r) => unsafe {
			class_meta.class_style_mark.set(FlexBasisType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::PositionLeft(r) => unsafe {
			class_meta.class_style_mark.set(PositionLeftType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::PositionTop(r) => unsafe {
			class_meta.class_style_mark.set(PositionTopType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::PositionRight(r) => unsafe {
			class_meta.class_style_mark.set(PositionRightType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::PositionBottom(r) => unsafe {
			class_meta.class_style_mark.set(PositionBottomType::get_type() as usize, true);
			r.write(style_buffer);
		},

		Attribute::MaskImage(r) => unsafe {
			class_meta.class_style_mark.set(MaskImageType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::BlendMode(r) => unsafe {
			class_meta.class_style_mark.set(BlendModeType::get_type() as usize, true);
			r.write(style_buffer);
		},

		Attribute::BackgroundColor(r) => unsafe {
			class_meta.class_style_mark.set(BackgroundColorType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::BorderColor(r) => unsafe {
			class_meta.class_style_mark.set(BorderColorType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::BoxShadow(r) => unsafe {
			class_meta.class_style_mark.set(BoxShadowType::get_type() as usize, true);
			r.write(style_buffer);
		},

		Attribute::BackgroundImageClip(r) => unsafe {
			class_meta.class_style_mark.set(BackgroundImageClipType::get_type() as usize, true);
			r.write(style_buffer);
		},

		Attribute::BorderImageClip(r) => unsafe {
			class_meta.class_style_mark.set(BorderImageClipType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::BorderImageSlice(r) => unsafe {
			class_meta.class_style_mark.set(BorderImageSliceType::get_type() as usize, true);
			r.write(style_buffer);
		},

		Attribute::Color(r) => unsafe {
			class_meta.class_style_mark.set(ColorType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::TextShadow(r) => unsafe {
			class_meta.class_style_mark.set(TextShadowType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::TextStroke(r) => unsafe {
			class_meta.class_style_mark.set(TextStrokeType::get_type() as usize, true);
			r.write(style_buffer);
		},

		Attribute::BorderRadius(r) => unsafe {
			class_meta.class_style_mark.set(BorderRadiusType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::Transform(r) => unsafe {
			class_meta.class_style_mark.set(TransformType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::TransformOrigin(r) => unsafe {
			class_meta.class_style_mark.set(TransformOriginType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::Hsi(r) => unsafe {
			class_meta.class_style_mark.set(HsiType::get_type() as usize, true);
			r.write(style_buffer);
		},

		Attribute::MaskImageClip(r) => unsafe {
			class_meta.class_style_mark.set(MaskImageClipType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::TransformWillChange(r) => unsafe {
			class_meta.class_style_mark.set(TransformWillChangeType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::Direction(r) => unsafe {
			class_meta.class_style_mark.set(TransformWillChangeType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::AspectRatio(r) => unsafe {
			class_meta.class_style_mark.set(TransformWillChangeType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::Order(r) => unsafe {
			class_meta.class_style_mark.set(TransformWillChangeType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::TextContent(r) => unsafe {
			class_meta.class_style_mark.set(TextContentType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::VNode(r) => unsafe {
			class_meta.class_style_mark.set(VNodeType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::AnimationName(r) => unsafe {
			class_meta.class_style_mark.set(AnimationNameType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::AnimationDuration(r) => unsafe {
			class_meta.class_style_mark.set(AnimationDurationType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::AnimationTimingFunction(r) => unsafe {
			class_meta.class_style_mark.set(AnimationTimingFunctionType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::AnimationDelay(r) => unsafe {
			class_meta.class_style_mark.set(AnimationDelayType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::AnimationIterationCount(r) => unsafe {
			class_meta.class_style_mark.set(AnimationIterationCountType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::AnimationDirection(r) => unsafe {
			class_meta.class_style_mark.set(AnimationDirectionType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::AnimationFillMode(r) => unsafe {
			class_meta.class_style_mark.set(AnimationFillModeType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::AnimationPlayState(r) => unsafe {
			class_meta.class_style_mark.set(AnimationPlayStateType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::ClipPath(r) => unsafe {
			class_meta.class_style_mark.set(ClipPathType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::Translate(r) => unsafe {
			class_meta.class_style_mark.set(TranslateType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::Scale(r) => unsafe {
			class_meta.class_style_mark.set(ScaleType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::Rotate(r) => unsafe {
			class_meta.class_style_mark.set(RotateType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::AsImage(r) => unsafe {
			class_meta.class_style_mark.set(AsImageType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::TextOverflow(r) => unsafe {
			class_meta.class_style_mark.set(TextOverflowType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::OverflowWrap(r) => unsafe {
			class_meta.class_style_mark.set(OverflowWrapType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::TransitionProperty(r) => unsafe {
			class_meta.class_style_mark.set(TransitionPropertyType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::TransitionDuration(r) => unsafe {
			class_meta.class_style_mark.set(TransitionDurationType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::TransitionTimingFunction(r) => unsafe {
			class_meta.class_style_mark.set(TransitionTimingFunctionType::get_type() as usize, true);
			r.write(style_buffer);
		},
		Attribute::TransitionDelay(r) => unsafe {
			class_meta.class_style_mark.set(TransitionDelayType::get_type() as usize, true);
			r.write(style_buffer);
		},
        Attribute::TextOuterGlow(r) => unsafe {
            class_meta.class_style_mark.set(TextOuterGlowType::get_type() as usize, true);
			r.write(style_buffer);
        },
        Attribute::RowGap(r) => unsafe {
            class_meta.class_style_mark.set(RowGapType::get_type() as usize, true);
			r.write(style_buffer);
        },
        Attribute::ColumnGap(r) => unsafe {
            class_meta.class_style_mark.set(ColumnGapType::get_type() as usize, true);
			r.write(style_buffer);
        },
        Attribute::AutoReduce(r) => unsafe {
            class_meta.class_style_mark.set(AutoReduceType::get_type() as usize, true);
			r.write(style_buffer);
        },
	}
	std::mem::forget(style);
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClassItem {
    pub count: usize,
    pub class_name: usize,
}

pub fn parse_class_map_from_string(value: &str, scope_hash: usize) -> Result<ClassMap, String> {
    let mut classes: ClassMap = ClassMap::default();
    let mut input = ParserInput::new(value);
    let mut parse = Parser::new(&mut input);

	classes.key_frames.scope_hash = scope_hash;
    loop {
        if parse.is_exhausted() {
            return Ok(classes);
        }

        if let Err(e) = parse_css_item(&mut classes, &mut parse, scope_hash) {
            log::error!("parse class err: {:?}", e);
        }
    }
}

pub fn parse_style_list_from_string(value: &str, scope_hash: usize) -> Result<VecDeque<Attribute>, String> {
    let mut list = VecDeque::default();
    let mut input = ParserInput::new(value);
    let mut parse = Parser::new(&mut input);

    let _ = parser_style_items(&mut parse, &mut list, scope_hash);
    Ok(list)
}

// 解析css文件中的每一项
pub fn parse_css_item<'i, 't>(context: &mut ClassMap, input: &mut Parser<'i, 't>, scope_hash: usize) -> Result<(), TokenParseError<'i>> {
    let next = input.next()?;
    match next {
        Token::Delim(r) if r == &'.' => {
            // 解析class
            let class_name = input.expect_ident()?.as_ref();
            log::trace!("class: {}", class_name);

            let class_name = match usize::from_str(&class_name[1..class_name.len()]) {
                Ok(r) => r,
                Err(_) => usize::MAX,
            };

            let start = context.attrs.len();
            input.expect_curly_bracket_block()?;
            let _ = input.parse_nested_block::<_, _, TokenErrorsInfo<'i>>(|i| {
                Ok(parser_style_items(i, &mut context.attrs, scope_hash))
            });

            if class_name != usize::MAX {
                context.classes.push(ClassItem {
                    count: context.attrs.len() - start,
                    class_name: class_name,
                });
            }
        }
        Token::AtKeyword(name) if &**name == "keyframes" => {
            // 解析keyframes
            let name = input.expect_ident()?;
            log::trace!("parse keyframes start: {:?}", name);
            let name = Atom::from(&**name);
            let key_frames = parse_key_frames(input, scope_hash)?;
            if key_frames.len() > 0 {
                context.key_frames.frames.insert(name, key_frames);
            }
        }
        ref i => {
            log::warn!("Unexpected css: {:?}", i);
            loop {
                if input.is_exhausted() {
                    return Ok(());
                }
                if let Ok(_) = input.expect_curly_bracket_block() {
                    return Ok(());
                }
            }
        }
    };

    Ok(())
}

pub fn parser_style_items<'i, 't>(input: &mut Parser<'i, 't>, arr: &mut VecDeque<Attribute>, scope_hash: usize) {
    loop {
        if let Err(e) = parse_style_item(arr, scope_hash, input) {
			if let ItemParseErrors::ValueError { .. } = e {
				log::warn!("{}", e);
			}
            end_cur_attr(input);
        } else {
            // 成功后，尝试解析一个或多个分号
            let _r = input.try_parse(|i| i.expect_semicolon());
        }
        if input.is_exhausted() {
            break;
        }
    }
}

pub fn parse_key_frames<'i, 't>(
    input: &mut Parser<'i, 't>,
	scope_hash: usize,
) -> Result<XHashMap<NotNan<f32>, VecDeque<Attribute>>, TokenParseError<'i>> {
    let mut key_frames: XHashMap<NotNan<f32>, VecDeque<Attribute>> = XHashMap::default();
    input.expect_curly_bracket_block()?;
    Ok(input.parse_nested_block::<_, _, TokenErrorsInfo<'i>>(|i| {
        loop {
            match parse_key_frame(i, scope_hash) {
                Ok((progress, attrs)) => {
                    if attrs.len() > 0 {
                        match key_frames.entry(progress) {
                            Entry::Occupied(mut r) => r.get_mut().extend(attrs),
                            Entry::Vacant(r) => {
                                r.insert(attrs);
                            }
                        };
                    }
                }
                Err(e) => {
                    if i.is_exhausted() {
                        break;
                    } else {
                        log::error!("parse_key_frames style error: {:?}", e);
                    }
                }
            }
        }
        Ok(key_frames)
    })?)
}

pub fn parse_key_frame<'i, 't>(input: &mut Parser<'i, 't>, scope_hash: usize) -> Result<(NotNan<f32>, VecDeque<Attribute>), TokenParseError<'i>> {
    let progress = parse_key_frame_progress(input)?;
    let mut attrs = VecDeque::default();
    input.expect_curly_bracket_block()?;
    if let Err(r) = input.parse_nested_block::<_, _, TokenErrorsInfo<'i>>(|i| {
        loop {
            if let Err(e) = parse_style_item(&mut attrs,  scope_hash, i) {
				if let ItemParseErrors::ValueError { .. } = e {
					log::warn!("{}", e);
				}
                end_cur_attr(i);
            } else {
                // 成功后，尝试解析一个分号
                let _r = i.try_parse(|i| i.expect_semicolon());
            }
            if i.is_exhausted() {
                break;
            }
        }
        Ok(())
    }) {
        log::error!("parse_key_frames fail, {:?}", r);
    }

    Ok((progress, attrs))
}

pub fn end_cur_attr<'i, 't>(input: &mut Parser<'i, 't>) {
    loop {
        if input.is_exhausted() {
            break;
        }
        if let Ok(_) = input.expect_semicolon() {
            break;
        }
    }
}

/// 解析KeyFrame进度
pub fn parse_key_frame_progress<'i, 't>(input: &mut Parser<'i, 't>) -> Result<NotNan<f32>, TokenParseError<'i>> {
    let location = input.current_source_location();
    let item = input.next()?;
    let r = match item {
        Token::Ident(r) => {
            if (&**r) == "from" {
                0.0
            } else if (&**r) == "to" {
                1.0
            } else {
                return Err(TokenParseError::from_expect(location, "from | to | <percentage>",item.clone()))?;
            }
        }
        Token::Percentage { unit_value, .. } => *unit_value,
        _ => return Err(TokenParseError::from_expect(location, "from | to | <percentage>",item.clone()))?
    };
    Ok(unsafe { NotNan::new_unchecked(r) })
}

fn parse_border_image_slice<'i, 't>(input: &mut Parser<'i, 't>) -> Result<BorderImageSlice, TokenParseError<'i>> {
    let r = match input.try_parse(|input| input.expect_percentage()) {
        Ok(r1) => match input.try_parse(|input| input.expect_percentage()) {
            Ok(r2) => match input.try_parse(|input| input.expect_percentage()) {
                Ok(r3) => match input.try_parse(|input| input.expect_percentage()) {
                    Ok(r4) => [r1, r2, r3, r4],
                    Err(_) => [r1, r2, r3, r2],
                },
                Err(_) => [r1, r2, r1, r2],
            },
            Err(_) => [r1, r1, r1, r1],
        },
        Err(_) => [0.0, 0.0, 0.0, 0.0],
    };

    let fill = match input.try_parse(|input| input.expect_ident_matching("fill")) {
        Ok(_) => true,
        Err(_) => false,
    };

    Ok(BorderImageSlice {
        top: match NotNan::new(r[0]) {
            Ok(r) => r,
            Err(_) => unsafe { NotNan::new_unchecked(0.0) },
        },
        right: match NotNan::new(r[1]) {
            Ok(r) => r,
            Err(_) => unsafe { NotNan::new_unchecked(0.0) },
        },
        bottom: match NotNan::new(r[2]) {
            Ok(r) => r,
            Err(_) => unsafe { NotNan::new_unchecked(0.0) },
        },
        left: match NotNan::new(r[3]) {
            Ok(r) => r,
            Err(_) => unsafe { NotNan::new_unchecked(0.0) },
        },
        fill,
    })
}

fn parse_top_right_bottom_left<'i, 't, T: StyleParse + Copy + Default>(
    input: &mut Parser<'i, 't>,
) -> Result<Rect<T>, TokenParseError<'i>> {
    let r = match input.try_parse(|input| T::parse(input)) {
        Ok(r1) => match input.try_parse(|input| T::parse(input)) {
            Ok(r2) => match input.try_parse(|input| T::parse(input)) {
                Ok(r3) => match input.try_parse(|input| T::parse(input)) {
                    Ok(r4) => Rect {
                        top: r1,
                        right: r2,
                        bottom: r3,
                        left: r4,
                    },
                    Err(_) => Rect {
                        top: r1,
                        right: r2,
                        bottom: r3,
                        left: r2,
                    },
                },
                Err(_) => Rect {
                    top: r1,
                    right: r2,
                    bottom: r1,
                    left: r2,
                },
            },
            Err(_) => Rect {
                top: r1,
                right: r1,
                bottom: r1,
                left: r1,
            },
        },
        Err(_) => Rect {
            top: T::default(),
            right: T::default(),
            bottom: T::default(),
            left: T::default(),
        },
    };
    Ok(r)
}

pub fn parse_border_radius<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<BorderRadius, TokenParseError<'i>> {
    let x = parse_top_right_bottom_left(input)?;
	let y = input.next();
	let y = if let Ok(&Token::Delim('/')) = y {
		parse_top_right_bottom_left(input)?
	} else {
		x.clone()
	};
	
    Ok(BorderRadius {
		x: [x.top, x.right, x.bottom, x.left],
		y: [y.top, y.right, y.bottom, y.left],
	})
}

fn to_four<T: Clone + Copy>(
    value: Vec<T>,
) -> [T; 4] {
	if value.len() == 1 {
		return [value[0].clone(), value[0].clone(), value[0].clone(), value[0]];
	} else if value.len() == 2 {
		return [value[0].clone(), value[1].clone(), value[0], value[1]];
	} else if value.len() == 3 {
		return [value[0], value[1].clone(), value[2], value[1]];
	} else {
		return [value[0], value[1], value[2], value[3]];
	}
}

fn parse_enable<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Enable, TokenParseError<'i>> {
    match input.expect_ident()?.as_ref() {
        "auto" => Ok(Enable::Auto),
        "none" => Ok(Enable::None),
        "visible" => Ok(Enable::Visible),
        _ => Ok(Enable::Auto),
    }
}

fn parse_bool<'i, 't>(input: &mut Parser<'i, 't>) -> Result<bool, TokenParseError<'i>> {
    match input.expect_ident()?.as_ref() {
        "true" => Ok(true),
        _ => Ok(false),
    }
}

fn parse_visibility<'i, 't>(input: &mut Parser<'i, 't>) -> Result<bool, TokenParseError<'i>> {
    match input.expect_ident()?.as_ref() {
        "hidden" => Ok(false),
        _ => Ok(true),
    }
}

fn parse_display<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Display, TokenParseError<'i>> {
    match input.expect_ident()?.as_ref() {
        "flex" => Ok(Display::Flex),
        "none" => Ok(Display::None),
        _ => Ok(Display::Flex), // 默认情况
    }
}

fn parse_overflow<'i, 't>(input: &mut Parser<'i, 't>) -> Result<bool, TokenParseError<'i>> {
    match input.expect_ident()?.as_ref() {
        "hidden" => Ok(true),
        _ => Ok(false), // 默认情况
    }
}

fn pasre_white_space<'i, 't>(input: &mut Parser<'i, 't>) -> Result<WhiteSpace, TokenParseError<'i>> {
    let location = input.current_source_location();
	let ident = input.expect_ident()?;
    let r = match ident.as_ref() {
        "normal" => WhiteSpace::Normal,
        "pre" => WhiteSpace::Pre,
        "nowrap" => WhiteSpace::Nowrap,
        "pre-wrap" => WhiteSpace::PreWrap,
        "pre-line" => WhiteSpace::PreLine,
        _ => return Err(TokenParseError::from_expect(location, "normal | pre | nowrap | pre-wrap | pre-line", Token::Ident(ident.clone())))?,
    };
    Ok(r)
}

fn parse_blend_mode<'i, 't>(input: &mut Parser<'i, 't>) -> Result<BlendMode, TokenParseError<'i>> {
    let location = input.current_source_location();
    let ident = input.expect_ident()?;
    let r = match ident.as_ref() {
        "normal" => BlendMode::Normal,
        "alpha-add" => BlendMode::AlphaAdd,
        "subtract" => BlendMode::Subtract,
        "multiply" => BlendMode::Multiply,
        "one-one" => BlendMode::OneOne,
        _ => return Err(TokenParseError::from_expect(location, "normal | alpha-add | subtract | multiply | one-one", Token::Ident(ident.clone())))?,
    };
    Ok(r)
}

pub fn parse_as_image<'i, 't>(input: &mut Parser<'i, 't>) -> Result<AsImage, TokenParseError<'i>> {
    let location = input.current_source_location();
    let ident = input.expect_ident()?;
    let r = match ident.as_ref() {
        "none" => AsImage::None,
        "advise" => AsImage::Advise,
        "force" => AsImage::Force,
        _ => return Err(TokenParseError::from_expect(location, "none | advise | force", Token::Ident(ident.clone())))?,
    };
    Ok(r)
}

fn parse_font_weight<'i, 't>(input: &mut Parser<'i, 't>) -> Result<f32, TokenParseError<'i>> {
    let location = input.current_source_location();
    let token = input.next()?;
    let r = match token {
        Token::Ident(r) => match r.as_ref() {
            "bold" => 700.0,
            _ => return Err(TokenParseError::from_expect(location, "bold | <number>", token.clone()))?,
        },
        Token::Number { value, .. } => *value,
        _ => return Err(TokenParseError::from_expect(location, "bold | <number>", token.clone()))?,
    };
    Ok(r)
}

fn parse_text_align<'i, 't>(input: &mut Parser<'i, 't>) -> Result<TextAlign, TokenParseError<'i>> {
    let location = input.current_source_location();
    let ident = input.expect_ident()?;
    let r = match ident.as_ref() {
        "left" => Ok(TextAlign::Left),
        "right" => Ok(TextAlign::Right),
        "center" => Ok(TextAlign::Center),
        "justify" => Ok(TextAlign::Justify),
		_ => return Err(TokenParseError::from_expect(location, "left | right | center | justify", Token::Ident(ident.clone())))?,
    };
    r
}

fn parse_yg_align_items<'i, 't>(input: &mut Parser<'i, 't>) -> Result<AlignItems, TokenParseError<'i>> {
    let location = input.current_source_location();
    let ident = input.expect_ident()?;
    match ident.as_ref() {
        // "auto" => Ok(AlignItems::Auto),
        "flex-start" => Ok(AlignItems::FlexStart),
        "center" => Ok(AlignItems::Center),
        "flex-end" => Ok(AlignItems::FlexEnd),
        "stretch" => Ok(AlignItems::Stretch),
        "baseline" => Ok(AlignItems::Baseline),
		_ => return Err(TokenParseError::from_expect(location, "flex-start | center | flex-end | stretch | baseline", Token::Ident(ident.clone())))?,
    }
}

fn parse_yg_align_self<'i, 't>(input: &mut Parser<'i, 't>) -> Result<AlignSelf, TokenParseError<'i>> {
    let location = input.current_source_location();
    let ident = input.expect_ident()?;
    match ident.as_ref() {
        // "auto" => Ok(AlignItems::Auto),
        "flex-start" => Ok(AlignSelf::FlexStart),
        "center" => Ok(AlignSelf::Center),
        "flex-end" => Ok(AlignSelf::FlexEnd),
        "stretch" => Ok(AlignSelf::Stretch),
        "baseline" => Ok(AlignSelf::Baseline),
        _ => return Err(TokenParseError::from_expect(location, "flex-start | center | flex-end | stretch | baseline", Token::Ident(ident.clone())))?,
    }
}

fn parse_yg_align_content<'i, 't>(input: &mut Parser<'i, 't>) -> Result<AlignContent, TokenParseError<'i>> {
    let location = input.current_source_location();
	let ident = input.expect_ident()?;
    match ident.as_ref() {
        // "auto" => Ok(AlignItems::Auto),
        "flex-start" => Ok(AlignContent::FlexStart),
        "center" => Ok(AlignContent::Center),
        "flex-end" => Ok(AlignContent::FlexEnd),
        "stretch" => Ok(AlignContent::Stretch),
        "space-between" => Ok(AlignContent::SpaceBetween),
        "space-around" => Ok(AlignContent::SpaceAround),
        _ => return Err(TokenParseError::from_expect(location, "flex-start | center | flex-end | stretch | space-between | space-around", Token::Ident(ident.clone())))?,
    }
}

fn parse_yg_direction<'i, 't>(input: &mut Parser<'i, 't>) -> Result<FlexDirection, TokenParseError<'i>> {
    let location = input.current_source_location();
	let ident = input.expect_ident()?;
    match ident.as_ref() {
        "column" => Ok(FlexDirection::Column),
        "column-reverse" => Ok(FlexDirection::ColumnReverse),
        "row" => Ok(FlexDirection::Row),
        "row-reverse" => Ok(FlexDirection::RowReverse),
        _ => return Err(TokenParseError::from_expect(location, "column | column-reverse | row | row-reverse", Token::Ident(ident.clone())))?,
    }
}

fn parse_yg_justify_content<'i, 't>(input: &mut Parser<'i, 't>) -> Result<JustifyContent, TokenParseError<'i>> {
    let location = input.current_source_location();
	let ident = input.expect_ident()?;
    match ident.as_ref() {
        "flex-start" => Ok(JustifyContent::FlexStart),
        "center" => Ok(JustifyContent::Center),
        "flex-end" => Ok(JustifyContent::FlexEnd),
        "space-between" => Ok(JustifyContent::SpaceBetween),
        "space-around" => Ok(JustifyContent::SpaceAround),
        _ => return Err(TokenParseError::from_expect(location, "flex-start | center | flex-end | space-between | space-around", Token::Ident(ident.clone())))?,
    }
}

fn parse_yg_position_type<'i, 't>(input: &mut Parser<'i, 't>) -> Result<PositionType, TokenParseError<'i>> {
    let location = input.current_source_location();
	let ident = input.expect_ident()?;
    match ident.as_ref() {
        "relative" => Ok(PositionType::Relative),
        "absolute" => Ok(PositionType::Absolute),
        _ => return Err(TokenParseError::from_expect(location, "relative | absolute", Token::Ident(ident.clone())))?,
    }
}

fn parse_yg_wrap<'i, 't>(input: &mut Parser<'i, 't>) -> Result<FlexWrap, TokenParseError<'i>> {
    let location = input.current_source_location();
	let ident = input.expect_ident()?;
    match ident.as_ref() {
        "nowrap" => Ok(FlexWrap::NoWrap),
        "wrap" => Ok(FlexWrap::Wrap),
        "wrap-reverse" => Ok(FlexWrap::WrapReverse),
        _ => return Err(TokenParseError::from_expect(location, "nowrap | wrap | wrap-reverse", Token::Ident(ident.clone())))?,
    }
}

fn parse_line_height<'i, 't>(input: &mut Parser<'i, 't>) -> Result<LineHeight, TokenParseError<'i>> {
    let location = input.current_source_location();
    let token = input.next()?;
    match token {
        Token::Ident(r) if r.as_ref() == "normal" => Ok(LineHeight::Normal),
        Token::Percentage { unit_value, .. } => Ok(LineHeight::Percent(*unit_value)),
        Token::Dimension { value, .. } => Ok(LineHeight::Length(*value)),
        Token::Number { value, .. } => Ok(LineHeight::Length(*value)),
        _ => return Err(TokenParseError::from_expect(location, "normal | <percentage> | <length>", token.clone()))?,
    }
}

pub fn parse_font_size<'i, 't>(input: &mut Parser<'i, 't>) -> Result<FontSize, TokenParseError<'i>> {
    let location = input.current_source_location();
    let token = input.next()?;
    match token {
        Token::Percentage { unit_value, .. } => Ok(FontSize::Percent(*unit_value)),
        Token::Dimension { value, .. } => Ok(FontSize::Length(*value as usize)),
        Token::Number { value, .. } => Ok(FontSize::Length(*value as usize)),
        _ => return Err(TokenParseError::from_expect(location, "<percentage> | <length>", token.clone()))?,
    }
}

pub fn parse_text_stroke<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Stroke, TokenParseError<'i>> {
    let location = input.current_source_location();
	let w: f32 = parse_len(input)?;
    Ok(Stroke {
        width: match NotNan::new(w) {
            Ok(r) => r,
            Err(_) => return Err(TokenParseError::from_expect(location, "<length>",  Token::Number { value: w, has_sign: false, int_value: None}))?,
        },
        color: parse_color(input)?,
    })
}

pub fn parse_outer_glow<'i, 't>(input: &mut Parser<'i, 't>) -> Result<OuterGlow, TokenParseError<'i>> {
    // let location = input.current_source_location();
    let mut glow = OuterGlow::default();
    let mut is_parse_color = false; // 是否已经解析了颜色
    let mut parse_len_count: u32 = 0; // 解析的长度的数量
    
    loop {
        let mut is_parse = false;
        if !is_parse_color {
            if let Ok(r) = input.try_parse(parse_color) {
                glow.color = r;
                is_parse_color = true;
                is_parse = true;
            };
        }

        if parse_len_count <= 2 {
            if let Ok(r) = input.try_parse(parse_len) {
                if parse_len_count == 0 {
                    glow.distance = r;
                }
                parse_len_count += 1;
                is_parse = true;
            };
        }

        // 本次没有解析到任何属性， 或者已经完成解析， 则跳出循环
        if !is_parse || (parse_len_count == 2 && is_parse_color) {
            break;
        }
    }
    Ok(glow)
}

fn parse_transform_origin<'i, 't>(input: &mut Parser<'i, 't>) -> Result<TransformOrigin, TokenParseError<'i>> {
    let x = parse_transform_origin1(input)?;
    Ok(TransformOrigin::XY(
        x,
        match input.try_parse(parse_transform_origin1) {
            Ok(r) => r,
            Err(_) => x,
        },
    ))
}

fn parse_transform_origin1<'i, 't>(input: &mut Parser<'i, 't>) -> Result<LengthUnit, TokenParseError<'i>> {
    let location = input.current_source_location();
    let token = input.next()?;
    match token {
        Token::Ident(r) if r.as_ref() == "center" => Ok(LengthUnit::Percent(0.5)),
        Token::Percentage { unit_value, .. } => Ok(LengthUnit::Percent(*unit_value)),
        Token::Dimension { value, .. } => Ok(LengthUnit::Pixel(*value)),
        Token::Number { value, .. } => Ok(LengthUnit::Pixel(*value)),
        _ => return Err(TokenParseError::from_expect(location, "center | <length> |<percentage> ",  token.clone()))?,
    }
}

pub fn parse_len_or_percent<'i, 't>(input: &mut Parser<'i, 't>) -> Result<LengthUnit, TokenParseError<'i>> {
    let location = input.current_source_location();
    let token = input.next()?;
    match token {
        Token::Percentage { unit_value, .. } => Ok(LengthUnit::Percent(*unit_value)),
        Token::Dimension { value, .. } => Ok(LengthUnit::Pixel(*value)),
        Token::Number { value, .. } => Ok(LengthUnit::Pixel(*value)),
        _ => return Err(TokenParseError::from_expect(location, "<length> |<percentage> ",  token.clone()))?,
    }
}

pub fn parse_number<'i, 't>(input: &mut Parser<'i, 't>) -> Result<f32, TokenParseError<'i>> {
	Ok(input.expect_number()?)
}

pub fn parse_transform<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Vec<TransformFunc>, TokenParseError<'i>> {
    let mut transforms = Vec::default();
	let error;
    loop {
		
        let r= input.try_parse(|input| {
            let location = input.current_source_location();
            let f = input.expect_function()?;
            match f.as_ref() {
                "scale" => input.parse_nested_block(|input| {
                    let x = input.expect_number()?;
                    let y = match input.expect_comma() {
                        Ok(_) => input.expect_number().unwrap_or(x),
                        Err(_) => x,
                    };
                    Ok(TransformFunc::Scale(x, y))
                }),
                "scaleX" => input.parse_nested_block(|input| Ok(TransformFunc::ScaleX(input.expect_number()?))),
                "scaleY" => input.parse_nested_block(|input| Ok(TransformFunc::ScaleY(input.expect_number()?))),
                "translate" => input.parse_nested_block(|input| {
                    let x = parse_len_or_percent(input)?;
                    input.expect_comma()?;
                    let y = parse_len_or_percent(input)?;
                    Ok(TransformFunc::Translate(x, y))
                }),
                "translateX" => input.parse_nested_block(|input| Ok(TransformFunc::TranslateX(parse_len_or_percent(input)? ))),
                "translateY" => input.parse_nested_block(|input| Ok(TransformFunc::TranslateY(parse_len_or_percent(input)? ))),
                "rotate" | "rotateZ" => input.parse_nested_block(|input| Ok(TransformFunc::RotateZ(parse_angle(input)?))),
				"rotateX" => input.parse_nested_block(|input| Ok(TransformFunc::RotateX(parse_angle(input)?))),
				"rotateY" => input.parse_nested_block(|input| Ok(TransformFunc::RotateY(parse_angle(input)?))),
                "skewX" => input.parse_nested_block(|input| Ok(TransformFunc::SkewX(parse_angle(input)?))),
                "skewY" => input.parse_nested_block(|input| Ok(TransformFunc::SkewY(parse_angle(input)?))),
                _ => return Err(TokenParseError::from_expect(location, "scale | scaleX | scaleY | translate | translateX | translateY | rotate | rotateX | rotateY | rotateZ | skewX | skewY", Token::Ident(f.clone())))?,
            }
        });
		match r {
			Ok(r) => transforms.push(r),
			Err(r) => {
				error = r;
				break;
			},
        };
    }
    if transforms.len() > 0 {
        Ok(transforms)
    } else {
		Err(error)?
    }
}

fn parse_object_fit<'i, 't>(input: &mut Parser<'i, 't>) -> Result<FitType, TokenParseError<'i>> {
    let location = input.current_source_location();
    let item = input.expect_ident()?;
    let r = match item.as_ref() {
        "contain" => FitType::Contain,
        "cover" => FitType::Cover,
        "fill" => FitType::Fill,
        "none" => FitType::None,
        "scale-down" => FitType::ScaleDown,
        _ => return Err(TokenParseError::from_expect(location, "contain | cover | fill | none | scale-down", Token::Ident(item.clone())))?,
    };
    Ok(r)
}

fn parse_image_repeat<'i, 't>(input: &mut Parser<'i, 't>) -> Result<ImageRepeat, TokenParseError<'i>> {
    let location = input.current_source_location();
    let item = input.expect_ident()?;
    let mut r = match item.as_ref() {
        "no-repeat" => ImageRepeat {
            x: ImageRepeatOption::Stretch,
            y: ImageRepeatOption::Stretch,
        },
        "repeat-x" => {
            return Ok(ImageRepeat {
                x: ImageRepeatOption::Repeat,
                y: ImageRepeatOption::Stretch,
            })
        }
        "repeat-y" => {
            return Ok(ImageRepeat {
                x: ImageRepeatOption::Stretch,
                y: ImageRepeatOption::Repeat,
            })
        }
        "space" => ImageRepeat {
            x: ImageRepeatOption::Space,
            y: ImageRepeatOption::Space,
        },
        "round" => ImageRepeat {
            x: ImageRepeatOption::Round,
            y: ImageRepeatOption::Round,
        },
        "repeat" => ImageRepeat {
            x: ImageRepeatOption::Repeat,
            y: ImageRepeatOption::Repeat,
        },
        _ => return Err(TokenParseError::from_expect(location, "no-repeat | repeat-x | repeat-y | space | round | repeat", Token::Ident(item.clone())))?,
    };

    let _ = input.try_parse::<_, _, ParseError<TokenErrorsInfo<'i>>>(|input| {
        let location = input.current_source_location();
        let item = input.expect_ident()?;
        match item.as_ref() {
            "no-repeat" => r.y = ImageRepeatOption::Stretch,
            "space" => r.y = ImageRepeatOption::Space,
            "round" => r.y = ImageRepeatOption::Round,
            "repeat" => r.y = ImageRepeatOption::Repeat,
            _ => return Err(TokenParseError::from_expect(location, "no-repeat | space | round | repeat", Token::Ident(item.clone())))?,
        }
        Ok(())
    });

    Ok(r)
}

pub fn parse_color_hex(value: &str) -> Result<CgColor, ()> {
    let value = value.as_bytes();
    match value.len() {
        8 => Ok(rgba(
            from_hex(value[0])? * 16 + from_hex(value[1])?,
            from_hex(value[2])? * 16 + from_hex(value[3])?,
            from_hex(value[4])? * 16 + from_hex(value[5])?,
            (from_hex(value[6])? * 16 + from_hex(value[7])?) as f32 / 255.0,
        )),
        6 => Ok(rgba(
            from_hex(value[0])? * 16 + from_hex(value[1])?,
            from_hex(value[2])? * 16 + from_hex(value[3])?,
            from_hex(value[4])? * 16 + from_hex(value[5])?,
            1.0,
        )),
        4 => Ok(rgba(
            from_hex(value[0])? * 17,
            from_hex(value[1])? * 17,
            from_hex(value[2])? * 17,
            (from_hex(value[3])? * 17) as f32 / 255.0,
        )),
        3 => Ok(rgba(from_hex(value[0])? * 17, from_hex(value[1])? * 17, from_hex(value[2])? * 17, 1.0)),
        _ => Err(()),
    }
}

fn rgba(red: u8, green: u8, blue: u8, alpha: f32) -> CgColor { CgColor::new(red as f32 / 255.0, green as f32 / 255.0, blue as f32 / 255.0, alpha) }

fn parser_color_stop_last<'i>(
    v: f32,
    list: &mut Vec<CgColor>,
    color_stop: &mut Vec<ColorAndPosition>,
    pre_percent: &mut f32,
    last_color: Option<CgColor>,
) -> Result<(), TokenParseError<'i>> {
    if list.len() > 0 {
        if color_stop.len() != 0 {
            let pos = (v - *pre_percent) / list.len() as f32;
            for i in 0..list.len() {
                color_stop.push(ColorAndPosition {
                    position: *pre_percent + pos * (i + 1) as f32,
                    rgba: list[i].clone(),
                });
            }
        } else {
            let pos = if list.len() == 1 {
                0.0
            } else {
                (v - *pre_percent) / (list.len() as f32 - 1.0)
            };
            for i in 0..list.len() {
                color_stop.push(ColorAndPosition {
                    position: *pre_percent + pos * i as f32,
                    rgba: list[i].clone(),
                });
            }
        }

        list.clear();
    }
    *pre_percent = v;
    if let Some(last_color) = last_color {
        color_stop.push(ColorAndPosition {
            position: v,
            rgba: last_color,
        });
    }
    Ok(())
}

fn from_hex(c: u8) -> Result<u8, ()> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err(()),
    }
}

fn trans_hsi_h(mut h: f32) -> f32 {
    if h > 180.0 {
        h = 180.0;
    } else if h < -180.0 {
        h = -180.0
    }
    h / 360.0
}

fn trans_hsi_s(mut s: f32) -> f32 {
    if s > 100.0 {
        s = 100.0;
    } else if s < -100.0 {
        s = -100.0
    }
    s / 100.0
}

fn trans_hsi_i(mut i: f32) -> f32 {
    if i > 100.0 {
        i = 100.0;
    } else if i < -100.0 {
        i = -100.0
    }
    i / 100.0
}


pub fn parse_style_item<'i, 't>(buffer: &mut VecDeque<Attribute>, scope_hash: usize, input: &mut Parser<'i, 't>) -> Result<(), ItemParseErrors<'i>> {
    let location = input.current_source_location();
	let key_token = input.next()?;
	let name = match key_token {
        Token::Semicolon => return Ok(()), // 如果是分号，直接结束本次匹配
        Token::Ident(r) => r.clone(),
        _ => return Err(ItemParseErrors::KeyError { location: location, kind: BasicParseErrorKind::UnexpectedToken(key_token.clone()) }),
    };
    
	 match parse_style_item_value(location, name.clone(), buffer, scope_hash, input) {
		Ok(_) => Ok(()),
		Err(e) => match e.error {
			TokenErrorsInfo::KeyError => Err(ItemParseErrors::KeyError { location, kind: BasicParseErrorKind::UnexpectedToken(Token::Ident(name)) }),
			_ => Err(ItemParseErrors::ValueError { attribute: name.clone(), error: e }),
		},
	}
}

pub fn parse_style_item_value<'i, 't>(location: SourceLocation, name: CowRcStr<'i>, buffer: &mut VecDeque<Attribute>, scope_hash: usize, input: &mut Parser<'i, 't>) -> Result<(), TokenParseError<'i>> {
    match name.as_ref() {
        "filter" => {
            input.expect_colon()?;
            parse_filter1(buffer, input)?;
        }
        "background-color" => {
            input.expect_colon()?;
            let ty = BackgroundColorType(Color::RGBA(parse_color(input)?));
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::BackgroundColor(ty));
        }
        "background" => {
            input.expect_colon()?;
            let ty = BackgroundColorType(parse_background(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::BackgroundColor(ty));
        }

        "border-color" => {
            input.expect_colon()?;
            let ty = BorderColorType(parse_color(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::BorderColor(ty));
        }
        "box-shadow" => {
            input.expect_colon()?;
            let ty = BoxShadowType(parse_box_shadow(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::BoxShadow(ty));
        }

        "background-image" => {
            input.expect_colon()?;
            match parse_gradient_image(input)? {
                GradientImage::Linear(gradient) => {
                    let ty = BackgroundColorType(Color::LinearGradient(gradient));
                    log::trace!("{:?}", ty);
                    buffer.push_back(Attribute::BackgroundColor(ty));
                }
                GradientImage::Url(image) => {
                    let ty = BackgroundImageType(Atom::from(image.as_ref().to_string()));
                    log::trace!("{:?}", ty);
                    buffer.push_back(Attribute::BackgroundImage(ty));
                }
            }
        }
        "image-clip" | "background-image-clip" => unsafe {
            input.expect_colon()?;
            let ty = BackgroundImageClipType(transmute::<_, NotNanRect>(parse_top_right_bottom_left::<Percentage>(input)?));
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::BackgroundImageClip(ty));
        },
        "object-fit" => {
            input.expect_colon()?;
            let ty = ObjectFitType(parse_object_fit(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::ObjectFit(ty));
        }
        "background-repeat" => {
            input.expect_colon()?;
            let ty = BackgroundRepeatType(parse_image_repeat(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::BackgroundRepeat(ty));
        }

        "border-image" => {
            input.expect_colon()?;
            let ty = BorderImageType(Atom::from(input.expect_url()?.as_ref().to_string()));
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::BorderImage(ty));
        }
        "border-image-clip" => unsafe {
            input.expect_colon()?;
            let ty = BorderImageClipType(transmute::<_, NotNanRect>(parse_top_right_bottom_left::<Percentage>(input)?));
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::BorderImageClip(ty));
        },
        "border-image-slice" => {
            input.expect_colon()?;
            let ty = BorderImageSliceType(parse_border_image_slice(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::BorderImageSlice(ty));
        }
        "border-image-repeat" => {
            input.expect_colon()?;
            let repeat = parse_image_repeat(input)?;
            let ty = BorderImageRepeatType(repeat);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::BorderImageRepeat(ty));
        }
        "mask-image" => {
            input.expect_colon()?;
            match parse_gradient_image(input)? {
                GradientImage::Linear(gradient) => {
                    let ty = MaskImageType(MaskImage::LinearGradient(gradient));
                    log::trace!("{:?}", ty);
                    buffer.push_back(Attribute::MaskImage(ty));
                }
                GradientImage::Url(image) => {
                    let ty = MaskImageType(MaskImage::Path(Atom::from(image.as_ref().to_string())));
                    log::trace!("{:?}", ty);
                    buffer.push_back(Attribute::MaskImage(ty));
                }
            }
        }
        "mask-image-clip" => unsafe {
            input.expect_colon()?;
            let ty = MaskImageClipType(transmute::<_, NotNanRect>(parse_top_right_bottom_left::<Percentage>(input)?));
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::MaskImageClip(ty));
        },
        "blend-mode" => {
            input.expect_colon()?;
            let ty = BlendModeType(parse_blend_mode(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::BlendMode(ty));
        }
        "text-gradient" => {
            input.expect_colon()?;
            let ty = ColorType(parse_background(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::Color(ty));
        }
        "color" => {
            input.expect_colon()?;
            let ty = ColorType(Color::RGBA(parse_color(input)?));
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::Color(ty));
        }
        "letter-spacing" => {
            input.expect_colon()?;
            let ty = LetterSpacingType(parse_len(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::LetterSpacing(ty));
        }
        "line-height" => {
            input.expect_colon()?;
            let ty = LineHeightType(parse_line_height(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::LineHeight(ty));
        }
        "text-align" => {
            input.expect_colon()?;
            let ty = TextAlignType(parse_text_align(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::TextAlign(ty));
        }
        "text-indent" => {
            input.expect_colon()?;
            let ty = TextIndentType(parse_len(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::TextIndent(ty));
        }
        "text-shadow" => {
            input.expect_colon()?;
            let ty = TextShadowType(parse_text_shadow(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::TextShadow(ty));
        },
		"content" => {
            input.expect_colon()?;
            let ty = TextContentType(TextContent(input.expect_string()?.to_string(), Atom::from("")));
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::TextContent(ty));
        }
        // "vertical-align" => show_attr.push(Attribute::Color( Color::RGBA(parse_color_string(value)?) )),
        "white-space" => {
            input.expect_colon()?;
            let ty = WhiteSpaceType(pasre_white_space(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::WhiteSpace(ty));
        }
        "word-spacing" => {
            input.expect_colon()?;
            let ty = WordSpacingType(parse_len(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::WordSpacing(ty));
        }

        "text-stroke" => {
            input.expect_colon()?;
            let ty = TextStrokeType(parse_text_stroke(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::TextStroke(ty));
        }
        // 文字外发光
        "text-outer-grow" => {
            input.expect_colon()?;
            let ty = TextOuterGlowType(parse_outer_glow(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::TextOuterGlow(ty));
        }

        // "font-style" => show_attr.push(Attribute::FontStyle( Color::RGBA(parse_color_string(value)?) )),
        "font-weight" => {
            input.expect_colon()?;
            let ty = FontWeightType(parse_font_weight(input)? as usize);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::FontWeight(ty));
        }
        "font-size" => {
            input.expect_colon()?;
            let ty = FontSizeType(parse_font_size(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::FontSize(ty));
        }
        "font-family" => {
            input.expect_colon()?;
            let ty = FontFamilyType(Atom::from(input.expect_ident()?.as_ref().to_string()));
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::FontFamily(ty));
        }

        "border-radius" => {
            input.expect_colon()?;
            let ty = BorderRadiusType(parse_border_radius(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::BorderRadius(ty));
        }
        "opacity" => {
            input.expect_colon()?;
            let ty = OpacityType(input.expect_number()?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::Opacity(ty));
        }
        "transform" => {
            input.expect_colon()?;
            let ty = TransformType(parse_transform(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::Transform(ty));
        }
		"translate" => {
			input.expect_colon()?;
            let ty = TranslateType(parse_mult(input, [LengthUnit::default(), LengthUnit::default()], parse_len_or_percent)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::Translate(ty));
		},
		"scale" => {
			input.expect_colon()?;
            let ty = ScaleType(parse_mult(input, [1.0, 1.0], parse_number)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::Scale(ty));
		},
		"rotate" => {
			input.expect_colon()?;
			// rotate: 90deg;
			/* x, y, or z axis name plus angle 暂不支持 */
			// rotate: x 90deg;
			/* Vector plus angle value 暂不支持 */
			// rotate: 1 1 1 90deg;
            let ty = RotateType(parse_angle(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::Rotate(ty));
		},
        "transform-origin" => {
            input.expect_colon()?;
            let ty = TransformOriginType(parse_transform_origin(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::TransformOrigin(ty));
        }
		"will-change-transform" => {
            let ty = TransformWillChangeType(true);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::TransformWillChange(ty));
        }
        "z-index" => {
            input.expect_colon()?;
            let ty = ZIndexType(input.expect_number()? as isize);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::ZIndex(ty));
        }
        "visibility" => {
            input.expect_colon()?;
            let ty = VisibilityType(parse_visibility(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::Visibility(ty));
        }
        "pointer-events" => {
            input.expect_colon()?;
            let ty = EnableType(parse_enable(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::Enable(ty));
        }
        "display" => {
            input.expect_colon()?;
            let ty = DisplayType(parse_display(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::Display(ty));
        }
        "overflow" => {
            input.expect_colon()?;
            let ty = OverflowType(parse_overflow(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::Overflow(ty));
        }
        "overflow-y" => {
            input.expect_colon()?;
            let ty = OverflowType(parse_overflow(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::Overflow(ty));
        }
        "width" => {
            input.expect_colon()?;
            let ty = WidthType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::Width(ty));
        }
        "height" => {
            input.expect_colon()?;
            let ty = HeightType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::Height(ty));
        }
        "left" => {
            input.expect_colon()?;
            let ty = PositionLeftType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::PositionLeft(ty));
        }
        "bottom" => {
            input.expect_colon()?;
            let ty = PositionBottomType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::PositionBottom(ty));
        }
        "right" => {
            input.expect_colon()?;
            let ty = PositionRightType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::PositionRight(ty));
        }
        "top" => {
            input.expect_colon()?;
            let ty = PositionTopType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::PositionTop(ty));
        }
        "margin-left" => {
            input.expect_colon()?;
            let ty = MarginLeftType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::MarginLeft(ty));
        }
        "margin-bottom" => {
            input.expect_colon()?;
            let ty = MarginBottomType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::MarginBottom(ty));
        }
        "margin-right" => {
            input.expect_colon()?;
            let ty = MarginRightType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::MarginRight(ty));
        }
        "margin-top" => {
            input.expect_colon()?;
            let ty = MarginTopType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::MarginTop(ty));
        }
        "margin" => {
            input.expect_colon()?;
            let ty = parse_top_right_bottom_left(input)?;
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::MarginTop(MarginTopType(ty.top)));
            buffer.push_back(Attribute::MarginRight(MarginRightType(ty.right)));
            buffer.push_back(Attribute::MarginBottom(MarginBottomType(ty.bottom)));
            buffer.push_back(Attribute::MarginLeft(MarginLeftType(ty.left)));
        }
        "padding-left" => {
            input.expect_colon()?;
            let ty = PaddingLeftType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::PaddingLeft(ty));
        }
        "padding-bottom" => {
            input.expect_colon()?;
            let ty = PaddingBottomType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::PaddingBottom(ty));
        }
        "padding-right" => {
            input.expect_colon()?;
            let ty = PaddingRightType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::PaddingRight(ty));
        }
        "padding-top" => {
            input.expect_colon()?;
            let ty = PaddingTopType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::PaddingTop(ty));
        }
        "padding" => {
            input.expect_colon()?;
            let ty = parse_top_right_bottom_left(input)?;
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::PaddingTop(PaddingTopType(ty.top)));
            buffer.push_back(Attribute::PaddingRight(PaddingRightType(ty.right)));
            buffer.push_back(Attribute::PaddingBottom(PaddingBottomType(ty.bottom)));
            buffer.push_back(Attribute::PaddingLeft(PaddingLeftType(ty.left)));
        }
        "border-left" => {
            input.expect_colon()?;
            let ty = BorderLeftType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::BorderLeft(ty));
        }
        "border-bottom" => {
            input.expect_colon()?;
            let ty = BorderBottomType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::BorderBottom(ty));
        }
        "border-right" => {
            input.expect_colon()?;
            let ty = BorderRightType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::BorderRight(ty));
        }
        "border-top" => {
            input.expect_colon()?;
            let ty = BorderTopType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::BorderTop(ty));
        }
        "border" => {
            input.expect_colon()?;
            let ty = parse_top_right_bottom_left(input)?;
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::BorderTop(BorderTopType(ty.top)));
            buffer.push_back(Attribute::BorderRight(BorderRightType(ty.right)));
            buffer.push_back(Attribute::BorderBottom(BorderBottomType(ty.bottom)));
            buffer.push_back(Attribute::BorderLeft(BorderLeftType(ty.left)));
        }
        "border-width" => {
            input.expect_colon()?;
            let ty = parse_top_right_bottom_left(input)?;
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::BorderTop(BorderTopType(ty.top)));
            buffer.push_back(Attribute::BorderRight(BorderRightType(ty.right)));
            buffer.push_back(Attribute::BorderBottom(BorderBottomType(ty.bottom)));
            buffer.push_back(Attribute::BorderLeft(BorderLeftType(ty.left)));
        }
        "min-width" => {
            input.expect_colon()?;
            let ty = MinWidthType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::MinWidth(ty));
        }
        "min-height" => {
            input.expect_colon()?;
            let ty = MinHeightType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::MinHeight(ty));
        }
        "max-width" => {
            input.expect_colon()?;
            let ty = MaxWidthType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::MaxWidth(ty));
        }
        "max-height" => {
            input.expect_colon()?;
            let ty = MaxHeightType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::MaxHeight(ty));
        }
        "flex-basis" => {
            input.expect_colon()?;
            let ty = FlexBasisType(Dimension::parse(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::FlexBasis(ty));
        }
        "flex-shrink" => {
            input.expect_colon()?;
            let ty = FlexShrinkType(input.expect_number()?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::FlexShrink(ty));
        }
        "flex-grow" => {
            input.expect_colon()?;
            let ty = FlexGrowType(input.expect_number()?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::FlexGrow(ty));
        }
        "position" => {
            input.expect_colon()?;
            let ty = PositionTypeType(parse_yg_position_type(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::PositionType(ty));
        }
        "flex-wrap" => {
            input.expect_colon()?;
            let ty = FlexWrapType(parse_yg_wrap(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::FlexWrap(ty));
        }
        "row-gap" => {
            input.expect_colon()?;
            let ty = RowGapType(parse_len(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::RowGap(ty));
        },
        "column-gap" => {
            input.expect_colon()?;
            let ty = ColumnGapType(parse_len(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::ColumnGap(ty));
        },
        "auto-reduce" => {
            input.expect_colon()?;
            let ty = AutoReduceType(parse_bool(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::AutoReduce(ty));
        }
        "flex-direction" => {
            input.expect_colon()?;
            let ty = FlexDirectionType(parse_yg_direction(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::FlexDirection(ty));
        }
        "align-content" => {
            input.expect_colon()?;
            let ty = AlignContentType(parse_yg_align_content(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::AlignContent(ty));
        }
        "align-items" => {
            input.expect_colon()?;
            let ty = AlignItemsType(parse_yg_align_items(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::AlignItems(ty));
        }
        "align-self" => {
            input.expect_colon()?;
            let ty = AlignSelfType(parse_yg_align_self(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::AlignSelf(ty));
        }
        "justify-content" => {
            input.expect_colon()?;
            let ty = JustifyContentType(parse_yg_justify_content(input)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::JustifyContent(ty));
        }
        "animation-name" => {
            input.expect_colon()?;
            let ty = AnimationNameType(AnimationName{ scope_hash, value: parse_comma_separated::<_, _>(input, |input| Ok(Atom::from(input.expect_ident()?.as_ref())))?});
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::AnimationName(ty));
        }
        "animation-duration" => {
            input.expect_colon()?;
            let ty = AnimationDurationType(parse_comma_separated(input, Time::parse)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::AnimationDuration(ty));
        }
        "animation-timing-function" => {
            input.expect_colon()?;
            let ty = AnimationTimingFunctionType(parse_comma_separated(input, AnimationTimingFunction::parse)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::AnimationTimingFunction(ty));
        }
        "animation-delay" => {
            input.expect_colon()?;
            let ty = AnimationDelayType(parse_comma_separated(input, Time::parse)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::AnimationDelay(ty));
        }
        "animation-iteration-count" => {
            input.expect_colon()?;
            let ty = AnimationIterationCountType(parse_comma_separated(input, IterationCount::parse)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::AnimationIterationCount(ty));
        }
        "animation-direction" => {
            input.expect_colon()?;
            let ty = AnimationDirectionType(parse_comma_separated(input, AnimationDirection::parse)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::AnimationDirection(ty));
        }
        "animation-fill-mode" => {
            input.expect_colon()?;
            let ty = AnimationFillModeType(parse_comma_separated(input, AnimationFillMode::parse)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::AnimationFillMode(ty));
        }
        "animation-play-state" => {
            input.expect_colon()?;
            let ty = AnimationPlayStateType(parse_comma_separated(input, AnimationPlayState::parse)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::AnimationPlayState(ty));
        }
        "animation" => {
            input.expect_colon()?;
            let mut animations = parse_animation(input)?;
			animations.name.scope_hash = scope_hash;
            log::trace!("{:?}", animations);
            if animations.name.value.len() > 0 {
                buffer.push_back(Attribute::AnimationName(AnimationNameType(animations.name)));
                buffer.push_back(Attribute::AnimationDuration(AnimationDurationType(animations.duration)));
                buffer.push_back(Attribute::AnimationTimingFunction(AnimationTimingFunctionType(
                    animations.timing_function,
                )));
                buffer.push_back(Attribute::AnimationIterationCount(AnimationIterationCountType(
                    animations.iteration_count,
                )));
                buffer.push_back(Attribute::AnimationDelay(AnimationDelayType(animations.delay)));
                buffer.push_back(Attribute::AnimationDirection(AnimationDirectionType(animations.direction)));
                buffer.push_back(Attribute::AnimationFillMode(AnimationFillModeType(animations.fill_mode)));
                buffer.push_back(Attribute::AnimationPlayState(AnimationPlayStateType(animations.play_state)));
            }
        },
		"transition" => {
			input.expect_colon()?;
            let transition = Transition::parse(input)?;
            log::trace!("{:?}", transition);
            if transition.property.len() > 0 {
                buffer.push_back(Attribute::TransitionProperty(TransitionPropertyType(transition.property)));
                buffer.push_back(Attribute::TransitionDuration(TransitionDurationType(transition.duration)));
				buffer.push_back(Attribute::TransitionDelay(TransitionDelayType(transition.delay)));
                buffer.push_back(Attribute::TransitionTimingFunction(TransitionTimingFunctionType(
                    transition.timing_function,
                )));
            }
		},
		"transition-property" => {
            input.expect_colon()?;
            let ty = TransitionPropertyType(parse_comma_separated(input, parse_transation_property)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::TransitionProperty(ty));
        }
        "transition-duration" => {
            input.expect_colon()?;
            let ty = TransitionDurationType(parse_comma_separated(input, Time::parse)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::TransitionDuration(ty));
        }
        "transition-timing-function" => {
            input.expect_colon()?;
            let ty = TransitionTimingFunctionType(parse_comma_separated(input, AnimationTimingFunction::parse)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::TransitionTimingFunction(ty));
        }
        "transition-delay" => {
            input.expect_colon()?;
            let ty = TransitionDelayType(parse_comma_separated(input, Time::parse)?);
            log::trace!("{:?}", ty);
            buffer.push_back(Attribute::TransitionDelay(ty));
        }

		"clip-path" => {
			input.expect_colon()?;
			let shape = BaseShape::parse(input)?;
			log::trace!("{:?}", shape);
			buffer.push_back(Attribute::ClipPath(ClipPathType(shape)));
		}
		"as-image" => {
			input.expect_colon()?;
			let as_image = parse_as_image(input)?;
			log::trace!("{:?}", as_image);
			buffer.push_back(Attribute::AsImage(AsImageType(as_image)));
		},
		"text-overflow" => {
			input.expect_colon()?;
			let text_overflow = TextOverflow::parse(input)?;
			log::trace!("{:?}", text_overflow);
			buffer.push_back(Attribute::TextOverflow(TextOverflowType(text_overflow)));
		},
		"overflow-wrap" => {
			input.expect_colon()?;
			let overflow_wrap = OverflowWrap::parse(input)?;
			log::trace!("{:?}", overflow_wrap);
			buffer.push_back(Attribute::OverflowWrap(OverflowWrapType(overflow_wrap)));
		},

        _ => {
			return Err( TokenParseError{location, error: TokenErrorsInfo::KeyError})
        }
    };
    Ok(())
}

// fixbit   all   &mut World

// fn A() {

// }

// pub fn aaa(

// 	query: ParamSet<(Query<EntityRef>, Query<&mut B>), (),   A>
// 	// 	Query<(&A, &mut B)>
// 	query: Query<EntityRef>,
// 	query1: Query<&mut B>,
// 	// Query<(&mut B)>
// 	// 	Add<(C)>,
// 	// )>,
// 	add: Add<(C)>,
// 	insert: Insert<(A, B)>
// ) {
// 	let (e, a, b) = query.get(e).unrap();
// 	query.p0().iter();
// 	add(e, (C));
// 	a, b
// 	query.p0().iter(|| {

// 	})
// 	// let r = insert((A, B));
// 	// querySet
	
// 	let mut vec: &'static mut Vec<usize> = unsafe { transmute(querySet.p0()) } ;
// 	// querySet.p1()
// 	let iter = vec.iter();
// 	vec.
// 	iter_index.next(&mut vec)
// 	let mut item = iter.next();
// 	while let Some(item) = item {

// 	}
// }


pub fn parse_animation<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Animation, TokenParseError<'i>> {
	
    let mut animations = Animation::default();
    parse_comma_separated::<_, ()>(input, |input| {
        let mut has_duration = false;
        let location = input.current_source_location();
        let mut name = Atom::from("");
        let mut duration = Time::default();
        let mut timing_function = AnimationTimingFunction::default();
        let mut iteration_count = IterationCount(1 as f32);
        let mut delay = Time::default();
        let mut direction = AnimationDirection::default();
        let mut fill_mode = AnimationFillMode::default();
        let mut play_state = AnimationPlayState::default();
        loop {
            let token = match input.next() {
                Ok(r) => r,
                Err(_r) => break,
            };

            match token {
                Token::Ident(r) => match r.as_ref() {
                    "normal" => direction = AnimationDirection::Normal,
                    "reverse" => direction = AnimationDirection::Reverse,
                    "alternate" => direction = AnimationDirection::Alternate,
                    "alternate-reverse" => direction = AnimationDirection::AlternateReverse,
					// 兼容老的gui的错误写法
					"direction" => direction = AnimationDirection::Normal,
                    "ease" => timing_function = AnimationTimingFunction::CubicBezier(0.25, 0.1, 0.25, 1.0),
                    "ease-in" => timing_function = AnimationTimingFunction::CubicBezier(0.42, 0.0, 1.0, 1.0),
                    "ease-out" => timing_function = AnimationTimingFunction::CubicBezier(0.0, 0.0, 0.58, 1.0),
                    "ease-in-out" => timing_function = AnimationTimingFunction::CubicBezier(0.42, 0.0, 0.58, 1.0),
                    "linear" => timing_function = AnimationTimingFunction::Linear,
					"step" => timing_function = AnimationTimingFunction::Step(1, EStepMode::JumpEnd), // 兼容曾经不规范的写法，移除？TODO
                    "step-start" => timing_function = AnimationTimingFunction::Step(1, EStepMode::JumpStart),
                    "step-end" => timing_function = AnimationTimingFunction::Step(1, EStepMode::JumpEnd),
                    "none" => fill_mode = AnimationFillMode::None,
                    "forwards" => fill_mode = AnimationFillMode::Forwards,
                    "backwards" => fill_mode = AnimationFillMode::Backwards,
                    "both" => fill_mode = AnimationFillMode::Both,
                    "paused" => play_state = AnimationPlayState::Paused,
                    "running" => play_state = AnimationPlayState::Running,
					"infinite" => iteration_count = IterationCount( f32::INFINITY),
                    ref name_str => {
                        if name.as_ref() != "" {
							continue; // 暂时不支持声明多个动画
                            // return Err(TokenParseError::from_message(location, format!("animation name is multiple, {} and {}", name.as_str(), name_str)));
                        } else {
                            name = Atom::from(*name_str);
                        }
                    }
                },
                Token::Dimension { value, unit, .. } => {
                    let time = if unit.as_ref() == "s" {
                        Time((value * 1000.0) as usize)
                    } else if unit.as_ref() == "ms" {
                        Time(*value as usize)
                    } else {
                        return Err(TokenParseError::from_expect(location, "<time>", token.clone()));
                    };
                    if has_duration {
                        delay = time;
                    } else {
                        duration = time;
                        has_duration = true;
                    }
                }
                Token::Function(name) => {
					let name = unsafe{transmute(name.clone())};
					timing_function = parse_timing_function(input, name)?;
				},
				// 支持老版本gui的写法， 小于0表示无穷次迭代
                Token::Number { value, .. } => if *value < 0.0 {
					iteration_count = IterationCount( f32::INFINITY);
				} else {
					iteration_count = IterationCount(*value)
				},
                _ => break, // 可能是分号，在这里结束解析
            };
        }

		animations.name.value.push(name.clone());
		animations.duration.push(duration);
		animations.timing_function.push(timing_function.clone());
		animations.iteration_count.push(iteration_count);
		animations.delay.push(delay);
		animations.direction.push(direction);
		animations.fill_mode.push(fill_mode.clone());
		animations.play_state.push(play_state.clone());
      
        Ok(())
    })?;
    Ok(animations)
}

impl StyleParse for Transition {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, TokenParseError<'i>> {
		let mut transition = Transition::default();
		parse_comma_separated::<_, ()>(input, |input| {
			let mut has_duration = false;
			let location = input.current_source_location();
			let mut property = 0; // 属性id
			let mut duration = Time::default();
			let mut timing_function = AnimationTimingFunction::default();
			let mut delay = Time::default();
			loop {
				let token = match input.next() {
					Ok(r) => r,
					Err(_r) => break,
				};

				log::warn!("token===={:?}", token);

				
				match token {
					Token::Ident(r) => {
						match parse_transation_property1(r) {
							Ok(r) => property = r,
							_ => {
								match r.as_ref() {
									"ease" => timing_function = AnimationTimingFunction::CubicBezier(0.25, 0.1, 0.25, 1.0),
									"ease-in" => timing_function = AnimationTimingFunction::CubicBezier(0.42, 0.0, 1.0, 1.0),
									"ease-out" => timing_function = AnimationTimingFunction::CubicBezier(0.0, 0.0, 0.58, 1.0),
									"ease-in-out" => timing_function = AnimationTimingFunction::CubicBezier(0.42, 0.0, 0.58, 1.0),
									"linear" => timing_function = AnimationTimingFunction::Linear,
									"step" => timing_function = AnimationTimingFunction::Step(1, EStepMode::JumpEnd), // 兼容曾经不规范的写法，移除？TODO
									"step-start" => timing_function = AnimationTimingFunction::Step(1, EStepMode::JumpStart),
									"step-end" => timing_function = AnimationTimingFunction::Step(1, EStepMode::JumpEnd),
		
									_ => (), // 其他属性忽略
								}
							}
						}
					},
					Token::Dimension { value, unit, .. } => {						let time = if unit.as_ref() == "s" {
							Time((value * 1000.0) as usize)
						} else if unit.as_ref() == "ms" {
							Time(*value as usize)
						} else {
							return Err(TokenParseError::from_expect(location, "<time>", token.clone()));
						};
						if has_duration {
							delay = time;
						} else {
							duration = time;
							has_duration = true;
						}
					}
					Token::Function(name) => {
						let name = unsafe{transmute(name.clone())};
						timing_function = parse_timing_function(input, name)?;
					}
					_ => break, // 可能是分号，在这里结束解析
				};
			}

			transition.property.push(property);
			transition.duration.push(duration);
			transition.timing_function.push(timing_function);
			transition.delay.push(delay);
			
			Ok(())
		})?;
		Ok(transition)
    }
}

// 解析timing_function的函数式写法
fn parse_timing_function<'a, 'i, 't>(input: &mut Parser<'i, 't>, name: CowRcStr<'static>) -> Result<AnimationTimingFunction, TokenParseError<'i>> {
	Ok(match name.as_ref() {
		"cubic-bezier" => input.parse_nested_block(|input| {
			Ok(AnimationTimingFunction::CubicBezier(
				input.expect_number()?,
				{
					input.expect_comma()?;
					input.expect_number()?
				},
				{
					input.expect_comma()?;
					input.expect_number()?
				},
				{
					input.expect_comma()?;
					input.expect_number()?
				},
			))
		})?,
		"linear" => AnimationTimingFunction::Linear,
		"steps" => input.parse_nested_block::<_, _, TokenErrorsInfo<'i>>(|input| {
			let location = input.current_source_location();
			Ok(AnimationTimingFunction::Step(input.expect_number()? as usize, {
				if let Ok(_r) = input.expect_comma() {
					let p = input.expect_ident()?;
					match p.as_ref() {
						"jump-start" | "start" => EStepMode::JumpStart,
						"jump-end" | "end" => EStepMode::JumpEnd,
						"jump-none" => EStepMode::JumpNone,
						"jump-both" => EStepMode::JumpEnd,
						_ => return Err(TokenParseError::from_expect(location, "jump-start | start | jump-end | end | jump-none | jump-both", Token::Ident(p.clone())))?,
					}
				} else {
					EStepMode::JumpStart
				}
			}))
		})?,
		_ => AnimationTimingFunction::Linear,
	})
}

pub fn parse_transation_property<'a, 'i, 't>(input: &mut Parser<'i, 't>) -> Result<usize, TokenParseError<'i>> {
	let location = input.current_source_location();
	let token = input.expect_ident()?;

	match parse_transation_property1(token) {
		Ok(r) => return Ok(r),
		Err(_) => return Err(TokenParseError::from_expect(location, "<transation_property>", Token::Ident(token.clone()))),
	};
}

pub fn parse_transation_property1<'a, 'i, 't>(name: &CowRcStr<'a>) -> Result<usize, ()> {
	let r = match name.as_ref() {
		"all" => std::usize::MAX,
		"background-repeat" => StyleType::BackgroundRepeat as usize,
		"color" => StyleType::Color as usize,
		"background-image-clip" => StyleType::BackgroundImageClip as usize,
		// 兼容老的gui的错误写法
		"background-color" => StyleType::BackgroundColor as usize,
		"border-color" => StyleType::BorderColor as usize,
		"hsi" => StyleType::Hsi as usize,
		"blur" => StyleType::Blur as usize,
		"transform" => StyleType::Transform as usize,
		"border-radius" => StyleType::BorderRadius as usize,
		"width" => StyleType::Width as usize,
		"height" => StyleType::Height as usize,
		"margin" => StyleType::MarginTop as usize | StyleType::MarginRight as usize | StyleType::MarginBottom as usize | StyleType::MarginLeft as usize,
		"margin-top" => StyleType::MarginTop as usize,
		"margin-right" => StyleType::MarginRight as usize,
		"margin-bottom" => StyleType::MarginBottom as usize,
		"margin-left" => StyleType::MarginLeft as usize,
		"padding" => StyleType::PaddingTop as usize | StyleType::PaddingRight as usize | StyleType::PaddingBottom as usize | StyleType::PaddingLeft as usize,
		"padding-top" => StyleType::PaddingTop as usize,
		"padding-right" => StyleType::PaddingRight as usize,
		"padding-bottom" => StyleType::PaddingBottom as usize,
		"padding-left" => StyleType::PaddingLeft as usize,
		"border-width" => StyleType::BorderTop as usize | StyleType::BorderRight as usize | StyleType::BorderBottom as usize | StyleType::BorderLeft as usize,
		"border-top" => StyleType::BorderTop as usize,
		"border-right" => StyleType::BorderRight as usize,
		"border-bottom" => StyleType::BorderBottom as usize,
		"border-left" => StyleType::BorderLeft as usize,
		"top" => StyleType::PositionTop as usize,
		"right" => StyleType::PositionRight as usize,
		"bottom" => StyleType::PositionBottom as usize,
		"left" => StyleType::PositionLeft as usize,
		"min-width" => StyleType::MinWidth as usize,
		"min-height" => StyleType::MinHeight as usize,
		"max-width" => StyleType::MaxWidth as usize,
		"max-height" => StyleType::MaxHeight as usize,
		"opacity" => StyleType::Opacity as usize,
		"tanslate" => StyleType::Translate as usize,
		"scale" => StyleType::Scale as usize,
		"rotate" => StyleType::Rotate as usize,

		_ => return Err(()), // 其他属性忽略
	};
	Ok(r)
}


pub trait StyleParse: Sized {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, TokenParseError<'i>>;
}

impl StyleParse for Dimension {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, TokenParseError<'i>> {
        if input.try_parse(|i| i.expect_ident_matching("auto")).is_ok() {
            return Ok(Dimension::Auto);
        }

        let location = input.current_source_location();
        let token = input.next()?;
        let dimension = match *token {
            Token::Dimension { value, ref unit, .. } if unit.as_ref() == "px" => Dimension::Points(value),
            Token::Percentage { unit_value, .. } => Dimension::Percent(unit_value),
            Token::Number { value, .. } => Dimension::Points(value),
            _ => return Err(TokenParseError::from_expect(location, "<length> | <percentage>", token.clone()))?,
        };
        Ok(dimension)
    }
}

impl StyleParse for LengthUnit {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, TokenParseError<'i>> {
        let location = input.current_source_location();
		let token = input.next()?;
		match token {
			Token::Percentage { unit_value, .. } => Ok(LengthUnit::Percent(*unit_value)),
			Token::Dimension { value, .. } => Ok(LengthUnit::Pixel(*value)),
			Token::Number { value, .. } => Ok(LengthUnit::Pixel(*value)),
			_ => return Err(TokenParseError::from_expect(location, "<length> | <percentage>", token.clone()))?,
		}
    }
}


impl StyleParse for IterationCount {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, TokenParseError<'i>> {
        let location = input.current_source_location();
        let r = match input.next()? {
            Token::Ident(r) if r.as_ref() == "infinite" => f32::INFINITY,
			// 支持老版本gui的写法， 小于0表示无穷次迭代
            Token::Number { value, .. } => if *value < 0.0 {
				f32::INFINITY
			} else {
				*value
			},
            token => return Err(TokenParseError::from_expect(location, "infinite | <number>", token.clone()))?,
        };
        Ok(IterationCount(r))
    }
}

/// 解析由逗号分割的列表，结果存放在SmallVec中
pub fn parse_comma_separated<'i, 't, F, T>(input: &mut Parser<'i, 't>, mut parse_one: F) -> Result<SmallVec<[T; 1]>, TokenParseError<'i>>
where
    F: for<'tt> FnMut(&mut Parser<'i, 'tt>) -> Result<T, TokenParseError<'i>>,
{

	let mut values = SmallVec::with_capacity(1);
	let mut f = move |input: &mut Parser<'i, '_>| {
		let location = input.current_source_location();
		match parse_one(input) {
			Ok(r) => Ok(r),
			Err(e) => Err(ParseError {kind: ParseErrorKind::Custom(e), location}),
		}
	};
	loop {
		input.skip_whitespace(); // Unnecessary for correctness, but may help try() in parse_one rewind less.
		match input.parse_until_before(Delimiter::Comma | Delimiter::Semicolon, &mut f) {
			Ok(r) => values.push(r),
			Err(e) => {
				match e.kind {
					ParseErrorKind::Basic(r) => return Err(TokenParseError {
						location: e.location,
						error: TokenErrorsInfo::BaseParseError(r)
					}),
					ParseErrorKind::Custom(r) => return Err(r),
				}
			},
		};
		match input.next() {
			Err(_) | Ok(&Token::Semicolon) => return Ok(values),
			Ok(&Token::Comma) => continue,
			Ok(_) => unreachable!(),
		}
	}
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Percentage(pub f32);

impl StyleParse for Percentage {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, TokenParseError<'i>> {
        Ok(Percentage(input.expect_percentage()?))
    }
}

impl StyleParse for AnimationDirection {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, TokenParseError<'i>> {
        let location = input.current_source_location();
        let p = input.expect_ident()?;
        match p.as_ref() {
            // "auto" => Ok(AlignItems::Auto),
            "normal" => Ok(AnimationDirection::Normal),
            "reverse" => Ok(AnimationDirection::Reverse),
            "alternate" => Ok(AnimationDirection::Alternate),
            "alternate-reverse" => Ok(AnimationDirection::AlternateReverse),
			// 兼容老的gui的错误写法
			"direction" => Ok(AnimationDirection::Normal),
            _ => return Err(TokenParseError::from_expect(location, "normal | reverse | alternate | alternate-reverse | direction", Token::Ident(p.clone())))?,
        }
    }
}

impl StyleParse for Time {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, TokenParseError<'i>> {
        let location = input.current_source_location();
        let token = input.next()?;
        if let Token::Dimension { value, unit, .. } = token {
            Ok(if unit.as_ref() == "s" {
                Self((value * 1000.0) as usize)
            } else if unit.as_ref() == "ms" {
                Self(*value as usize)
            } else {
                return Err(TokenParseError::from_expect(location, "<time>", token.clone()))?;
            })
        } else {
            return Err(TokenParseError::from_expect(location, "<time>", token.clone()))?;
        }
    }
}

impl StyleParse for AnimationTimingFunction {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, TokenParseError<'i>> {
        let location = input.current_source_location();
        let token = input.next()?;

        match token {
            Token::Ident(func_name) => match func_name.as_ref() {
                "ease" => Ok(AnimationTimingFunction::CubicBezier(0.25, 0.1, 0.25, 1.0)),
                "ease-in" => Ok(AnimationTimingFunction::CubicBezier(0.42, 0.0, 1.0, 1.0)),
                "ease-out" => Ok(AnimationTimingFunction::CubicBezier(0.0, 0.0, 0.58, 1.0)),
                "ease-in-out" => Ok(AnimationTimingFunction::CubicBezier(0.42, 0.0, 0.58, 1.0)),
                "linear" => Ok(AnimationTimingFunction::Linear),
                "step-start" => Ok(AnimationTimingFunction::Step(1, EStepMode::JumpStart)),
                "step-end" => Ok(AnimationTimingFunction::Step(1, EStepMode::JumpEnd)),
                _ => return Err(TokenParseError::from_expect(location, "ease | ease-in | ease-out | ease-in-out | linear | step-start | step-end | cubic-bezier(...) | steps | steps(...)", token.clone()))?
            },
            Token::Function(name) => {
                match name.as_ref() {
                    "cubic-bezier" => Ok(input.parse_nested_block(|input| {
                        Ok(AnimationTimingFunction::CubicBezier(
                            input.expect_number()?,
                            {
                                input.expect_comma()?;
                                input.expect_number()?
                            },
                            {
                                input.expect_comma()?;
                                input.expect_number()?
                            },
                            {
                                input.expect_comma()?;
                                input.expect_number()?
                            },
                        ))
                    })?),
                    "linear" => {
                        input.parse_nested_block(|_input| {
							Ok(())
                        })?;
						Ok(AnimationTimingFunction::Linear)
                        // TODO
                        
                    }
                    "steps" => Ok(input.parse_nested_block(|input| {
                        let location = input.current_source_location();
                        Ok(AnimationTimingFunction::Step(input.expect_number()? as usize, {
                            if let Ok(_r) = input.expect_comma() {
                                let p = input.expect_ident()?;
                                match p.as_ref() {
                                    "jump-start" | "start" => EStepMode::JumpStart,
                                    "jump-end" | "end" => EStepMode::JumpEnd,
                                    "jump-none" => EStepMode::JumpNone,
                                    "jump-both" => EStepMode::JumpEnd,
                                    _ => return Err(TokenParseError::from_expect(location, "jump-start | start | jump-end | end | jump-none | jump-both", Token::Ident(p.clone())))?,
                                }
                            } else {
                                EStepMode::JumpStart
                            }
                        }))
                    })?),
                    _ => return Err(TokenParseError::from_expect(location, "ease | ease-in | ease-out | ease-in-out | linear | step-start | step-end | cubic-bezier(...) | steps | steps(...)", token.clone()))?
                }
            }
            _ => return Err(TokenParseError::from_expect(location, "ease | ease-in | ease-out | ease-in-out | linear | step-start | step-end | cubic-bezier(...) | steps | steps(...)", token.clone()))?
        }
    }
}

impl StyleParse for AnimationFillMode {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, TokenParseError<'i>> {
        let location = input.current_source_location();
		let ident = input.expect_ident()?;
        match ident.as_ref() {
            // "auto" => Ok(AlignItems::Auto),
            "none" => Ok(AnimationFillMode::None),
            "forwards" => Ok(AnimationFillMode::Forwards),
            "backwards" => Ok(AnimationFillMode::Backwards),
            "both" => Ok(AnimationFillMode::Both),
            _ => return Err(TokenParseError::from_expect(location, "none | forwards | backwards | both", Token::Ident(ident.clone())))?
        }
    }
}


impl StyleParse for AnimationPlayState {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, TokenParseError<'i>> {
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        match ident.as_ref() {
            // "auto" => Ok(AlignItems::Auto),
            "paused" => Ok(AnimationPlayState::Paused),
            "running" => Ok(AnimationPlayState::Running),
            _ => return Err(TokenParseError::from_expect(location, "paused | running", Token::Ident(ident.clone())))?
        }
    }
}

impl StyleParse for TextOverflow {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, TokenParseError<'i>> {
		let r = input.try_parse::<_, _, TokenParseError>(|input| {
			let location = input.current_source_location();
			let ident = input.expect_ident()?;
			match ident.as_ref() {
				"clip" => Ok(TextOverflow::Clip),
				"ellipsis" => Ok(TextOverflow::Ellipsis),
				_ => return Err(TokenParseError::from_expect(location, "clip | ellipsis | <string>", Token::Ident(ident.clone())))?,
			}
		});
		if r.is_ok() {
			return r;
		}

       let location = input.current_source_location();
		let next = input.next()?;
		match next {
			Token::QuotedString(r) => return Ok(TextOverflow::Custom(r.to_string())),
			_ => return Err(TokenParseError::from_expect(location, "clip | ellipsis | <string>", next.clone()))?
		};
    }
}

impl StyleParse for OverflowWrap {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, TokenParseError<'i>> {
		let location = input.current_source_location();
		let ident = input.expect_ident()?;
		match ident.as_ref() {
			"normal" => Ok(OverflowWrap::Normal),
			"anywhere" => Ok(OverflowWrap::Anywhere),
			"break-word" => Ok(OverflowWrap::BreakWord),
			_ => return Err(TokenParseError::from_expect(location, "normal | anywhere | break-word", Token::Ident(ident.clone())))?,
		}
    }
}


impl StyleParse for BaseShape {
    fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, TokenParseError<'i>> {
        let location = input.current_source_location();
		let func_name = input.expect_function()?;
		match func_name.as_ref() {
			"inset" => Ok(input.parse_nested_block(|input| {
				let mut rect = vec![parse_len_or_percent(input)?];
				while let Ok(r) = input.try_parse(|input| {parse_len_or_percent(input)})  {
					rect.push(r);
				}
				let radius = input.try_parse::<_, _, TokenParseError>(|input| {
					let location1 = input.current_source_location();
					let ident = input.expect_ident()?;
					if ident.as_ref() == "round" {
						Ok(parse_border_radius(input)?)
					} else {
						Err(TokenParseError::from_expect(location1, "round", Token::Ident(ident.clone())))?
					}
				});
				Ok(BaseShape::Inset { rect_box: to_four(rect), border_radius: match radius {
					Ok(r) => r,
					_ => BorderRadius::default(),
				} })
			})?),
			"circle" => Ok(input.parse_nested_block(|input| {
				let radius = parse_len_or_percent(input)?;
				let center = parse_center(input);
				Ok(BaseShape::Circle { radius, center })
			})?),
			"ellipse" => Ok(input.parse_nested_block(|input| {
				let rx = parse_len_or_percent(input)?;
				let ry = parse_len_or_percent(input)?;
				let center = parse_center(input);
				Ok(BaseShape::Ellipse { rx, ry, center })
			})?),
			"sector" => Ok(input.parse_nested_block(|input| {
				let rotate = parse_angle(input)?/180.0 * 3.1415926535;
				let angle = parse_angle(input)?/180.0 * 3.1415926535;
				let radius = parse_len_or_percent(input)?;
				let center = parse_center(input);
				Ok(BaseShape::Sector { rotate, angle, radius, center: center })
			})?),
			_ => Err(TokenParseError::from_expect(location, "inset | circle | ellipse | sector", Token::Ident(func_name.clone())))?
		}
    }
}

pub fn parse_mult<'i, 't, const C: usize, O: Default + Copy, F: Fn(&mut Parser<'i, 't>) -> Result<O, TokenParseError<'i>> >(input: &mut Parser<'i, 't>, mut arr: [O;C], f: F) -> Result<[O;C], TokenParseError<'i>> {
	let mut i = 0;
	while i < C {
		if let Ok(r) = input.try_parse(|input| {f(input)}) {
			arr[i] = r;
		} else {
			break;
		}
		i += 1;
	}
	if C > 1 && i <= 1 {
		arr[1] = arr[0];
	}
	if C > 2 && i <= 2 {
		arr[2] = arr[0];
	}
	if C > 3 && i <= 3 {
		arr[3] = arr[1];
	}
	Ok(arr)
}

fn parse_center<'i, 't>(input: &mut Parser<'i, 't>) -> Center {
    let mut center = Center {x: LengthUnit::Percent(0.5), y: LengthUnit::Percent(0.5)};
	let _ = input.try_parse::<_, _, TokenParseError>(|input| {
		let location1 = input.current_source_location();
		let ident = input.expect_ident()?;
		if ident.as_ref() == "at" {
			center.x = parse_len_or_percent(input)?;
			if let Ok(r) = parse_len_or_percent(input) {
				center.y = r;
			};
			Ok(())
		} else {
			Err(TokenParseError::from_expect(location1, "at", Token::Ident(ident.clone())))?
		}
	});
	center
}

pub fn parse_len<'i, 't>(input: &mut Parser<'i, 't>) -> Result<f32, TokenParseError<'i>> {
    let location = input.current_source_location();
    let token = input.next()?;
    let dimension = match *token {
        Token::Dimension { value, ref unit, .. } if unit.as_ref() == "px" => value,
        Token::Number { value, .. } => value,
        _ => return Err(TokenParseError::from_expect(location, "<length>", token.clone()))?
    };
    Ok(dimension)
}

fn parse_filter1<'i, 't>(buffer: &mut VecDeque<Attribute>, input: &mut Parser<'i, 't>) -> Result<(), TokenParseError<'i>> {
    let mut hah_hsi = false;
    let mut hsi = Hsi {
        hue_rotate: 0.0,
        saturate: 0.0,
        bright_ness: 0.0,
    };
    loop {
        let location = input.current_source_location();
        let function = match input.expect_function() {
            Ok(f) => f.clone(),
            Err(_e) => break,
        };


        input.parse_nested_block(|i| {
            match function.as_ref() {
                "blur" => {
                    let ty = BlurType(i.try_parse(|i|  {
						parse_len(i)
                    })?);
                    buffer.push_back(Attribute::Blur(ty));
                }
                "hue-rotate" => {
                    let r = i.try_parse(|i| parse_angle(i))?;
                    hsi.hue_rotate = if r > 180.0 { r - 360.0 } else { r };
                    hah_hsi = true;
                }
                "saturate" => {
                    hsi.saturate = i.try_parse(|i| i.expect_percentage())? * 100.0 - 100.0;
                    hah_hsi = true;
                }
                "brightness" => {
                    hsi.bright_ness = i.try_parse(|i| i.expect_percentage())? * 100.0 - 100.0;
                    hah_hsi = true;
                }
                "grayscale" => {
                    hsi.saturate = -i.try_parse(|i| i.expect_percentage())? * 100.0;
                    hah_hsi = true;
                }
                "hsi" => {
                    i.try_parse(|i| {
                        i.skip_whitespace();
                        i.parse_until_before::<_, _, TokenErrorsInfo<'i>>(Delimiter::Comma, |i| {
                            hsi.hue_rotate = trans_hsi_h(i.expect_number()?);
                            Ok(())
                        })?;
						let location1 = i.current_source_location();
                        match i.next() {
                            Ok(&Token::Comma) => (),
							Ok(r) => return Err(TokenParseError::from_expect(location1, ",", r.clone())),
                            _ => return Err(TokenParseError::from_expect_but_end(location1, ","))?,
                        }
                        i.skip_whitespace();
                        i.parse_until_before::<_, _, TokenErrorsInfo<'i>>(Delimiter::Comma, |i| {
                            hsi.saturate = trans_hsi_s(i.expect_number()?);
                            Ok(())
                        })?;
						let location1 = i.current_source_location();
                        match i.next() {
                            Ok(&Token::Comma) => (),
                            Ok(r) => return Err(TokenParseError::from_expect(location1, ",", r.clone()))?,
                            _ => return Err(TokenParseError::from_expect_but_end(location1, ","))?,
                        }
                        i.skip_whitespace();
                        i.parse_until_before::<_, _, TokenErrorsInfo<'i>>(Delimiter::Comma, |i| {
                            hsi.bright_ness = trans_hsi_i(i.expect_number()?);
                            Ok(())
                        })?;
						let location1 = i.current_source_location();
                        match i.next() {
                            Ok(&Token::Comma) | Err(_) => (),
                            Ok(r) => return Err(TokenParseError::from_expect(location1, ",", r.clone()))?,
                        }
                        hah_hsi = true;
                        Ok(())
                    })?;
                }
                _ => return Err(TokenParseError::from_expect(location, "blur | hue-rotate | saturate | brightness | grayscale | hsi", Token::Ident(function.clone())))?,
            };
            Ok(())
        })?;
    }

    if hah_hsi {
        let ty = HsiType(hsi);
        log::trace!("{:?}", ty);
        buffer.push_back(Attribute::Hsi(ty));
    }

    Ok(())
}

pub fn parse_color<'i, 't>(input: &mut Parser<'i, 't>) -> Result<CgColor, TokenParseError<'i>> {
    let location = input.current_source_location();
    let token = input.next()?;
    match *token {
        Token::Hash(ref value) | Token::IDHash(ref value) if let Ok(r) = parse_color_hex(value.as_ref()) => Ok(r),
        Token::Ident(ref value) if let Ok(r) = parse_color_keyword(value.as_ref()) => Ok(r),
        Token::Function(ref name) => {
			let n = name.clone();
            Ok(input.parse_nested_block(|input| Ok(parse_color_function(location, n, input)?))?)
        }
        _ => return Err(TokenParseError::from_expect(location, "<color>", token.clone())),
    }
}

fn parse_background<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Color, TokenParseError<'i>> {
    let location = input.current_source_location();
    let function = input.expect_function()?;
    match function.as_ref() {
        "linear-gradient" => Ok(Color::LinearGradient(input.parse_nested_block(|input: &mut Parser| {Ok(parse_linear(input)?)})?)),
        _ => return Err(TokenParseError::from_expect(location, "linear-gradient(...)", Token::Ident(function.clone()))),
    }
}

fn parse_linear<'i, 't>(input: &mut Parser<'i, 't>) -> Result<LinearGradientColor, TokenParseError<'i>> {
    let direction = if let Ok(d) = input.try_parse(|i| parse_angle(i)) {
        input.expect_comma()?;
        d - 90.0
    } else {
        0.0
    };

    Ok(LinearGradientColor {
        direction,
        list: parse_stops(input)?,
    })
}

fn parse_stops<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Vec<ColorAndPosition>, TokenParseError<'i>> {
    let mut list = Vec::new();
    let mut color_stop = Vec::new();
    let mut pre_percent = 0.0;

    loop {
        if let Err(e) = parse_stop_item(&mut list, &mut color_stop, &mut pre_percent, input) {
            log::error!("parse_stops fail: {:?}", e);
        }
        match input.next() {
            Ok(&Token::Comma) => continue,
            _ => break,
        }
    }
    parser_color_stop_last(1.0, &mut list, &mut color_stop, &mut pre_percent, None)?;
    Ok(color_stop)
}

fn parse_stop_item<'i, 't>(
    list: &mut Vec<CgColor>,
    color_stop: &mut Vec<ColorAndPosition>,
    pre_percent: &mut f32,
    input: &mut Parser<'i, 't>,
) -> Result<(), TokenParseError<'i>> {
    let pos = input.try_parse(|i| i.expect_percentage());
    let color = parse_color(input)?;

    if let Ok(v) = pos {
        parser_color_stop_last(v, list, color_stop, pre_percent, Some(color))?;
    } else {
		let pos = input.try_parse(|i| i.expect_percentage());
		if let Ok(v) = pos {
			parser_color_stop_last(v, list, color_stop, pre_percent, Some(color))?;
		} else {
			list.push(color);
		}
    }

    Ok(())
}

pub fn parse_text_shadow<'i, 't>(input: &mut Parser<'i, 't>) -> Result<SmallVec<[TextShadow; 1]>, TokenParseError<'i>> {
    let mut arr = SmallVec::default();
	let mut err;
    loop {
        err = parse_text_shadow_item(input, &mut arr);

        match input.next() {
            Ok(&Token::Comma) => continue,
            _ => break,
        }
    }
    if arr.len() > 0 {
        Ok(arr)
    } else {
		return match err {
			Ok(_) => unreachable!(),
			Err(e) => Err(e),
		}
    }
}

pub fn parse_text_shadow_item<'i, 't>(
    input: &mut Parser<'i, 't>,
    arr: &mut SmallVec<[TextShadow; 1]>,
) -> Result<(), TokenParseError<'i>> {
    let mut color = input.try_parse(parse_color);
    let h = input.try_parse(parse_len)?;
    let v = input.try_parse(parse_len)?;
    let blur = input.try_parse(|i| parse_len(i));
    if let Err(_) = color {
        color = input.try_parse(parse_color);
    }
    arr.push(TextShadow {
        h,
        v,
        blur: blur.unwrap_or(0.0),
        color: color.unwrap_or(CgColor::new(0.0, 0.0, 0.0, 1.0)),
    });
    Ok(())
}

fn parse_box_shadow<'i, 't>(input: &mut Parser<'i, 't>) -> Result<BoxShadow, TokenParseError<'i>> {
    Ok(input.parse_until_before(Delimiter::Comma, |i| {
        let h = parse_len(i)?;
        let v = parse_len(i)?;
        let blur = i.try_parse(|i| parse_len(i));
        let spread = i.try_parse(|i| parse_len(i));
        let color = i.try_parse(parse_color);
        Ok(BoxShadow {
            h,
            v,
            spread: spread.unwrap_or(0.0),
            blur: blur.unwrap_or(0.0),
            color: color.unwrap_or(CgColor::new(0.0, 0.0, 0.0, 1.0)),
        })
    })?)
}

pub enum GradientImage<'a> {
    Linear(LinearGradientColor),
    Url(CowRcStr<'a>),
}

fn parse_gradient_image<'i, 't>(input: &mut Parser<'i, 't>) -> Result<GradientImage<'i>, TokenParseError<'i>> {
    let location = input.current_source_location();
    let token = input.next()?;

    match token {
        Token::UnquotedUrl(ref value) => Ok(GradientImage::Url(value.clone())),
        Token::Function(ref name) => {
            if name.eq_ignore_ascii_case("url") {
                Ok(input.parse_nested_block(|input| input.expect_string().map_err(Into::into).map(|s| GradientImage::Url(s.clone())))?)
            } else if name.eq_ignore_ascii_case("linear-gradient") {
                Ok(GradientImage::Linear(input.parse_nested_block(|input| {Ok(parse_linear(input)?)})?))
            } else {
				return Err(TokenParseError::from_expect(location, "url(...) | <url> | linear-gradient(...)", token.clone()));
            }
        }
        _ => return Err(TokenParseError::from_expect(location, "url(...) | <url> | linear-gradient(...)", token.clone()))
    }
}

#[derive(Debug, Error)]
#[error("{error}, location: {location:?}")]
pub struct TokenParseError<'i> {
	pub location: SourceLocation,
	pub error: TokenErrorsInfo<'i>
}

#[derive(Debug, Error)]
#[error("{list:?}")]
pub struct TokenParseErrorList<'i> {
	pub list: Vec<TokenParseError<'i>>,
}

#[derive(Debug, Error)]
pub enum TokenErrorsInfo<'i> {
	#[error("expect: {0}, but parse id end")]
	ExpectButEnd(&'static str),
	#[error("expect: {0}, find: {1:?}")]
	ExpectError(&'static str, Token<'i>),
	#[error("{0}")]
	ErrorMessage(String),
	#[error("{0:?}")]
	BaseParseError(BasicParseErrorKind<'i>),
	#[error("")]
	KeyError,
}

#[derive(Debug, Error)]
pub enum ItemParseErrors<'i> {
	#[error("expect: <attrribute>, find: {kind:?}, location: {location:?}")]
	KeyError {
		location: SourceLocation,
		kind: BasicParseErrorKind<'i>,
	},
	#[error("attribute parse error from {attribute}, {error}")]
	ValueError {
		attribute: CowRcStr<'i>,
		error: TokenParseError<'i>,
	}
}

impl<'i> From<BasicParseError<'i>> for ItemParseErrors<'i> {
    fn from(v: BasicParseError<'i>) -> ItemParseErrors<'i> {
		ItemParseErrors::KeyError {
			location: v.location,
			kind: v.kind,
		}
    }
}

impl<'i> TokenParseError<'i> {
	pub fn from_expect(location: SourceLocation, expect: &'static str, find: Token<'i>) -> Self {
		TokenParseError {
			location, 
			error: TokenErrorsInfo::ExpectError(expect, find)
		}
	}
	pub fn from_expect_but_end(location: SourceLocation, expect: &'static str) -> Self {
		TokenParseError {
			location, 
			error: TokenErrorsInfo::ExpectButEnd(expect)
		}
	}

	pub fn from_message(location: SourceLocation, message: String) -> Self {
		TokenParseError {
			location, 
			error: TokenErrorsInfo::ErrorMessage(message)
		}
	}
}


// #[derive(Clone, Debug, PartialEq)]
// pub struct ValueParseErrors<'i> (&'static str, Token<'i>);

// impl<'i> Into<ValueParseError<'i>> for ParseError<'i, ()> {
//     fn into(self) -> ValueParseError<'i> {
//         ValueParseError::ParseError(self)
//     }
// }

// required for `Result<f32, ValueParseError<'_>>` to implement `FromResidual<Result<Infallible, BasicParseError<'_>>>`

impl<'i> From<ParseError<'i, TokenErrorsInfo<'i>>> for TokenParseError<'i> {
    fn from(v: ParseError<'i, TokenErrorsInfo<'i>>) -> TokenParseError<'i> {
		let error = match v.kind {
			ParseErrorKind::Basic(r) => TokenErrorsInfo::BaseParseError(r),
			ParseErrorKind::Custom(r) => r,
		};
		TokenParseError {
			location: v.location,
			error,
		}
    }
}

impl<'i> From<TokenParseError<'i>> for ParseError<'i, TokenErrorsInfo<'i>> {
    fn from(v: TokenParseError<'i>) -> ParseError<'i, TokenErrorsInfo<'i>> {
		let error = match v.error {
			TokenErrorsInfo::BaseParseError(r) => ParseErrorKind::Basic(r),
			r => ParseErrorKind::Custom(r),
		};
		ParseError {
			location: v.location,
			kind: error,
		}
    }
}

impl<'i> From<BasicParseError<'i>> for TokenParseError<'i> {
    fn from(v: BasicParseError<'i>) -> TokenParseError<'i> {
		TokenParseError {
			location: v.location,
			error: TokenErrorsInfo::BaseParseError(v.kind),
		}
    }
}


// impl<'i> From<BasicParseError<'i>> for ValueParseError<'i> {
//     fn from(v: BasicParseError<'i>) -> ValueParseError<'i> {
//         Self::BasicParseError(v)
//     }
// }

pub fn parse_angle<'i, 't>(input: &mut Parser<'i, 't>) -> Result<f32, TokenParseError<'i>> {
    let location = input.current_source_location();
    let t = input.next()?;
	if let Token::Dimension { value, ref unit, .. } = *t {
		if unit.as_ref() == "deg" {
			return Ok(value);
		}
	}
	Err(TokenParseError::from_expect(location, "deg", t.clone()))?
}

#[inline]
pub fn parse_color_keyword(ident: &str) -> Result<CgColor, ()> {
    macro_rules! rgb {
        ($red: expr, $green: expr, $blue: expr) => {
            CgColor::new($red as f32 / 255.0, $green as f32 / 255.0, $blue as f32 / 255.0, 1.0)
        };
    }
    let color = match ident {
        "black" => rgb!(0, 0, 0),
        "silver" => rgb!(192, 192, 192),
        "gray" => rgb!(128, 128, 128),
        "white" => rgb!(255, 255, 255),
        "maroon" => rgb!(128, 0, 0),
        "red" => rgb!(255, 0, 0),
        "purple" => rgb!(128, 0, 128),
        "fuchsia" => rgb!(255, 0, 255),
        "green" => rgb!(0, 128, 0),
        "lime" => rgb!(0, 255, 0),
        "olive" => rgb!(128, 128, 0),
        "yellow" => rgb!(255, 255, 0),
        "navy" => rgb!(0, 0, 128),
        "blue" => rgb!(0, 0, 255),
        "teal" => rgb!(0, 128, 128),
        "aqua" => rgb!(0, 255, 255),

        "aliceblue" => rgb!(240, 248, 255),
        "antiquewhite" => rgb!(250, 235, 215),
        "aquamarine" => rgb!(127, 255, 212),
        "azure" => rgb!(240, 255, 255),
        "beige" => rgb!(245, 245, 220),
        "bisque" => rgb!(255, 228, 196),
        "blanchedalmond" => rgb!(255, 235, 205),
        "blueviolet" => rgb!(138, 43, 226),
        "brown" => rgb!(165, 42, 42),
        "burlywood" => rgb!(222, 184, 135),
        "cadetblue" => rgb!(95, 158, 160),
        "chartreuse" => rgb!(127, 255, 0),
        "chocolate" => rgb!(210, 105, 30),
        "coral" => rgb!(255, 127, 80),
        "cornflowerblue" => rgb!(100, 149, 237),
        "cornsilk" => rgb!(255, 248, 220),
        "crimson" => rgb!(220, 20, 60),
        "cyan" => rgb!(0, 255, 255),
        "darkblue" => rgb!(0, 0, 139),
        "darkcyan" => rgb!(0, 139, 139),
        "darkgoldenrod" => rgb!(184, 134, 11),
        "darkgray" => rgb!(169, 169, 169),
        "darkgreen" => rgb!(0, 100, 0),
        "darkgrey" => rgb!(169, 169, 169),
        "darkkhaki" => rgb!(189, 183, 107),
        "darkmagenta" => rgb!(139, 0, 139),
        "darkolivegreen" => rgb!(85, 107, 47),
        "darkorange" => rgb!(255, 140, 0),
        "darkorchid" => rgb!(153, 50, 204),
        "darkred" => rgb!(139, 0, 0),
        "darksalmon" => rgb!(233, 150, 122),
        "darkseagreen" => rgb!(143, 188, 143),
        "darkslateblue" => rgb!(72, 61, 139),
        "darkslategray" => rgb!(47, 79, 79),
        "darkslategrey" => rgb!(47, 79, 79),
        "darkturquoise" => rgb!(0, 206, 209),
        "darkviolet" => rgb!(148, 0, 211),
        "deeppink" => rgb!(255, 20, 147),
        "deepskyblue" => rgb!(0, 191, 255),
        "dimgray" => rgb!(105, 105, 105),
        "dimgrey" => rgb!(105, 105, 105),
        "dodgerblue" => rgb!(30, 144, 255),
        "firebrick" => rgb!(178, 34, 34),
        "floralwhite" => rgb!(255, 250, 240),
        "forestgreen" => rgb!(34, 139, 34),
        "gainsboro" => rgb!(220, 220, 220),
        "ghostwhite" => rgb!(248, 248, 255),
        "gold" => rgb!(255, 215, 0),
        "goldenrod" => rgb!(218, 165, 32),
        "greenyellow" => rgb!(173, 255, 47),
        "grey" => rgb!(128, 128, 128),
        "honeydew" => rgb!(240, 255, 240),
        "hotpink" => rgb!(255, 105, 180),
        "indianred" => rgb!(205, 92, 92),
        "indigo" => rgb!(75, 0, 130),
        "ivory" => rgb!(255, 255, 240),
        "khaki" => rgb!(240, 230, 140),
        "lavender" => rgb!(230, 230, 250),
        "lavenderblush" => rgb!(255, 240, 245),
        "lawngreen" => rgb!(124, 252, 0),
        "lemonchiffon" => rgb!(255, 250, 205),
        "lightblue" => rgb!(173, 216, 230),
        "lightcoral" => rgb!(240, 128, 128),
        "lightcyan" => rgb!(224, 255, 255),
        "lightgoldenrodyellow" => rgb!(250, 250, 210),
        "lightgray" => rgb!(211, 211, 211),
        "lightgreen" => rgb!(144, 238, 144),
        "lightgrey" => rgb!(211, 211, 211),
        "lightpink" => rgb!(255, 182, 193),
        "lightsalmon" => rgb!(255, 160, 122),
        "lightseagreen" => rgb!(32, 178, 170),
        "lightskyblue" => rgb!(135, 206, 250),
        "lightslategray" => rgb!(119, 136, 153),
        "lightslategrey" => rgb!(119, 136, 153),
        "lightsteelblue" => rgb!(176, 196, 222),
        "lightyellow" => rgb!(255, 255, 224),
        "limegreen" => rgb!(50, 205, 50),
        "linen" => rgb!(250, 240, 230),
        "magenta" => rgb!(255, 0, 255),
        "mediumaquamarine" => rgb!(102, 205, 170),
        "mediumblue" => rgb!(0, 0, 205),
        "mediumorchid" => rgb!(186, 85, 211),
        "mediumpurple" => rgb!(147, 112, 219),
        "mediumseagreen" => rgb!(60, 179, 113),
        "mediumslateblue" => rgb!(123, 104, 238),
        "mediumspringgreen" => rgb!(0, 250, 154),
        "mediumturquoise" => rgb!(72, 209, 204),
        "mediumvioletred" => rgb!(199, 21, 133),
        "midnightblue" => rgb!(25, 25, 112),
        "mintcream" => rgb!(245, 255, 250),
        "mistyrose" => rgb!(255, 228, 225),
        "moccasin" => rgb!(255, 228, 181),
        "navajowhite" => rgb!(255, 222, 173),
        "oldlace" => rgb!(253, 245, 230),
        "olivedrab" => rgb!(107, 142, 35),
        "orange" => rgb!(255, 165, 0),
        "orangered" => rgb!(255, 69, 0),
        "orchid" => rgb!(218, 112, 214),
        "palegoldenrod" => rgb!(238, 232, 170),
        "palegreen" => rgb!(152, 251, 152),
        "paleturquoise" => rgb!(175, 238, 238),
        "palevioletred" => rgb!(219, 112, 147),
        "papayawhip" => rgb!(255, 239, 213),
        "peachpuff" => rgb!(255, 218, 185),
        "peru" => rgb!(205, 133, 63),
        "pink" => rgb!(255, 192, 203),
        "plum" => rgb!(221, 160, 221),
        "powderblue" => rgb!(176, 224, 230),
        "rebeccapurple" => rgb!(102, 51, 153),
        "rosybrown" => rgb!(188, 143, 143),
        "royalblue" => rgb!(65, 105, 225),
        "saddlebrown" => rgb!(139, 69, 19),
        "salmon" => rgb!(250, 128, 114),
        "sandybrown" => rgb!(244, 164, 96),
        "seagreen" => rgb!(46, 139, 87),
        "seashell" => rgb!(255, 245, 238),
        "sienna" => rgb!(160, 82, 45),
        "skyblue" => rgb!(135, 206, 235),
        "slateblue" => rgb!(106, 90, 205),
        "slategray" => rgb!(112, 128, 144),
        "slategrey" => rgb!(112, 128, 144),
        "snow" => rgb!(255, 250, 250),
        "springgreen" => rgb!(0, 255, 127),
        "steelblue" => rgb!(70, 130, 180),
        "tan" => rgb!(210, 180, 140),
        "thistle" => rgb!(216, 191, 216),
        "tomato" => rgb!(255, 99, 71),
        "turquoise" => rgb!(64, 224, 208),
        "violet" => rgb!(238, 130, 238),
        "wheat" => rgb!(245, 222, 179),
        "whitesmoke" => rgb!(245, 245, 245),
        "yellowgreen" => rgb!(154, 205, 50),

        "transparent" => CgColor::new(0.0, 0.0, 0.0, 0.0),
        _ => return Err(()),
    };
    Ok(color)
}

pub fn parse_color_function<'i, 't>(location: SourceLocation, name: CowRcStr<'i>, input: &mut Parser<'i, 't>) -> Result<CgColor, TokenParseError<'i>> {
    let (red, green, blue, uses_commas) = match name.as_ref() {
        "rgb" | "rgba" => parse_rgb_components_rgb(input)?,
        // "hsl" | "hsla" => parse_rgb_components_hsl(input)?,
        _ => return Err(TokenParseError::from_expect(location, "rgb(...) | rgba(...)", Token::Ident(name.clone()))),
    };

    let alpha = if !input.is_exhausted() {
        if uses_commas {
            input.expect_comma()?;
        } else {
            input.expect_delim('/')?;
        };
        input.expect_number()?
    } else {
        1.0
    };

    input.expect_exhausted()?;
    Ok(CgColor::new(red, green, blue, alpha))
}

fn parse_rgb_components_rgb<'i, 't>(input: &mut Parser<'i, 't>) -> Result<(f32, f32, f32, bool), TokenParseError<'i>> {
    // Either integers or percentages, but all the same type.
    // https://drafts.csswg.org/css-color/#rgb-functions
    let red = input.expect_number()? / 256.0;
    let uses_commas = input.try_parse(|i| i.expect_comma()).is_ok();

    let green = input.expect_number()? / 256.0;

    if uses_commas {
        input.expect_comma()?;
    }
    let blue = input.expect_number()? / 256.0;

    Ok((red, green, blue, uses_commas))
}

#[test]
fn test_error() {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let s = ".c123{
		width: 10deg;
	}";

    if let Err(_r) = parse_class_map_from_string(s, 0) {}
}

#[test]
fn test1() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let s = ".c123{
		width: 10px;
		height:20px;
		filter:grayscale(50%) hue-rotate(90deg) saturate(20%) brightness(10%);

		background-color: rgba(255, 155, 0, 0.5);
		background-color: rgb(255, 155, 0);
		background-color: #ff00ffff;
		background-color: #ffff;
		background-color: #555;
		background-color: #ffffff;
		background-color: blue;
		background: linear-gradient(20deg, 10% #555, 100% #fff);

		background-image: url('a.png');
		background-image: linear-gradient(20deg, 10% #555, 100% #fff);

		border-image: url('a.png');
		border-image-slice: 10% 10% 20%;
		border-image-clip: 10% 10% 20%;
		border-image-repeat: repeat;

		box-shadow: 10px 10px 5px #888888;
		text-shadow: 2px 2px #ff0000;
	}.c456{width: 10px;height:20px;filter:blur(2px) hsi(10,10,10)}";

    if let Err(_r) = parse_class_map_from_string(s, 0) {}

    // log::trace!("parse: {:?}", parse);
}

#[test]
fn test2() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let s = "@keyframes role1Move1 {
		0.00% {
			width: 98px;
			height: 185px;
			left: 6.67%;
			top: 0.00%;
			background-image: url(psd/3652127002.80.png);
			image-clip: 16.9283% 30.8594% 33.5202% 0.0000%;
		}}.c123{width: 98px;
			height: 185px;}";

    if let Ok(r) = parse_class_map_from_string(s, 0) {
        log::info!("parser result: {:?}", r);
    }
}

#[test]
fn test3() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let s = ".c112024820{
		color: #00ffff;
	}";

    if let Err(_r) = parse_class_map_from_string(s, 0) {}
}

#[test]
fn test4() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let s = ".c1203870451{
		transform: scale(0.8,0.8);
	}";

    if let Err(_r) = parse_class_map_from_string(s, 0) {}
}

#[test]
fn test5() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let s = ".c2677724671{
		text-shadow: rgb(255,0,0) 0px 0px 5px,rgb(255,0,0) 0px 0px 3px,rgb(255,255,255) 0px 0px 1px;
	}";

    if let Err(_r) = parse_class_map_from_string(s, 0) {}
}

#[test]
fn test6() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let s = "left: 20px;";

    if let Err(_r) = parse_class_map_from_string(s, 0) {}
}

#[test]
fn test_animation() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let s = "
	.c2677724671{
		animation: myanimation 2s 2s 5 reverse paused backwards cubic-bezier(0.1, 0.7, 1.0, 0.1);
	},
	.c2677724672{
		animation-timing-function: steps(2, start), steps(2, start), ease, ease-in, ease-out, ease-in-out, linear, step-start, step-end, cubic-bezier(0.1, 0.7, 1.0, 0.1) ;
		animation-name: myanimation, myanimation1;
		animation-duration: 2s, 10ms ;
		animation-delay: 2s, 10ms;
		animation-iteration-count: 10, infinite;
		animation-direction: reverse, alternate, normal, alternate-reverse;
		animation-fill-mode: backwards, both, none, forwards ;
		animation-play-state: running, paused ;
	}
	";

	match parse_class_map_from_string(s, 0) {
		Ok(r) => println!("animation====={:?}", r),
		Err(_) => todo!(),
	}
}

#[test]
fn test_mul_semicolon() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let s = "
	.c1363885129{
		position: absolute;
		left:25px;
		right: 25px;;
		height: 100%;
	  }";

    if let Ok(r) = parse_class_map_from_string(s, 0) {
        println!("ret: {:?}", r);
    }
}

#[test]
fn test_border_radius() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let s = "
	.c1363885129{
		border-radius: 10% / 14% 9%;
	  }";

    if let Ok(r) = parse_class_map_from_string(s, 0) {
        println!("ret: {:?}", r);
    }
}

#[test]
fn test_clip_path() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let s = "
	.c1363885129{
		clip-path: circle(10px);
		clip-path: circle(10%);
		clip-path: circle(10% at 50px);
		clip-path: circle(10% at 50px 20%);

		clip-path: ellipse(10% 20px);
		clip-path: ellipse(10% 20px at 50px);
		clip-path: ellipse(10% 20px at 50px 20%);

		clip-path: inset(30px);
		clip-path: inset(30px 20%);
		clip-path: inset(30px 20% 10%);
		clip-path: inset(30px 20% 10% 50% round 20px);

		clip-path: sector(30deg 20deg 50px);
		clip-path: sector(30deg 20deg 50px  at 50px);
		clip-path: sector(30deg 20deg 20px at 50px 50px);
	  }";

    if let Ok(r) = parse_class_map_from_string(s, 0) {
        println!("ret: {:?}", r);
    }
}

#[test]
fn test_will_change_transform() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let s = "
	.c1363885129{
		width:200px;height:100px;background-color:#45f518;transform: scale(0.6);will-change-transform
	  }";

    if let Ok(r) = parse_class_map_from_string(s, 0) {
        println!("ret: {:?}", r);
    }
}

#[test]
fn test_content() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let s = r#"
	.c1363885129{
		content: "zzzfff",
	  }"#;

    if let Ok(r) = parse_class_map_from_string(s, 0) {
        println!("ret: {:?}", r);
    }
}

#[test]
fn test_transform() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let s = r#"
	.c1363885129{
		@keyframes leftRightAnim {
			0% {transform: translateY(600px); opacity: 0}
			75% {transform: translateX(0px)}
			100% {transform: translateX(0px); opacity: 1}
		}
	  }"#;

    if let Ok(r) = parse_class_map_from_string(s, 0) {
        println!("ret: {:?}", r);
    }
}

#[test]
fn test_as_image() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let s = r#"
	.c1363885129{
		as-image: none;
	}
	.c2{
		as-image: advise;
	}
	.c3{
		as-image: force;
	}"#;

    if let Ok(r) = parse_class_map_from_string(s, 0) {
        println!("ret: {:?}", r);
    }
}

#[test]
fn transition() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let s = r#"
	.c1363885129 {
		transition: left 500ms 5ms, bottom 500ms 5ms;
	}
	.c2{
		transition-property: right,top;
		transition-duration: 2s, 10ms ;
		transition-delay: 1s, 5ms;
		transition-timing-function: ease, ease-in;
	}"#;
	
    if let Ok(r) = parse_class_map_from_string(s, 0) {
        println!("ret: {:?}", r);
    }
}

#[test]
fn text_content() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let s = r#"
	.c1363885129 {
		content: 'aaaaa';
	}"#;
	
    if let Ok(r) = parse_class_map_from_string(s, 0) {
        println!("ret: {:?}", r);
    }
}

