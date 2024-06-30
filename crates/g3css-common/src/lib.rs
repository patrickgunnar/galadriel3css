/// Enum representing different panoramic viewers in the G3CSS framework
/// Breakpoint - represents the media rules (e.g., Mobile, Tablet, Laptop, Desktop)
/// Children - represents the properties of elements in the G3CSS framework
#[derive(PartialEq, Debug, Clone)]
pub enum G3cssPanoramic {
    Breakpoint(String),
    Children(Vec<G3cssClass>),
}

/// Enum representing different properties of class elements in the G3CSS framework
#[derive(PartialEq, Debug, Clone)]
pub enum G3cssClass {
    ClassName(String),
    Inherits(String),
    Important(String),
    Properties(Vec<G3cssElements>),
    Hover(Vec<G3cssElements>),
    Active(Vec<G3cssElements>),
    Focus(Vec<G3cssElements>),
    FirstChild(Vec<G3cssElements>),
    LastChild(Vec<G3cssElements>),
    FirstOfType(Vec<G3cssElements>),
    LastOfType(Vec<G3cssElements>),
    OnlyChild(Vec<G3cssElements>),
    OnlyOfType(Vec<G3cssElements>),
    TargetPseudoClass(Vec<G3cssElements>),
    Visited(Vec<G3cssElements>),
    Checked(Vec<G3cssElements>),
    Disabled(Vec<G3cssElements>),
    Enabled(Vec<G3cssElements>),
    ReadOnly(Vec<G3cssElements>),
    ReadWrite(Vec<G3cssElements>),
    PlaceholderShown(Vec<G3cssElements>),
    Valid(Vec<G3cssElements>),
    Invalid(Vec<G3cssElements>),
    Required(Vec<G3cssElements>),
    Optional(Vec<G3cssElements>),
    Fullscreen(Vec<G3cssElements>),
    FocusWithin(Vec<G3cssElements>),
    FirstLine(Vec<G3cssElements>),
    FirstLetter(Vec<G3cssElements>),
    Before(Vec<G3cssElements>),
    After(Vec<G3cssElements>),
    OutOfRange(Vec<G3cssElements>),
    Root(Vec<G3cssElements>),
    FirstPage(Vec<G3cssElements>),
    LeftPage(Vec<G3cssElements>),
    RightPage(Vec<G3cssElements>),
    Empty(Vec<G3cssElements>),
    PanoramicViewer(Vec<Vec<G3cssPanoramic>>),
}

/// Enum representing a G3CSS alias.
/// Represents an alias with a vector of strings.
#[derive(PartialEq, Debug, Clone)]
pub enum G3cssAlias {
    Alias(Vec<String>),
}

/// Enum representing a G3CSS variable.
/// Represents a variable with a vector of strings.
#[derive(PartialEq, Debug, Clone)]
pub enum G3cssVariable {
    Variable(Vec<String>),
}

/// Enum representing a G3CSS theme.
/// Represents a theme with a vector of G3CSS variables.
#[derive(PartialEq, Debug, Clone)]
pub enum G3cssTheme {
    Variables(Vec<G3cssVariable>),
}

/// Enum representing different types of children elements in the G3CSS framework
#[derive(PartialEq, Debug, Clone)]
pub enum G3cssChildren {
    LightTheme(Vec<G3cssTheme>),
    DarkTheme(Vec<G3cssTheme>),
    Aliases(Vec<G3cssAlias>),
    Variables(Vec<G3cssVariable>),
    Class(Vec<G3cssClass>),
    Classes(Vec<Vec<G3cssClass>>),
}

/// Enum representing nodes in the G3CSS abstract syntax tree (AST)
#[derive(PartialEq, Debug, Clone)]
pub enum G3cssNode {
    Component(Vec<G3cssNode>),
    Global(Vec<G3cssNode>),
    Name(String),
    Extends(String),
    Children(Vec<G3cssChildren>),
}

/// Enum representing different types of elements in the G3CSS framework
#[derive(PartialEq, Debug, Clone)]
pub enum G3cssElements {
    Nickname(Vec<String>),
    AspectRatio(String),
    AccentColor(String),
    BackdropFilter(String),
    Content(String),
    Gap(String),
    RowGap(String),
    Scale(String),
    Order(String),
    PointerEvents(String),
    Margin(String),
    MarginBottom(String),
    MarginLeft(String),
    MarginRight(String),
    MarginTop(String),
    Padding(String),
    PaddingBottom(String),
    PaddingLeft(String),
    PaddingRight(String),
    PaddingTop(String),
    Height(String),
    Width(String),
    Filter(String),
    MaxHeight(String),
    MaxWidth(String),
    MinHeight(String),
    MinWidth(String),
    Border(String),
    BorderBottom(String),
    BorderBottomColor(String),
    BorderBottomStyle(String),
    BorderBottomWidth(String),
    BorderColor(String),
    BorderLeft(String),
    BorderLeftColor(String),
    BorderLeftStyle(String),
    BorderLeftWidth(String),
    BorderRight(String),
    BorderRightColor(String),
    BorderRightStyles(String),
    BorderRightWidth(String),
    BorderStyle(String),
    BorderTop(String),
    BorderTopColor(String),
    BorderTopStyle(String),
    BorderTopWidth(String),
    BorderWidth(String),
    Outline(String),
    OutlineColor(String),
    OutlineStyle(String),
    OutlineWidth(String),
    BorderBottomLeftRadius(String),
    BorderBottomRightRadius(String),
    BorderImage(String),
    BorderImageOutset(String),
    BorderImageRepeat(String),
    BorderImageSlice(String),
    BorderImageSource(String),
    BorderImageWidth(String),
    BorderRadius(String),
    BorderTopLeftRadius(String),
    BorderTopRightRadius(String),
    BoxDecorationBreak(String),
    BoxShadow(String),
    Background(String),
    BackgroundAttachment(String),
    BackgroundColor(String),
    BackgroundImage(String),
    BackgroundPosition(String),
    BackgroundPositionX(String),
    BackgroundPositionY(String),
    BackgroundRepeat(String),
    BackgroundClip(String),
    BackgroundOrigin(String),
    BackgroundSize(String),
    BackgroundBlendMode(String),
    ColorProfile(String),
    Opacity(String),
    RenderingIntent(String),
    Font(String),
    FontFamily(String),
    FontSize(String),
    FontStyle(String),
    FontVariant(String),
    FontWeight(String),
    FontSizeAdjust(String),
    FontStretch(String),
    Positioning(String),
    Bottom(String),
    Clear(String),
    ClipPath(String),
    Cursor(String),
    Display(String),
    Float(String),
    Left(String),
    Overflow(String),
    Position(String),
    Right(String),
    Top(String),
    Visibility(String),
    ZIndex(String),
    Color(String),
    Direction(String),
    FlexDirection(String),
    FlexWrap(String),
    LetterSpacing(String),
    LineHeight(String),
    LineBreak(String),
    TextAlign(String),
    TextDecoration(String),
    TextIndent(String),
    TextTransform(String),
    UnicodeBidi(String),
    VerticalAlign(String),
    WhiteSpace(String),
    WordSpacing(String),
    TextOutline(String),
    TextOverflow(String),
    TextShadow(String),
    TextWrap(String),
    WordBreak(String),
    WordWrap(String),
    ListStyle(String),
    ListStyleImage(String),
    ListStylePosition(String),
    ListStyleType(String),
    BorderCollapse(String),
    BorderSpacing(String),
    CaptionSide(String),
    EmptyCells(String),
    TableLayout(String),
    MarqueeDirection(String),
    MarqueePlayCount(String),
    MarqueeSpeed(String),
    MarqueeStyle(String),
    OverflowX(String),
    OverflowY(String),
    OverflowStyle(String),
    Rotation(String),
    BoxAlign(String),
    BoxDirection(String),
    BoxFlex(String),
    BoxFlexGroup(String),
    BoxLines(String),
    BoxOrdinalGroup(String),
    BoxOrient(String),
    BoxPack(String),
    AlignmentAdjust(String),
    AlignmentBaseline(String),
    BaselineShift(String),
    DominantBaseline(String),
    DropInitialAfterAdjust(String),
    DropInitialAfterAlign(String),
    DropInitialBeforeAdjust(String),
    DropInitialBeforeAlign(String),
    DropInitialSize(String),
    DropInitialValue(String),
    InlineBoxAlign(String),
    LineStacking(String),
    LineStackingRuby(String),
    LineStackingShift(String),
    LineStackingStrategy(String),
    TextHeight(String),
    ColumnCount(String),
    ColumnFill(String),
    ColumnGap(String),
    ColumnRule(String),
    ColumnRuleColor(String),
    ColumnRuleStyle(String),
    ColumnRuleWidth(String),
    ColumnSpan(String),
    ColumnWidth(String),
    Columns(String),
    Animation(String),
    AnimationName(String),
    AnimationDuration(String),
    AnimationTimingFunction(String),
    AnimationDelay(String),
    AnimationFillMode(String),
    AnimationIterationCount(String),
    AnimationDirection(String),
    AnimationPlayState(String),
    Transform(String),
    TransformOrigin(String),
    TransformStyle(String),
    Perspective(String),
    PerspectiveOrigin(String),
    BackfaceVisibility(String),
    Transition(String),
    TransitionProperty(String),
    TransitionDuration(String),
    TransitionTimingFunction(String),
    TransitionDelay(String),
    Orphans(String),
    PageBreakAfter(String),
    PageBreakBefore(String),
    PageBreakInside(String),
    Widows(String),
    Mark(String),
    MarkAfter(String),
    MarkBefore(String),
    Phonemes(String),
    Rest(String),
    RestAfter(String),
    RestBefore(String),
    VoiceBalance(String),
    VoiceDuration(String),
    VoicePitch(String),
    VoicePitchRange(String),
    VoiceRate(String),
    VoiceStress(String),
    VoiceVolume(String),
    Appearance(String),
    BoxSizing(String),
    Icon(String),
    NavDown(String),
    NavIndex(String),
    NavLeft(String),
    NavRight(String),
    NavUp(String),
    OutlineOffset(String),
    Resize(String),
    Quotes(String),
    Rotate(String),
    Translate(String),
    UserSelect(String),
    WritingMode(String),
    ObjectPosition(String),
    ObjectFit(String),
    JustifySelf(String),
    JustifyContent(String),
    JustifyItems(String),
    AlignSelf(String),
    AlignContent(String),
    AlignItems(String),
    Grid(String),
    GridArea(String),
    GridAutoColumns(String),
    GridAutoFlow(String),
    GridAutoRows(String),
    GridColumn(String),
    GridColumnEnd(String),
    GridColumnStart(String),
    GridRow(String),
    GridRowEnd(String),
    GridRowStart(String),
    GridTemplate(String),
    GridTemplateAreas(String),
    GridTemplateColumns(String),
    GridTemplateRows(String),
    ScrollbarColor(String),
    ScrollbarWidth(String),
    ScrollbarGutter(String),
}
