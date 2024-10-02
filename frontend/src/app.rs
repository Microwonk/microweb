use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::cursor::Cursor;
use crate::pages::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Nicolas Frey"/>
        <Html lang="en" attr:data-theme="light" class="scroll-smooth cursor-none"/>
        <Cursor/>

        <Router>
            <Routes>
                <Route path="" view=HomePage/>
                <Route path="/*any" view=NotFound/>
                <Route path="/projects/:slug" view=UseCasesPage/>
                <Route path="/resume" view=DownloadCVPage/>
            </Routes>
        </Router>
    }
}
