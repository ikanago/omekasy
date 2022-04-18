use crate::font::Font;

pub fn convert(source: &str, font: Font) -> String {
    let mapping = font.characters();
    source
        .chars()
        .map(|original| {
            if let Some(converted) = mapping.get(&original) {
                *converted
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
        let source = "ernct_jahmlsz あwgdqi-uxfpvobyk";
        assert_eq!(
            "𝐞𝐫𝐧𝐜𝐭_𝐣𝐚𝐡𝐦𝐥𝐬𝐳 あ𝐰𝐠𝐝𝐪𝐢-𝐮𝐱𝐟𝐩𝐯𝐨𝐛𝐲𝐤",
            convert(source, Font::MathBold)
        );
    }

    #[test]
    fn mathbold() {
        let source = "8WymXbLV3nINUhOoQkKGfuY9HsZSC675jzBEtATDFMRgPpeaxiJcr0q4l1w2dv";
        assert_eq!(
            "𝟖𝐖𝐲𝐦𝐗𝐛𝐋𝐕𝟑𝐧𝐈𝐍𝐔𝐡𝐎𝐨𝐐𝐤𝐊𝐆𝐟𝐮𝐘𝟗𝐇𝐬𝐙𝐒𝐂𝟔𝟕𝟓𝐣𝐳𝐁𝐄𝐭𝐀𝐓𝐃𝐅𝐌𝐑𝐠𝐏𝐩𝐞𝐚𝐱𝐢𝐉𝐜𝐫𝟎𝐪𝟒𝐥𝟏𝐰𝟐𝐝𝐯",
            convert(source, Font::MathBold)
        );
    }
}
