use std::collections::HashMap;

pub enum Font {
    MathBold,
}

impl Font {
    pub fn characters(&self) -> HashMap<char, char> {
        let source = "abcdefghijklmnopqrstuvwxyz";
        let target = match self {
            Font::MathBold => "𝐚𝐛𝐜𝐝𝐞𝐟𝐠𝐡𝐢𝐣𝐤𝐥𝐦𝐧𝐨𝐩𝐪𝐫𝐬𝐭𝐮𝐯𝐰𝐱𝐲𝐳",
        };

        source.chars().zip(target.chars()).collect()
    }
}
