use termimad::*;

pub fn format_md<'sk, 'st>(skin: &'sk MadSkin, string: &'st str) -> FmtInline<'sk, 'st> {
    return skin.inline(string);
}
