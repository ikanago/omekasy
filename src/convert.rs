use crate::font::Font;

pub fn convert(source: &[char], font: Font) -> String {
    let mapping = font.characters();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_non_target_chars() {
        let source = "ernct_jahmlsz あwgdqi-uxfpvobyk"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "𝐞𝐫𝐧𝐜𝐭_𝐣𝐚𝐡𝐦𝐥𝐬𝐳 あ𝐰𝐠𝐝𝐪𝐢-𝐮𝐱𝐟𝐩𝐯𝐨𝐛𝐲𝐤",
            convert(&source, Font::Bold)
        );
    }

    #[test]
    fn bold() {
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "𝟖𝐖𝐲𝐦𝐗𝐛𝐋𝐕𝟑𝐧𝐈𝐍𝐔𝐡𝐎𝐨𝐐𝐤𝐊𝐆𝐟𝐮𝐘𝟗𝐇𝐬𝐙𝐒𝐂𝟔𝟕𝟓𝐣𝐳𝐁𝐄𝐭𝐀𝐓𝐃𝐅𝐌𝐑𝐠𝐏𝐩𝐞𝐚𝐱𝐢𝐉𝐜𝐫𝟎𝐪𝟒𝐥𝟏𝐰𝟐𝐝𝐯",
            convert(&source, Font::Bold)
        );
    }

    #[test]
    fn italic() {
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "8𝑊𝑦𝑚𝑋𝑏𝐿𝑉3𝑛𝐼𝑁𝑈ℎ𝑂𝑜𝑄𝑘𝐾𝐺𝑓𝑢𝑌9𝐻𝑠𝑍𝑆𝐶675𝑗𝑧𝐵𝐸𝑡𝐴𝑇𝐷𝐹𝑀𝑅𝑔𝑃𝑝𝑒𝑎𝑥𝑖𝐽𝑐𝑟0𝑞4𝑙1𝑤2𝑑𝑣",
            convert(&source, Font::Italic)
        );
    }

    #[test]
    fn bold_italic() {
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "8𝑾𝒚𝒎𝑿𝒃𝑳𝑽3𝒏𝑰𝑵𝑼𝒉𝑶𝒐𝑸𝒌𝑲𝑮𝒇𝒖𝒀9𝑯𝒔𝒁𝑺𝑪675𝒋𝒛𝑩𝑬𝒕𝑨𝑻𝑫𝑭𝑴𝑹𝒈𝑷𝒑𝒆𝒂𝒙𝒊𝑱𝒄𝒓0𝒒4𝒍1𝒘2𝒅𝒗",
            convert(&source, Font::BoldItalic)
        );
    }

    #[test]
    fn script() {
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "8𝒲𝓎𝓂𝒳𝒷ℒ𝒱3𝓃ℐ𝒩𝒰𝒽𝒪ℴ𝒬𝓀𝒦𝒢𝒻𝓊𝒴9ℋ𝓈𝒵𝒮𝒞675𝒿𝓏ℬℰ𝓉𝒜𝒯𝒟ℱℳℛℊ𝒫𝓅ℯ𝒶𝓍𝒾𝒥𝒸𝓇0𝓆4𝓁1𝓌2𝒹𝓋",
            convert(&source, Font::Script)
        );
    }

    #[test]
    fn bold_script() {
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "8𝓦𝔂𝓶𝓧𝓫𝓛𝓥3𝓷𝓘𝓝𝓤𝓱𝓞𝓸𝓠𝓴𝓚𝓖𝓯𝓾𝓨9𝓗𝓼𝓩𝓢𝓒675𝓳𝔃𝓑𝓔𝓽𝓐𝓣𝓓𝓕𝓜𝓡𝓰𝓟𝓹𝓮𝓪𝔁𝓲𝓙𝓬𝓻0𝓺4𝓵1𝔀2𝓭𝓿",
            convert(&source, Font::BoldScript)
        );
    }

    #[test]
    fn monospace() {
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv"
            .chars()
            .collect::<Vec<_>>();
        assert_eq!(
            "𝟾𝚆𝚢𝚖𝚇𝚋𝙻𝚅𝟹𝚗𝙸𝙽𝚄𝚑𝙾𝚘𝚀𝚔𝙺𝙶𝚏𝚞𝚈𝟿𝙷𝚜𝚉𝚂𝙲𝟼𝟽𝟻𝚓𝚣𝙱𝙴𝚝𝙰𝚃𝙳𝙵𝙼𝚁𝚐𝙿𝚙𝚎𝚊𝚡𝚒𝙹𝚌𝚛𝟶𝚚𝟺𝚕𝟷𝚠𝟸𝚍𝚟",
            convert(&source, Font::Monospace)
        );
    }
}
