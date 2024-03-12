use termimad::*;

pub fn paragraph<'sk, 'st>(skin: &'sk MadSkin, string: &'st str) -> FmtText<'sk, 'st> {
    return skin.text(string, None)
}

pub fn inline<'sk, 'st>(skin: &'sk MadSkin, string: &'st str) -> FmtInline<'sk, 'st> {
  return skin.inline(string);
}