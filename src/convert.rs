use std::collections::HashMap;

use crate::font::{Font, FontMap};

/// This struct holds each font's mapping between normal characters to ones of the font.
pub struct Converter {
    font_mappings: HashMap<Font, FontMap>,
}

impl Converter {
    pub fn new(fonts: &[Font]) -> Self {
        let mut font_mappings = HashMap::new();
        for font in fonts {
            font_mappings.insert(*font, font.characters());
        }

        Self { font_mappings }
    }

    /// Convert given characters to specified font.
    /// Non-alphanumeric characters remain unchanged.
    pub fn convert(&self, source: &[char], font: Font) -> String {
        let mapping = self
            .font_mappings
            .get(&font)
            .expect("Unexpected font specified");
        source
            .iter()
            .map(|original| {
                if let Some(converted) = mapping.get(original) {
                    converted
                } else {
                    original
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::ArgEnum;

    fn setup_converter() -> Converter {
        Converter::new(Font::value_variants())
    }

    #[test]
    fn skip_non_target_chars() {
        let converter = setup_converter();
        let source = "ernct_jahmlsz ãwgdqi-uxfpvobyk"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "ðð«ð§ðð­_ð£ðð¡ð¦ð¥ð¬ð³ ãð°ð ððªð¢-ð®ð±ðð©ð¯ð¨ðð²ð¤",
            converter.convert(&source, Font::Bold)
        );
    }

    #[test]
    fn bold() {
        let converter = setup_converter();
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "ððð²ð¦ðððððð§ðððð¡ðð¨ðð¤ðððð®ðððð¬ððððððð£ð³ððð­ððððððð ðð©ððð±ð¢ððð«ððªðð¥ðð°ððð¯",
            converter.convert(&source, Font::Bold)
        );
    }

    #[test]
    fn italic() {
        let converter = setup_converter();
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "8ðð¦ðððð¿ð3ðð¼ððâððððð¾ðºðð¢ð9ð»ð ððð¶675ðð§ðµð¸ð¡ð´ðð·ð¹ðððððððð¥ðð½ðð0ð4ð1ð¤2ðð£",
            converter.convert(&source, Font::Italic)
        );
    }

    #[test]
    fn bold_italic() {
        let converter = setup_converter();
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "8ð¾ððð¿ðð³ð½3ðð°ðµð¼ðð¶ðð¸ðð²ð®ððð9ð¯ðððºðª675ððð©ð¬ðð¨ð»ð«ð­ð´ð¹ðð·ðððððð±ðð0ð4ð1ð2ðð",
            converter.convert(&source, Font::BoldItalic)
        );
    }

    #[test]
    fn sans() {
        let converter = setup_converter();
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "ðªð¶ððð·ð»ð«ðµð¥ðð¨ð­ð´ðð®ðð°ððªð¦ð¿ðð¸ð«ð§ðð¹ð²ð¢ð¨ð©ð§ððð¡ð¤ðð ð³ð£ð¥ð¬ð±ðð¯ðð¾ðºððð©ð¼ðð¢ðð¦ðð£ðð¤ð½ð",
            converter.convert(&source, Font::Sans)
        );
    }

    #[test]
    fn bold_sans() {
        let converter = setup_converter();
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "ð´ðªððºð«ð¯ðð©ð¯ð»ðð¡ð¨ðµð¢ð¼ð¤ð¸ððð³ðð¬ðµððð­ð¦ðð²ð³ð±ð·ðððððð§ððð ð¥ð´ð£ð½ð²ð®ðð¶ðð°ð¿ð¬ð¾ð°ð¹ð­ðð®ð±ð",
            converter.convert(&source, Font::BoldSans)
        );
    }

    #[test]
    fn italic_sans() {
        let converter = setup_converter();
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "ð´ððºð®ðð£ððð¯ð¯ðððð©ðð°ðð¬ððð§ð¶ð ðµðð´ð¡ððð²ð³ð±ð«ð»ðððµððððððð¨ðð±ð¦ð¢ð¹ðªðð¤ð³ð¬ð²ð°ð­ð­ð¸ð®ð¥ð·",
            converter.convert(&source, Font::ItalicSans)
        );
    }

    #[test]
    fn bold_italic_sans() {
        let converter = setup_converter();
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "ð´ðð®ð¢ððððð¯ð£ðððððð¤ðð ððððªððµðð¨ððð¾ð²ð³ð±ðð¯ð½ðð©ð¼ðð¿ðððððð¥ððð­ðððð§ð¬ð¦ð°ð¡ð­ð¬ð®ðð«",
            converter.convert(&source, Font::BoldItalicSans)
        );
    }

    #[test]
    fn script() {
        let converter = setup_converter();
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "8ð²ððð³ð·âð±3ðâð©ð°ð½ðªâ´ð¬ðð¦ð¢ð»ðð´9âððµð®ð675ð¿ðâ¬â°ððð¯ðâ±â³ââð«ðâ¯ð¶ðð¾ð¥ð¸ð0ð4ð1ð2ð¹ð",
            converter.convert(&source, Font::Script)
        );
    }

    #[test]
    fn bold_script() {
        let converter = setup_converter();
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "8ð¦ðð¶ð§ð«ðð¥3ð·ððð¤ð±ðð¸ð ð´ððð¯ð¾ð¨9ðð¼ð©ð¢ð675ð³ðððð½ðð£ðððð¡ð°ðð¹ð®ðªðð²ðð¬ð»0ðº4ðµ1ð2ð­ð¿",
            converter.convert(&source, Font::BoldScript)
        );
    }

    #[test]
    fn fraktur() {
        let converter = setup_converter();
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "8ðð¶ðªðððð3ð«âððð¥ðð¬ðð¨ððð£ð²ð9âð°â¨ðâ­675ð§ð·ððð±ðððððâð¤ðð­ð¢ððµð¦ðð ð¯0ð®4ð©1ð´2ð¡ð³",
            converter.convert(&source, Font::Fraktur)
        );
    }

    #[test]
    fn bold_fraktur() {
        let converter = setup_converter();
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "8ðððððð·ð3ðð´ð¹ðððºðð¼ðð¶ð²ððð9ð³ððð¾ð®675ððð­ð°ðð¬ð¿ð¯ð±ð¸ð½ðð»ððððððµðð0ð4ð1ð2ðð",
            converter.convert(&source, Font::BoldFraktur)
        );
    }

    #[test]
    fn monospace() {
        let converter = setup_converter();
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "ð¾ðð¢ðððð»ðð¹ðð¸ð½ððð¾ððððºð¶ðððð¿ð·ðððð²ð¼ð½ð»ðð£ð±ð´ðð°ðð³ðµð¼ððð¿ðððð¡ðð¹ððð¶ððºðð·ð ð¸ðð",
            converter.convert(&source, Font::Monospace)
        );
    }

    #[test]
    fn blackboard() {
        let converter = setup_converter();
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "ð ððªððððððððâðððð âððð¾ðð¦ðð¡âð¤â¤ðâððððð«ð¹ð¼ð¥ð¸ðð»ð½ðâðâð¡ððð©ðððð£ðð¢ðððð¨ððð§",
            converter.convert(&source, Font::Blackboard)
        );
    }
}
