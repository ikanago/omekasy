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
            "𝟴𝗪𝘆𝗺𝗫𝗯𝗟𝗩𝟯𝗻𝗜𝗡𝗨𝗵𝗢𝗼𝗤𝗸𝗞𝗚𝗳𝘂𝗬𝟵𝗛𝘀𝗭𝗦𝗖𝟲𝟳𝟱𝗷𝘇𝗕𝗘𝘁𝗔𝗧𝗗𝗙𝗠𝗥𝗴𝗣𝗽𝗲𝗮𝘅𝗶𝗝𝗰𝗿𝟬𝗾𝟰𝗹𝟭𝘄𝟮𝗱𝘃",
            convert(source, Font::MathBold)
        );
    }
}
