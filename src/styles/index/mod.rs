use styled::{Styles, style};

pub fn carousel_styles() -> styled::Result<Styles> {
    style!(
            .carousel {
                margin-bottom: 10rem;
            }

            .carousel-caption {
                bottom: 3rem;
                z-index: 10;
            }

            .carousel-item {
                height: 36rem;
            }

            @media (max-width: 1250px) {
                .carousel-item {
                    height: 26rem;
                }
            }
    )
}

pub fn example_styles() -> styled::Result<Styles> {
    style!(
                .marketing ".col-lg-4" {
                    margin-bottom: 1.5rem;
                    text-align: center;
                }

                .marketing ".col-lg-4" p {
                    margin-right: 0.75rem;
                    margin-left: 0.75rem;
                }
    )
}

