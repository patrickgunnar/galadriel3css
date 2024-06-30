use g3css_common::G3cssElements;

use crate::Rule;

/// Builds a nickname vector from a Pest `Pair`.
///
/// Given a `Pair` object representing a parsed nickname, this function
/// extracts each inner pair, trims any surrounding double quotes from
/// the string, and collects them into a `Vec<String>`.
///
/// # Arguments
///
/// - `pair` - A `Pair` from the Pest parser representing a parsed nickname.
///
/// # Returns
///
/// An `Option<Vec<String>>` containing the parts of the nickname, or `None`
/// if extraction fails.
fn build_node_from_nickname(pair: pest::iterators::Pair<Rule>) -> Option<Vec<String>> {
    // Create an empty vector to hold the parts of the nickname
    let mut nickname: Vec<String> = vec![];

    // Iterate over each inner pair within the provided pair
    for inner_pair in pair.into_inner() {
        // Match the rule of the inner pair to determine the nickname component type
        match inner_pair.as_rule() {
            // If it matches Rule::primary, push the trimmed string to nickname vector
            Rule::primary => {
                nickname.push(inner_pair.as_str().trim_matches('"').to_string());
            },
            // If it matches Rule::valuation, push the trimmed string to nickname vector
            Rule::valuation => {
                nickname.push(inner_pair.as_str().trim_matches('"').to_string());
            },
            // Ignore other rules
            _ => ()
        }
    }

    // Return the nickname vector wrapped in `Some`, indicating successful extraction
    Some(nickname)
}

/// Builds a G3CSS elements node from a parsing pair based on its rule.
///
/// # Parameters
/// - `pair`: Parsing pair from which to build the G3CSS elements node.
///
/// # Returns
/// Option containing the constructed G3CSS elements node if matched, or None if the rule doesn't match.
pub fn build_ast_from_elements(pair: pest::iterators::Pair<Rule>) -> Option<G3cssElements> {
    match pair.as_rule() {
        // Collects the value from the nickname rule.
        Rule::nickname => Some(G3cssElements::Nickname(build_node_from_nickname(pair)?)),
        // Collects the value from the aspect_ratio rule.
        Rule::aspect_ratio => Some(G3cssElements::AspectRatio(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the accent_color rule.
        Rule::accent_color => Some(G3cssElements::AccentColor(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the backdrop_filter rule.
        Rule::backdrop_filter => Some(G3cssElements::BackdropFilter(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the content rule.
        Rule::content => Some(G3cssElements::Content(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the gap rule.
        Rule::gap => Some(G3cssElements::Gap(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the row_gap rule.
        Rule::row_gap => Some(G3cssElements::RowGap(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the scale rule.
        Rule::scale => Some(G3cssElements::Scale(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the order rule.
        Rule::order => Some(G3cssElements::Order(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the pointer_events rule.
        Rule::pointer_events => Some(G3cssElements::PointerEvents(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the margin rule.
        Rule::margin => Some(G3cssElements::Margin(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the margin_bottom rule.
        Rule::margin_bottom => Some(G3cssElements::MarginBottom(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the margin_left rule.
        Rule::margin_left => Some(G3cssElements::MarginLeft(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the margin_right rule.
        Rule::margin_right => Some(G3cssElements::MarginRight(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the margin_top rule.
        Rule::margin_top => Some(G3cssElements::MarginTop(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the padding rule.
        Rule::padding => Some(G3cssElements::Padding(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the padding_bottom rule.
        Rule::padding_bottom => Some(G3cssElements::PaddingBottom(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the padding_left rule.
        Rule::padding_left => Some(G3cssElements::PaddingLeft(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the padding_right rule.
        Rule::padding_right => Some(G3cssElements::PaddingRight(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the padding_top rule.
        Rule::padding_top => Some(G3cssElements::PaddingTop(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the height rule.
        Rule::height => Some(G3cssElements::Height(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the width rule.
        Rule::width => Some(G3cssElements::Width(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the filter rule.
        Rule::filter => Some(G3cssElements::Filter(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the max_height rule.
        Rule::max_height => Some(G3cssElements::MaxHeight(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the max_width rule.
        Rule::max_width => Some(G3cssElements::MaxWidth(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the min_height rule.
        Rule::min_height => Some(G3cssElements::MinHeight(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the min_width rule.
        Rule::min_width => Some(G3cssElements::MinWidth(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border rule.
        Rule::border => Some(G3cssElements::Border(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_bottom rule.
        Rule::border_bottom => Some(G3cssElements::BorderBottom(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_bottom_color rule.
        Rule::border_bottom_color => Some(G3cssElements::BorderBottomColor(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_bottom_style rule.
        Rule::border_bottom_style => Some(G3cssElements::BorderBottomStyle(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_bottom_width rule.
        Rule::border_bottom_width => Some(G3cssElements::BorderBottomWidth(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_color rule.
        Rule::border_color => Some(G3cssElements::BorderColor(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_left rule.
        Rule::border_left => Some(G3cssElements::BorderLeft(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_left_color rule.
        Rule::border_left_color => Some(G3cssElements::BorderLeftColor(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_left_style rule.
        Rule::border_left_style => Some(G3cssElements::BorderLeftStyle(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_left_width rule.
        Rule::border_left_width => Some(G3cssElements::BorderLeftWidth(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_right rule.
        Rule::border_right => Some(G3cssElements::BorderRight(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_right_color rule.
        Rule::border_right_color => Some(G3cssElements::BorderRightColor(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_right_styles rule.
        Rule::border_right_styles => Some(G3cssElements::BorderRightStyles(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_right_width rule.
        Rule::border_right_width => Some(G3cssElements::BorderRightWidth(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_style rule.
        Rule::border_style => Some(G3cssElements::BorderStyle(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_top rule.
        Rule::border_top => Some(G3cssElements::BorderTop(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_top_color rule.
        Rule::border_top_color => Some(G3cssElements::BorderTopColor(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_top_style rule.
        Rule::border_top_style => Some(G3cssElements::BorderTopStyle(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_top_width rule.
        Rule::border_top_width => Some(G3cssElements::BorderTopWidth(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_width rule.
        Rule::border_width => Some(G3cssElements::BorderWidth(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the outline rule.
        Rule::outline => Some(G3cssElements::Outline(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the outline_color rule.
        Rule::outline_color => Some(G3cssElements::OutlineColor(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the outline_style rule.
        Rule::outline_style => Some(G3cssElements::OutlineStyle(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the outline_width rule.
        Rule::outline_width => Some(G3cssElements::OutlineWidth(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_bottom_left_radius rule.
        Rule::border_bottom_left_radius => Some(G3cssElements::BorderBottomLeftRadius(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_bottom_right_radius rule.
        Rule::border_bottom_right_radius => Some(G3cssElements::BorderBottomRightRadius(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_image rule.
        Rule::border_image => Some(G3cssElements::BorderImage(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_image_outset rule.
        Rule::border_image_outset => Some(G3cssElements::BorderImageOutset(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_image_repeat rule.
        Rule::border_image_repeat => Some(G3cssElements::BorderImageRepeat(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_image_slice rule.
        Rule::border_image_slice => Some(G3cssElements::BorderImageSlice(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_image_source rule.
        Rule::border_image_source => Some(G3cssElements::BorderImageSource(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_image_width rule.
        Rule::border_image_width => Some(G3cssElements::BorderImageWidth(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_radius rule.
        Rule::border_radius => Some(G3cssElements::BorderRadius(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_top_left_radius rule.
        Rule::border_top_left_radius => Some(G3cssElements::BorderTopLeftRadius(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_top_right_radius rule.
        Rule::border_top_right_radius => Some(G3cssElements::BorderTopRightRadius(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the box_decoration_break rule.
        Rule::box_decoration_break => Some(G3cssElements::BoxDecorationBreak(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the box_shadow rule.
        Rule::box_shadow => Some(G3cssElements::BoxShadow(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the background rule.
        Rule::background => Some(G3cssElements::Background(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the background_attachment rule.
        Rule::background_attachment => Some(G3cssElements::BackgroundAttachment(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the background_color rule.
        Rule::background_color => Some(G3cssElements::BackgroundColor(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the background_image rule.
        Rule::background_image => Some(G3cssElements::BackgroundImage(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the background_position rule.
        Rule::background_position => Some(G3cssElements::BackgroundPosition(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the background_position_x rule.
        Rule::background_position_x => Some(G3cssElements::BackgroundPositionX(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the background_position_y rule.
        Rule::background_position_y => Some(G3cssElements::BackgroundPositionY(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the background_repeat rule.
        Rule::background_repeat => Some(G3cssElements::BackgroundRepeat(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the background_clip rule.
        Rule::background_clip => Some(G3cssElements::BackgroundClip(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the background_origin rule.
        Rule::background_origin => Some(G3cssElements::BackgroundOrigin(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the background_size rule.
        Rule::background_size => Some(G3cssElements::BackgroundSize(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the background_blend_mode rule.
        Rule::background_blend_mode => Some(G3cssElements::BackgroundBlendMode(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the color_profile rule.
        Rule::color_profile => Some(G3cssElements::ColorProfile(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the opacity rule.
        Rule::opacity => Some(G3cssElements::Opacity(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the rendering_intent rule.
        Rule::rendering_intent => Some(G3cssElements::RenderingIntent(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the font rule.
        Rule::font => Some(G3cssElements::Font(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the font_family rule.
        Rule::font_family => Some(G3cssElements::FontFamily(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the font_size rule.
        Rule::font_size => Some(G3cssElements::FontSize(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the font_style rule.
        Rule::font_style => Some(G3cssElements::FontStyle(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the font_variant rule.
        Rule::font_variant => Some(G3cssElements::FontVariant(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the font_weight rule.
        Rule::font_weight => Some(G3cssElements::FontWeight(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the font_size_adjust rule.
        Rule::font_size_adjust => Some(G3cssElements::FontSizeAdjust(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the font_stretch rule.
        Rule::font_stretch => Some(G3cssElements::FontStretch(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the positioning rule.
        Rule::positioning => Some(G3cssElements::Positioning(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the bottom rule.
        Rule::bottom => Some(G3cssElements::Bottom(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the clear rule.
        Rule::clear => Some(G3cssElements::Clear(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the clip_path rule.
        Rule::clip_path => Some(G3cssElements::ClipPath(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the cursor rule.
        Rule::cursor => Some(G3cssElements::Cursor(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the display rule.
        Rule::display => Some(G3cssElements::Display(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the float rule.
        Rule::float => Some(G3cssElements::Float(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the left rule.
        Rule::left => Some(G3cssElements::Left(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the overflow rule.
        Rule::overflow => Some(G3cssElements::Overflow(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the position rule.
        Rule::position => Some(G3cssElements::Position(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the right rule.
        Rule::right => Some(G3cssElements::Right(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the top rule.
        Rule::top => Some(G3cssElements::Top(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the visibility rule.
        Rule::visibility => Some(G3cssElements::Visibility(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the z_index rule.
        Rule::z_index => Some(G3cssElements::ZIndex(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the color rule.
        Rule::color => Some(G3cssElements::Color(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the direction rule.
        Rule::direction => Some(G3cssElements::Direction(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the flex_direction rule.
        Rule::flex_direction => Some(G3cssElements::FlexDirection(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the flex_wrap rule.
        Rule::flex_wrap => Some(G3cssElements::FlexWrap(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the letter_spacing rule.
        Rule::letter_spacing => Some(G3cssElements::LetterSpacing(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the line_height rule.
        Rule::line_height => Some(G3cssElements::LineHeight(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the line_break rule.
        Rule::line_break => Some(G3cssElements::LineBreak(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the text_align rule.
        Rule::text_align => Some(G3cssElements::TextAlign(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the text_decoration rule.
        Rule::text_decoration => Some(G3cssElements::TextDecoration(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the text_indent rule.
        Rule::text_indent => Some(G3cssElements::TextIndent(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the text_transform rule.
        Rule::text_transform => Some(G3cssElements::TextTransform(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the unicode_bidi rule.
        Rule::unicode_bidi => Some(G3cssElements::UnicodeBidi(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the vertical_align rule.
        Rule::vertical_align => Some(G3cssElements::VerticalAlign(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the white_space rule.
        Rule::white_space => Some(G3cssElements::WhiteSpace(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the word_spacing rule.
        Rule::word_spacing => Some(G3cssElements::WordSpacing(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the text_outline rule.
        Rule::text_outline => Some(G3cssElements::TextOutline(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the text_overflow rule.
        Rule::text_overflow => Some(G3cssElements::TextOverflow(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the text_shadow rule.
        Rule::text_shadow => Some(G3cssElements::TextShadow(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the text_wrap rule.
        Rule::text_wrap => Some(G3cssElements::TextWrap(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the word_break rule.
        Rule::word_break => Some(G3cssElements::WordBreak(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the word_wrap rule.
        Rule::word_wrap => Some(G3cssElements::WordWrap(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the list_style rule.
        Rule::list_style => Some(G3cssElements::ListStyle(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the list_style_image rule.
        Rule::list_style_image => Some(G3cssElements::ListStyleImage(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the list_style_position rule.
        Rule::list_style_position => Some(G3cssElements::ListStylePosition(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the list_style_type rule.
        Rule::list_style_type => Some(G3cssElements::ListStyleType(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_collapse rule.
        Rule::border_collapse => Some(G3cssElements::BorderCollapse(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the border_spacing rule.
        Rule::border_spacing => Some(G3cssElements::BorderSpacing(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the caption_side rule.
        Rule::caption_side => Some(G3cssElements::CaptionSide(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the empty_cells rule.
        Rule::empty_cells => Some(G3cssElements::EmptyCells(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the table_layout rule.
        Rule::table_layout => Some(G3cssElements::TableLayout(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the marquee_direction rule.
        Rule::marquee_direction => Some(G3cssElements::MarqueeDirection(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the marquee_play_count rule.
        Rule::marquee_play_count => Some(G3cssElements::MarqueePlayCount(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the marquee_speed rule.
        Rule::marquee_speed => Some(G3cssElements::MarqueeSpeed(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the marquee_style rule.
        Rule::marquee_style => Some(G3cssElements::MarqueeStyle(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the overflow_x rule.
        Rule::overflow_x => Some(G3cssElements::OverflowX(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the overflow_y rule.
        Rule::overflow_y => Some(G3cssElements::OverflowY(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the overflow_style rule.
        Rule::overflow_style => Some(G3cssElements::OverflowStyle(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the rotation rule.
        Rule::rotation => Some(G3cssElements::Rotation(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the box_align rule.
        Rule::box_align => Some(G3cssElements::BoxAlign(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the box_direction rule.
        Rule::box_direction => Some(G3cssElements::BoxDirection(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the box_flex rule.
        Rule::box_flex => Some(G3cssElements::BoxFlex(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the box_flex_group rule.
        Rule::box_flex_group => Some(G3cssElements::BoxFlexGroup(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the box_lines rule.
        Rule::box_lines => Some(G3cssElements::BoxLines(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the box_ordinal_group rule.
        Rule::box_ordinal_group => Some(G3cssElements::BoxOrdinalGroup(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the box_orient rule.
        Rule::box_orient => Some(G3cssElements::BoxOrient(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the box_pack rule.
        Rule::box_pack => Some(G3cssElements::BoxPack(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the alignment_adjust rule.
        Rule::alignment_adjust => Some(G3cssElements::AlignmentAdjust(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the alignment_baseline rule.
        Rule::alignment_baseline => Some(G3cssElements::AlignmentBaseline(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the baseline_shift rule.
        Rule::baseline_shift => Some(G3cssElements::BaselineShift(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the dominant_baseline rule.
        Rule::dominant_baseline => Some(G3cssElements::DominantBaseline(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the drop_initial_after_adjust rule.
        Rule::drop_initial_after_adjust => Some(G3cssElements::DropInitialAfterAdjust(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the drop_initial_after_align rule.
        Rule::drop_initial_after_align => Some(G3cssElements::DropInitialAfterAlign(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the drop_initial_before_adjust rule.
        Rule::drop_initial_before_adjust => Some(G3cssElements::DropInitialBeforeAdjust(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the drop_initial_before_align rule.
        Rule::drop_initial_before_align => Some(G3cssElements::DropInitialBeforeAlign(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the drop_initial_size rule.
        Rule::drop_initial_size => Some(G3cssElements::DropInitialSize(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the drop_initial_value rule.
        Rule::drop_initial_value => Some(G3cssElements::DropInitialValue(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the inline_box_align rule.
        Rule::inline_box_align => Some(G3cssElements::InlineBoxAlign(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the line_stacking rule.
        Rule::line_stacking => Some(G3cssElements::LineStacking(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the line_stacking_ruby rule.
        Rule::line_stacking_ruby => Some(G3cssElements::LineStackingRuby(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the line_stacking_shift rule.
        Rule::line_stacking_shift => Some(G3cssElements::LineStackingShift(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the line_stacking_strategy rule.
        Rule::line_stacking_strategy => Some(G3cssElements::LineStackingStrategy(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the text_height rule.
        Rule::text_height => Some(G3cssElements::TextHeight(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the column_count rule.
        Rule::column_count => Some(G3cssElements::ColumnCount(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the column_fill rule.
        Rule::column_fill => Some(G3cssElements::ColumnFill(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the column_gap rule.
        Rule::column_gap => Some(G3cssElements::ColumnGap(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the column_rule rule.
        Rule::column_rule => Some(G3cssElements::ColumnRule(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the column_rule_color rule.
        Rule::column_rule_color => Some(G3cssElements::ColumnRuleColor(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the column_rule_style rule.
        Rule::column_rule_style => Some(G3cssElements::ColumnRuleStyle(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the column_rule_width rule.
        Rule::column_rule_width => Some(G3cssElements::ColumnRuleWidth(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the column_span rule.
        Rule::column_span => Some(G3cssElements::ColumnSpan(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the column_width rule.
        Rule::column_width => Some(G3cssElements::ColumnWidth(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the columns rule.
        Rule::columns => Some(G3cssElements::Columns(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the animation rule.
        Rule::animation => Some(G3cssElements::Animation(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the animation_name rule.
        Rule::animation_name => Some(G3cssElements::AnimationName(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the animation_duration rule.
        Rule::animation_duration => Some(G3cssElements::AnimationDuration(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the animation_timing_function rule.
        Rule::animation_timing_function => Some(G3cssElements::AnimationTimingFunction(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the animation_delay rule.
        Rule::animation_delay => Some(G3cssElements::AnimationDelay(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the animation_fill_mode rule.
        Rule::animation_fill_mode => Some(G3cssElements::AnimationFillMode(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the animation_iteration_count rule.
        Rule::animation_iteration_count => Some(G3cssElements::AnimationIterationCount(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the animation_direction rule.
        Rule::animation_direction => Some(G3cssElements::AnimationDirection(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the animation_play_state rule.
        Rule::animation_play_state => Some(G3cssElements::AnimationPlayState(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the transform rule.
        Rule::transform => Some(G3cssElements::Transform(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the transform_origin rule.
        Rule::transform_origin => Some(G3cssElements::TransformOrigin(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the transform_style rule.
        Rule::transform_style => Some(G3cssElements::TransformStyle(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the perspective rule.
        Rule::perspective => Some(G3cssElements::Perspective(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the perspective_origin rule.
        Rule::perspective_origin => Some(G3cssElements::PerspectiveOrigin(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the backface_visibility rule.
        Rule::backface_visibility => Some(G3cssElements::BackfaceVisibility(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the transition rule.
        Rule::transition => Some(G3cssElements::Transition(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the transition_property rule.
        Rule::transition_property => Some(G3cssElements::TransitionProperty(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the transition_duration rule.
        Rule::transition_duration => Some(G3cssElements::TransitionDuration(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the transition_timing_function rule.
        Rule::transition_timing_function => Some(G3cssElements::TransitionTimingFunction(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the transition_delay rule.
        Rule::transition_delay => Some(G3cssElements::TransitionDelay(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the orphans rule.
        Rule::orphans => Some(G3cssElements::Orphans(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the page_break_after rule.
        Rule::page_break_after => Some(G3cssElements::PageBreakAfter(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the page_break_before rule.
        Rule::page_break_before => Some(G3cssElements::PageBreakBefore(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the page_break_inside rule.
        Rule::page_break_inside => Some(G3cssElements::PageBreakInside(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the widows rule.
        Rule::widows => Some(G3cssElements::Widows(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the mark rule.
        Rule::mark => Some(G3cssElements::Mark(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the mark_after rule.
        Rule::mark_after => Some(G3cssElements::MarkAfter(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the mark_before rule.
        Rule::mark_before => Some(G3cssElements::MarkBefore(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the phonemes rule.
        Rule::phonemes => Some(G3cssElements::Phonemes(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the rest rule.
        Rule::rest => Some(G3cssElements::Rest(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the rest_after rule.
        Rule::rest_after => Some(G3cssElements::RestAfter(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the rest_before rule.
        Rule::rest_before => Some(G3cssElements::RestBefore(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the voice_balance rule.
        Rule::voice_balance => Some(G3cssElements::VoiceBalance(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the voice_duration rule.
        Rule::voice_duration => Some(G3cssElements::VoiceDuration(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the voice_pitch rule.
        Rule::voice_pitch => Some(G3cssElements::VoicePitch(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the voice_pitch_range rule.
        Rule::voice_pitch_range => Some(G3cssElements::VoicePitchRange(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the voice_rate rule.
        Rule::voice_rate => Some(G3cssElements::VoiceRate(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the voice_stress rule.
        Rule::voice_stress => Some(G3cssElements::VoiceStress(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the voice_volume rule.
        Rule::voice_volume => Some(G3cssElements::VoiceVolume(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the appearance rule.
        Rule::appearance => Some(G3cssElements::Appearance(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the box_sizing rule.
        Rule::box_sizing => Some(G3cssElements::BoxSizing(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the icon rule.
        Rule::icon => Some(G3cssElements::Icon(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the nav_down rule.
        Rule::nav_down => Some(G3cssElements::NavDown(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the nav_index rule.
        Rule::nav_index => Some(G3cssElements::NavIndex(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the nav_left rule.
        Rule::nav_left => Some(G3cssElements::NavLeft(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the nav_right rule.
        Rule::nav_right => Some(G3cssElements::NavRight(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the nav_up rule.
        Rule::nav_up => Some(G3cssElements::NavUp(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the outline_offset rule.
        Rule::outline_offset => Some(G3cssElements::OutlineOffset(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the resize rule.
        Rule::resize => Some(G3cssElements::Resize(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the quotes rule.
        Rule::quotes => Some(G3cssElements::Quotes(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the rotate rule.
        Rule::rotate => Some(G3cssElements::Rotate(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the translate rule.
        Rule::translate => Some(G3cssElements::Translate(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the user_select rule.
        Rule::user_select => Some(G3cssElements::UserSelect(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the writing_mode rule.
        Rule::writing_mode => Some(G3cssElements::WritingMode(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the object_position rule.
        Rule::object_position => Some(G3cssElements::ObjectPosition(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the object_fit rule.
        Rule::object_fit => Some(G3cssElements::ObjectFit(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the justify_self rule.
        Rule::justify_self => Some(G3cssElements::JustifySelf(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the justify_content rule.
        Rule::justify_content => Some(G3cssElements::JustifyContent(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the justify_items rule.
        Rule::justify_items => Some(G3cssElements::JustifyItems(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the align_self rule.
        Rule::align_self => Some(G3cssElements::AlignSelf(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the align_content rule.
        Rule::align_content => Some(G3cssElements::AlignContent(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the align_items rule.
        Rule::align_items => Some(G3cssElements::AlignItems(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the grid rule.
        Rule::grid => Some(G3cssElements::Grid(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the grid_area rule.
        Rule::grid_area => Some(G3cssElements::GridArea(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the grid_auto_columns rule.
        Rule::grid_auto_columns => Some(G3cssElements::GridAutoColumns(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the grid_auto_flow rule.
        Rule::grid_auto_flow => Some(G3cssElements::GridAutoFlow(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the grid_auto_rows rule.
        Rule::grid_auto_rows => Some(G3cssElements::GridAutoRows(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the grid_column rule.
        Rule::grid_column => Some(G3cssElements::GridColumn(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the grid_column_end rule.
        Rule::grid_column_end => Some(G3cssElements::GridColumnEnd(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the grid_column_start rule.
        Rule::grid_column_start => Some(G3cssElements::GridColumnStart(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the grid_row rule.
        Rule::grid_row => Some(G3cssElements::GridRow(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the grid_row_end rule.
        Rule::grid_row_end => Some(G3cssElements::GridRowEnd(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the grid_row_start rule.
        Rule::grid_row_start => Some(G3cssElements::GridRowStart(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the grid_template rule.
        Rule::grid_template => Some(G3cssElements::GridTemplate(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the grid_template_areas rule.
        Rule::grid_template_areas => Some(G3cssElements::GridTemplateAreas(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the grid_template_columns rule.
        Rule::grid_template_columns => Some(G3cssElements::GridTemplateColumns(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the grid_template_rows rule.
        Rule::grid_template_rows => Some(G3cssElements::GridTemplateRows(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the scrollbar_color rule.
        Rule::scrollbar_color => Some(G3cssElements::ScrollbarColor(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the scrollbar_width rule.
        Rule::scrollbar_width => Some(G3cssElements::ScrollbarWidth(
            pair.as_str().trim_matches('"').to_string(),
        )),
        // Collects the value from the scrollbar_gutter rule.
        Rule::scrollbar_gutter => Some(G3cssElements::ScrollbarGutter(
            pair.as_str().trim_matches('"').to_string(),
        )),
        _ => None,
    }
}
