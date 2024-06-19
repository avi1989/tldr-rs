/// A simple struct to represent the color in the RGB format.
pub struct Color {
    /// A number between 0 and 255 which represents Red spectrum.
    pub r: u8,
    /// A number between 0 and 255 which represents Green spectrum.
    pub g: u8,
    /// A number between 0 and 255 which represents Blue spectrum.
    pub b: u8,
}

impl Color {
    /// Creates a new instance of Color with the hex code
    /// ```rust
    /// use markdown::Color;
    /// let white = Color::new("#FFF");
    /// assert_eq!(white.r, 255)
    /// ```
    pub fn new(hex_color: &str) -> Self {
        let mut color_code = hex_color.to_string();

        color_code.retain(|c| !r#"#"#.contains(c));

        let r_str = format!("{}{}", &color_code[0..1], &color_code[0..1]);
        let g_str = format!("{}{}", &color_code[1..2], &color_code[1..2]);
        let b_str = format!("{}{}", &color_code[2..3], &color_code[2..3]);

        if color_code.len() == 3 {
            let r = u8::from_str_radix(&r_str, 16);
            let g = u8::from_str_radix(&g_str, 16);
            let b = u8::from_str_radix(&b_str, 16);

            if r.is_err() || g.is_err() || b.is_err() {
                panic!("Invalid Hex color code {}", hex_color);
            }

            return Self {
                r: r.unwrap(),
                g: g.unwrap(),
                b: b.unwrap(),
            };
        } else if color_code.len() == 6 {
            let r = u8::from_str_radix(&color_code[0..2], 16);
            let g = u8::from_str_radix(&color_code[2..4], 16);
            let b = u8::from_str_radix(&color_code[4..6], 16);

            return Self {
                r: r.unwrap(),
                g: g.unwrap(),
                b: b.unwrap(),
            };
        } else {
            panic!("Invalid length of codes");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! invalid_input_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _color = Color::new($value);
            }
        )*
        }
    }

    macro_rules! valid_input_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let _color = Color::new($value);
                // No Panic
            }
        )*
        }
    }

    macro_rules! conversion_test {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, r, g, b) = $value;
                    let color = Color::new(input);

                    assert_eq!(color.r, r);
                    assert_eq!(color.g, g);
                    assert_eq!(color.b, b);
                }
            )*
        }
    }

    conversion_test!{
        should_convert_000000_to_black: ("#000000", 0, 0, 0),
        should_convert_000_to_black: ("#000", 0, 0, 0),
        should_convert_ffffff_to_white: ("#FFFFFF", 255, 255, 255),
        should_convert_fff_to_white: ("#FFF", 255, 255, 255),
        should_convert_f00_to_red: ("#F00", 255, 0, 0),
        should_convert_ff0000_to_red: ("#FF0000", 255, 0, 0),
        should_convert_f00_to_green: ("#0F0", 0, 255, 0),
        should_convert_ff0000_to_green: ("#00FF00", 0, 255, 0),
        should_convert_f00_to_blue: ("#00F", 0, 0, 255),
        should_convert_ff0000_to_blue: ("#0000FF", 0, 0, 255),
    }

    invalid_input_tests! {
        input_codes_cannot_be_1_characters: "0",
        input_codes_cannot_be_1_characters_with_hash: "#0",
        input_codes_cannot_be_2_characters: "00",
        input_codes_cannot_be_2_characters_with_hash: "#00",
        input_codes_cannot_be_4_characters: "0000",
        input_codes_cannot_be_4_characters_with_hash: "#0000",
        input_codes_cannot_be_5_characters: "00000",
        input_codes_cannot_be_5_characters_with_hash: "#00000",
        input_codes_cannot_be_7_characters: "0000000",
        input_codes_cannot_be_7_characters_with_hash: "#0000000",
        should_fail_for_invalid_rgb_code_in_red: "#GG0011",
        should_fail_for_invalid_rgb_code_in_green: "#00GG11",
        should_fail_for_invalid_rgb_code_in_blue: "#00AAZZ",
    }

    valid_input_tests! {
        input_codes_can_be_3_digits: "000",
        input_codes_can_be_3_digits_with_hash: "#000",
        input_codes_can_be_6_digits: "000000",
        input_codes_can_be_6_digits_with_hash: "#000000",
    }
}
