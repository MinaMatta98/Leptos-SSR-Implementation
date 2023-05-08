use styled::{style, Styles};


pub fn navbar_style() -> styled::Result<Styles> {
    style! (
                nav a:hover {
                    text-decoration: underline "!important";
                }
    )
}

