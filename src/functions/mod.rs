use crate::pages::*;
use augment::*;
use footer::*;
use index::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use navbar::*;

#[component]
pub(crate) fn RenderIndex(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {cx,
        <Meta name="description"/>
            <Router>
                <nav>
                    <NavBar/>
                </nav>
            <main>
                <Routes>
                    <Route path="" view=move |cx| view! {cx, <Index/> } ssr=SsrMode::Async />
                    <Route path="augment" view=move |cx| view! {cx, <Augment/> } ssr=SsrMode::Async />
                </Routes>
            </main>
                <Footer/>
            </Router>
    }
}
