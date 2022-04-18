use crate::font::Font;

pub fn convert(source: &str, font: Font) -> String {
    let mapping = font.characters();
    source
        .chars()
        .map(|original| if let Some(converted) = mapping.get(&original) {
            converted.clone()
        } else {
            original
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mathbold() {
        let source = "ernctjahmlszwgdqiuxfpvobyk";
        assert_eq!(
            "𝐞𝐫𝐧𝐜𝐭𝐣𝐚𝐡𝐦𝐥𝐬𝐳𝐰𝐠𝐝𝐪𝐢𝐮𝐱𝐟𝐩𝐯𝐨𝐛𝐲𝐤",
            convert(source, Font::MathBold)
        );
    }
}
