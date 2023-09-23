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
        let converted = source.iter().map(|original| {
            if let Some(converted) = mapping.get(original) {
                converted
            } else {
                original
            }
        });

        if font == Font::Emoji {
            // In this application, we want reginal indicator symbols to be rendered as emoji.
            // To prevent them from being rendered as flags, we insert zero-width joiner(U+200C) between each character.
            // For a simple implementation, we U+200C between all characters.
            converted
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join('\u{200C}'.to_string().as_str())
        } else {
            converted.collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::ValueEnum;

    fn setup_converter() -> Converter {
        Converter::new(Font::value_variants())
    }

    #[test]
    fn skip_non_target_chars() {
        let converter = setup_converter();
        let source = "ernct_jahmlsz あwgdqi-uxfpvobyk"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "𝐞𝐫𝐧𝐜𝐭_𝐣𝐚𝐡𝐦𝐥𝐬𝐳 あ𝐰𝐠𝐝𝐪𝐢-𝐮𝐱𝐟𝐩𝐯𝐨𝐛𝐲𝐤",
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
            "𝟖𝐖𝐲𝐦𝐗𝐛𝐋𝐕𝟑𝐧𝐈𝐍𝐔𝐡𝐎𝐨𝐐𝐤𝐊𝐆𝐟𝐮𝐘𝟗𝐇𝐬𝐙𝐒𝐂𝟔𝟕𝟓𝐣𝐳𝐁𝐄𝐭𝐀𝐓𝐃𝐅𝐌𝐑𝐠𝐏𝐩𝐞𝐚𝐱𝐢𝐉𝐜𝐫𝟎𝐪𝟒𝐥𝟏𝐰𝟐𝐝𝐯",
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
            "8𝑊𝑦𝑚𝑋𝑏𝐿𝑉3𝑛𝐼𝑁𝑈ℎ𝑂𝑜𝑄𝑘𝐾𝐺𝑓𝑢𝑌9𝐻𝑠𝑍𝑆𝐶675𝑗𝑧𝐵𝐸𝑡𝐴𝑇𝐷𝐹𝑀𝑅𝑔𝑃𝑝𝑒𝑎𝑥𝑖𝐽𝑐𝑟0𝑞4𝑙1𝑤2𝑑𝑣",
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
            "8𝑾𝒚𝒎𝑿𝒃𝑳𝑽3𝒏𝑰𝑵𝑼𝒉𝑶𝒐𝑸𝒌𝑲𝑮𝒇𝒖𝒀9𝑯𝒔𝒁𝑺𝑪675𝒋𝒛𝑩𝑬𝒕𝑨𝑻𝑫𝑭𝑴𝑹𝒈𝑷𝒑𝒆𝒂𝒙𝒊𝑱𝒄𝒓0𝒒4𝒍1𝒘2𝒅𝒗",
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
            "𝟪𝖶𝗒𝗆𝖷𝖻𝖫𝖵𝟥𝗇𝖨𝖭𝖴𝗁𝖮𝗈𝖰𝗄𝖪𝖦𝖿𝗎𝖸𝟫𝖧𝗌𝖹𝖲𝖢𝟨𝟩𝟧𝗃𝗓𝖡𝖤𝗍𝖠𝖳𝖣𝖥𝖬𝖱𝗀𝖯𝗉𝖾𝖺𝗑𝗂𝖩𝖼𝗋𝟢𝗊𝟦𝗅𝟣𝗐𝟤𝖽𝗏",
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
            "𝟴𝗪𝘆𝗺𝗫𝗯𝗟𝗩𝟯𝗻𝗜𝗡𝗨𝗵𝗢𝗼𝗤𝗸𝗞𝗚𝗳𝘂𝗬𝟵𝗛𝘀𝗭𝗦𝗖𝟲𝟳𝟱𝗷𝘇𝗕𝗘𝘁𝗔𝗧𝗗𝗙𝗠𝗥𝗴𝗣𝗽𝗲𝗮𝘅𝗶𝗝𝗰𝗿𝟬𝗾𝟰𝗹𝟭𝘄𝟮𝗱𝘃",
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
            "𝟴𝘞𝘺𝘮𝘟𝘣𝘓𝘝𝟯𝘯𝘐𝘕𝘜𝘩𝘖𝘰𝘘𝘬𝘒𝘎𝘧𝘶𝘠𝟵𝘏𝘴𝘡𝘚𝘊𝟲𝟳𝟱𝘫𝘻𝘉𝘌𝘵𝘈𝘛𝘋𝘍𝘔𝘙𝘨𝘗𝘱𝘦𝘢𝘹𝘪𝘑𝘤𝘳𝟬𝘲𝟰𝘭𝟭𝘸𝟮𝘥𝘷",
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
            "𝟴𝙒𝙮𝙢𝙓𝙗𝙇𝙑𝟯𝙣𝙄𝙉𝙐𝙝𝙊𝙤𝙌𝙠𝙆𝙂𝙛𝙪𝙔𝟵𝙃𝙨𝙕𝙎𝘾𝟲𝟳𝟱𝙟𝙯𝘽𝙀𝙩𝘼𝙏𝘿𝙁𝙈𝙍𝙜𝙋𝙥𝙚𝙖𝙭𝙞𝙅𝙘𝙧𝟬𝙦𝟰𝙡𝟭𝙬𝟮𝙙𝙫",
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
            "8𝒲𝓎𝓂𝒳𝒷ℒ𝒱3𝓃ℐ𝒩𝒰𝒽𝒪ℴ𝒬𝓀𝒦𝒢𝒻𝓊𝒴9ℋ𝓈𝒵𝒮𝒞675𝒿𝓏ℬℰ𝓉𝒜𝒯𝒟ℱℳℛℊ𝒫𝓅ℯ𝒶𝓍𝒾𝒥𝒸𝓇0𝓆4𝓁1𝓌2𝒹𝓋",
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
            "8𝓦𝔂𝓶𝓧𝓫𝓛𝓥3𝓷𝓘𝓝𝓤𝓱𝓞𝓸𝓠𝓴𝓚𝓖𝓯𝓾𝓨9𝓗𝓼𝓩𝓢𝓒675𝓳𝔃𝓑𝓔𝓽𝓐𝓣𝓓𝓕𝓜𝓡𝓰𝓟𝓹𝓮𝓪𝔁𝓲𝓙𝓬𝓻0𝓺4𝓵1𝔀2𝓭𝓿",
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
            "8𝔚𝔶𝔪𝔛𝔟𝔏𝔙3𝔫ℑ𝔑𝔘𝔥𝔒𝔬𝔔𝔨𝔎𝔊𝔣𝔲𝔜9ℌ𝔰ℨ𝔖ℭ675𝔧𝔷𝔅𝔈𝔱𝔄𝔗𝔇𝔉𝔐ℜ𝔤𝔓𝔭𝔢𝔞𝔵𝔦𝔍𝔠𝔯0𝔮4𝔩1𝔴2𝔡𝔳",
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
            "8𝖂𝖞𝖒𝖃𝖇𝕷𝖁3𝖓𝕴𝕹𝖀𝖍𝕺𝖔𝕼𝖐𝕶𝕲𝖋𝖚𝖄9𝕳𝖘𝖅𝕾𝕮675𝖏𝖟𝕭𝕰𝖙𝕬𝕿𝕯𝕱𝕸𝕽𝖌𝕻𝖕𝖊𝖆𝖝𝖎𝕵𝖈𝖗0𝖖4𝖑1𝖜2𝖉𝖛",
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
            "𝟾𝚆𝚢𝚖𝚇𝚋𝙻𝚅𝟹𝚗𝙸𝙽𝚄𝚑𝙾𝚘𝚀𝚔𝙺𝙶𝚏𝚞𝚈𝟿𝙷𝚜𝚉𝚂𝙲𝟼𝟽𝟻𝚓𝚣𝙱𝙴𝚝𝙰𝚃𝙳𝙵𝙼𝚁𝚐𝙿𝚙𝚎𝚊𝚡𝚒𝙹𝚌𝚛𝟶𝚚𝟺𝚕𝟷𝚠𝟸𝚍𝚟",
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
            "𝟠𝕎𝕪𝕞𝕏𝕓𝕃𝕍𝟛𝕟𝕀ℕ𝕌𝕙𝕆𝕠ℚ𝕜𝕂𝔾𝕗𝕦𝕐𝟡ℍ𝕤ℤ𝕊ℂ𝟞𝟟𝟝𝕛𝕫𝔹𝔼𝕥𝔸𝕋𝔻𝔽𝕄ℝ𝕘ℙ𝕡𝕖𝕒𝕩𝕚𝕁𝕔𝕣𝟘𝕢𝟜𝕝𝟙𝕨𝟚𝕕𝕧",
            converter.convert(&source, Font::Blackboard)
        );
    }

    #[test]
    fn emoji() {
        let converter = setup_converter();
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "8‌🇽‌🇾‌🇲‌🇾‌🇧‌🇲‌🇼‌3‌🇳‌🇯‌🇴‌🇻‌🇭‌🇵‌🇴‌🇷‌🇰‌🇱‌🇭‌🇫‌🇺‌🇿‌9‌🇮‌🇸‌🇿‌🇹‌🇩‌6‌7‌5‌🇯‌🇦‌🇨‌🇫‌🇹‌🇧‌🇺‌🇪‌🇬‌🇳‌🇸‌🇬‌🇶‌🇵‌🇪‌🇦‌🇽‌🇮‌🇰‌🇨‌🇷‌0‌🇶‌4‌🇱‌1‌🇼‌2‌🇩‌🇻",
            converter.convert(&source, Font::Emoji)
        );
    }
}
