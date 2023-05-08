use crate::styles::index::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component(transparent)]
pub fn Index(cx: Scope) -> impl IntoView {
    view! {cx,
        <Router>
        <Outlet/>
            <Carousel/>
            <Examples/>
        </Router>
    }
}

#[component(transparent)]
fn IndexHead(cx: Scope) -> impl IntoView {
    view! {cx,
        <Router>
            <head>
                <Meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>"Demo 1"</title>
                <Stylesheet href="/bootstrap-5.2.3-dist/css/bootstrap.min.css" />
                <Script src="/bootstrap-5.2.3-dist/js/bootstrap.bundle.min.js"/>
            </head>
        </Router>
    }
}

#[component(transparent)]
fn Examples(cx: Scope) -> impl IntoView {
    let style = example_styles();
    styled::view! {cx, style,
        <Router>
                <div class="container marketing text-center">
                    <div class="row">
                        <div class="col-lg-4">
                            <svg class="bd-placeholder-img rounded-circle" width="140" height="140"
                                xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Placeholder: 140x140"
                                preserveAspectRatio="xMidYMid slice" focusable="false">
                                <title>"Placeholder"</title>
                                <rect width="100%" height="100%" fill="#777" />
                                <text x="50%" y="50%" fill="#777" dy=".3em">"140x140"</text>
                            </svg>
                                <h2 class="fw-normal">"Example 1"</h2>
                            <p>
                                "Some representative placeholder content for the three columns of
                                text below the carousel. This is the first column."
                            </p>
                            <p><a class="btn btn-secondary" href="#">"View details &raquo;"</a></p>
                        </div>
                        <div class="col-lg-4">
                            <svg class="bd-placeholder-img rounded-circle" width="140" height="140"
                                xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Placeholder: 140x140"
                                preserveAspectRatio="xMidYMid slice" focusable="false">
                                <title>"Placeholder"</title>
                                <rect width="100%" height="100%" fill="#777" />
                                <text x="50%" y="50%" fill="#777" dy=".3em">"140x140"</text>
                            </svg>

                            <h2 class="fw-normal">"Example 2"</h2>
                            <p>
                                "Another exciting bit of representative placeholder content. This
                                time, we've moved on to the second column."
                            </p>
                            <p><a class="btn btn-secondary" href="#">"View details &raquo;"</a></p>
                        </div>
                        <div class="col-lg-4">
                            <svg class="bd-placeholder-img rounded-circle" width="140" height="140"
                                xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Placeholder: 140x140"
                                preserveAspectRatio="xMidYMid slice" focusable="false">
                                <title>"Placeholder"</title>
                                <rect width="100%" height="100%" fill="#777" />
                                <text x="50%" y="50%" fill="#777" dy=".3em">"140x140"</text>
                            </svg>

                            <h2 class="fw-normal">"Example 3"</h2>
                            <p>
                                "And lastly this, the third column of representative placeholder
                                content."
                            </p>
                            <p><a class="btn btn-secondary" href="#">"View details &raquo;"</a></p>
                        </div>
                    </div>
                </div>
        </Router>
    }
}

#[component(transparent)]
fn Carousel(cx: Scope) -> impl IntoView {
    let style = carousel_styles();
    styled::view! {cx, style,
        <Router>
        <div class="bg-dark text-white rounded jumbotron">
            <h1>"Jumbotron example"</h1>
            <p>"hello"</p>
        </div>

        <div id="load-navbar"></div>
        <div id="myCarousel" class="carousel slide" data-bs-ride="carousel">
            <div class="carousel-indicators">
                <button type="button" data-bs-target="#myCarousel" data-bs-slide-to="0" class="active" aria-current="true"
                    aria-label="Slide 1"></button> <button type="button" data-bs-target="#myCarousel" data-bs-slide-to="1" aria-label="Slide 2"></button>
                <button type="button" data-bs-target="#myCarousel" data-bs-slide-to="2" aria-label="Slide 3"></button>
            </div>
            <div class="carousel-inner">
                <div class="carousel-item active">
                    <svg class="bd-placeholder-img" width="100%" height="100%" xmlns="http://www.w3.org/2000/svg"
                        aria-hidden="true" preserveAspectRatio="xMidYMid slice" focusable="false">
                        <rect width="100%" height="100%" fill="#777" />
                    </svg>

                    <div class="container">
                        <div class="carousel-caption">
                            <h1>"Augmented Search"</h1>
                            <p>
                                "Implement the power of data science to explore your documents"
                            </p>
                            <p>
                                <A class="btn btn-lg btn-dark btn-primary" href="augment" exact=true>
            // <img class="my-auto"
                                        // src="/search.svg" />
            "Search"
                                </A>
                            </p>
                        </div>
                    </div>
                </div>
                <div class="carousel-item">
                    <svg class="bd-placeholder-img" width="100%" height="100%" xmlns="http://www.w3.org/2000/svg"
                        aria-hidden="true" preserveAspectRatio="xMidYMid slice" focusable="false">
                        <rect width="100%" height="100%" fill="#777" />
                    </svg>

                    <div class="container">
                        <div class="carousel-caption">
                            <h1>"Analyze your Data"</h1>
                            <p>
                                "Utilise the power of explanatory AI to see unlock more meaning
                                from your data."
                            </p>
                            <p>
                                <a class="btn btn-lg btn-dark btn-primary" href="http://localhost:8080/">"Learn more"</a>
                            </p>
                        </div>
                    </div>
                </div>
                <div class="carousel-item">
                    <svg class="bd-placeholder-img" width="100%" height="100%" xmlns="http://www.w3.org/2000/svg"
                        aria-hidden="true" preserveAspectRatio="xMidYMid slice" focusable="false">
                        <rect width="100%" height="100%" fill="#777" />
                    </svg>

                    <div class="container">
                        <div class="carousel-caption">
                            <h1>"Link your Data"</h1>
                            <p>
                                "Dive deeper into your data by utilising semantic data and
                                knowledge graphs to create meaning from your data lakes"
                            </p>
                            <p>
                                <a class="btn btn-lg btn-dark btn-primary" href="#">"Explore"</a>
                            </p>
                        </div>
                    </div>
                </div>
            </div>
            <button class="carousel-control-prev" type="button" data-bs-target="#myCarousel" data-bs-slide="prev">
                <span class="carousel-control-prev-icon" aria-hidden="true"></span>
                <span class="visually-hidden">"Previous"</span>
            </button>
            <button class="carousel-control-next" type="button" data-bs-target="#myCarousel" data-bs-slide="next">
                <span class="carousel-control-next-icon" aria-hidden="true"></span>
                <span class="visually-hidden">"Next"</span>
            </button>
        </div>
        </Router>
    }
}
