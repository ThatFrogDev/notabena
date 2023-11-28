use termimad::*;

pub fn format_md<'a>(skin: &'a MadSkin, string: &'a str) -> FmtInline<'a, 'a> {
    return skin.inline(string);
}
