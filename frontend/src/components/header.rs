use leptos::*;

use crate::{components::drop_down::DropDown, types::Profile, util::Api};

#[component]
pub fn Header(logged_in: ReadSignal<bool>) -> impl IntoView {
    let (user, set_user) = create_signal(None::<Profile>);
    let (dropdown, set_dropdown) = create_signal(false);

    spawn_local(async move {
        set_user(Api::get_profile().await.ok());
    });

    view! {
        <header class="bg-white shadow-xl rounded-lg">
            <div class="mx-auto max-w-screen-xl px-4 sm:px-6 lg:px-8">
                <div class="flex h-64 md:h-48 lg:h-16 items-center justify-between">
                    <div class="flex-1 md:flex md:items-center md:gap-12">
                        <a class="block text-black" href="/">
                        <span class="sr-only">Home</span>
                        // blog.svg
                        <img src="assets/blog_black.svg" class="md:h-48 lg:h-16 md:w-48 lg:w-16"/>
                        // <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 5120 5110"><g id="l2Khb3ZxvTi3O7SUZDGbPNt" fill="rgb(0,0,0)" style="transform: none;"><g style="transform: none;"><path id="pF9CBXpNJ" d="M155 4921 c-57 -27 -102 -67 -138 -121 -16 -23 -17 -197 -17 -2233 0 -1886 2 -2212 14 -2235 20 -38 84 -97 133 -122 23 -12 70 -25 105 -30 84 -13 4532 -13 4616 0 35 5 82 18 105 30 49 25 113 84 133 122 12 23 14 349 14 2235 0 2036 -1 2210 -17 2233 -36 54 -81 94 -138 121 l-60 29 -968 0 -969 0 -29 -29 c-26 -27 -29 -36 -29 -95 0 -58 3 -69 29 -98 l29 -33 922 -3 c844 -2 923 -4 947 -19 56 -37 53 48 53 -1683 l0 -1600 -2330 0 -2330 0 0 1600 c0 1732 -3 1646 53 1683 24 16 99 17 984 17 608 0 971 4 994 10 53 15 72 41 77 109 5 63 -13 112 -47 131 -13 7 -362 10 -1048 10 l-1028 0 -60 -29z m4735 -4086 c0 -320 -4 -345 -53 -378 -25 -16 -173 -17 -2277 -17 -2104 0 -2252 1 -2277 17 -49 33 -53 58 -53 378 l0 295 2330 0 2330 0 0 -295z"></path><path id="p17mbPNjZY" d="M584 892 c-32 -25 -44 -55 -44 -109 0 -95 62 -144 164 -129 69 11 96 48 96 133 0 58 -3 67 -29 94 -27 26 -35 29 -98 29 -49 0 -74 -5 -89 -18z"></path><path id="p5aKpfmcP" d="M1069 881 c-26 -27 -29 -35 -29 -99 0 -61 3 -72 26 -95 48 -49 176 -44 209 7 37 55 31 148 -12 190 -23 23 -34 26 -95 26 -64 0 -72 -3 -99 -29z"></path><path id="pOnmEBCIb" d="M1573 892 c-46 -36 -58 -133 -23 -189 38 -63 166 -70 215 -12 21 26 25 40 25 96 0 58 -3 67 -29 94 -27 26 -35 29 -98 29 -49 0 -74 -5 -90 -18z"></path><path id="pJVBb4YbV" d="M2417 884 c-50 -50 -44 -176 10 -212 25 -16 94 -17 1056 -17 l1029 0 29 33 c26 29 29 40 29 98 0 59 -3 68 -29 95 l-29 29 -1035 0 -1034 0 -26 -26z"></path><path id="pUShxCDyH" d="M2529 4921 c-26 -26 -29 -36 -29 -93 0 -74 16 -103 66 -124 99 -41 194 13 194 112 0 100 -35 134 -135 134 -60 0 -69 -3 -96 -29z"></path><path id="pYg610c84" d="M1033 4393 c-33 -6 -70 -49 -78 -90 -16 -89 23 -157 98 -168 23 -3 717 -5 1542 -3 1628 3 1526 0 1557 57 7 13 13 47 13 76 0 29 -6 63 -13 76 -31 58 73 54 -1577 55 -836 1 -1530 -1 -1542 -3z"></path><path id="p11DwAE4e4" d="M3606 3865 c-111 -45 -189 -141 -215 -263 -34 -163 16 -243 144 -229 44 5 47 5 27 -7 -42 -23 -114 -100 -138 -148 -13 -26 -29 -82 -36 -123 -41 -265 125 -465 387 -465 160 0 285 73 353 207 36 70 44 163 40 476 -4 278 -5 295 -28 352 -32 83 -123 174 -203 203 -86 33 -249 31 -331 -3z m224 -258 c47 -31 69 -73 77 -147 l6 -62 -24 8 c-42 13 -211 7 -257 -10 l-43 -16 24 29 c14 17 27 48 30 76 8 54 37 99 81 126 41 25 66 24 106 -4z m-2 -477 c46 -28 74 -77 68 -119 -6 -39 -55 -95 -97 -110 -57 -20 -149 56 -149 124 0 50 74 125 125 125 11 0 35 -9 53 -20z"></path><path id="p7dBpspJA" d="M1033 3414 c-36 -8 -72 -52 -78 -97 -3 -23 -5 -287 -3 -587 3 -588 2 -573 57 -602 18 -10 85 -13 265 -12 225 0 245 2 298 22 104 42 181 118 215 214 9 24 18 82 21 130 5 99 -19 201 -61 252 l-24 30 27 38 c60 82 78 264 37 376 -34 97 -111 173 -215 213 -52 20 -78 22 -287 25 -126 1 -240 0 -252 -2z m419 -273 c32 -12 75 -57 86 -91 8 -22 6 -36 -9 -67 -35 -68 -58 -78 -196 -81 l-123 -4 0 126 0 126 109 0 c60 0 120 -4 133 -9z m0 -520 c32 -12 75 -57 86 -91 8 -22 6 -36 -9 -67 -35 -68 -58 -78 -196 -81 l-123 -4 0 126 0 126 109 0 c60 0 120 -4 133 -9z"></path><path id="pxPqz7H74" d="M2142 3389 c-64 -25 -139 -96 -172 -164 l-25 -50 -3 -496 -3 -497 27 -31 c24 -29 29 -31 99 -31 64 0 75 3 97 25 14 14 28 39 32 56 3 18 6 218 6 446 0 366 2 418 16 443 20 33 56 55 108 64 21 4 49 14 62 22 59 39 58 179 -1 218 -36 24 -175 21 -243 -5z"></path><path id="pufuEjIe3" d="M2739 3396 c-98 -37 -185 -122 -220 -215 -33 -85 -31 -234 2 -317 66 -161 207 -244 394 -231 96 7 154 26 217 72 180 132 198 443 37 604 -24 24 -69 56 -99 70 -48 23 -69 26 -175 28 -82 2 -131 -2 -156 -11z m190 -261 c43 -22 81 -72 81 -107 0 -72 -90 -147 -151 -126 -35 12 -86 69 -95 105 -9 35 21 90 65 120 40 27 61 29 100 8z"></path></g></g><g id="l5YiAwBqgQStSnoy8YSkHL9" fill="rgb(255,255,255)" style="transform: none;"><g style="transform: none;"><path id="ptaYbbZ7L" d="M1 4883 c0 -198 2 -223 13 -188 28 90 86 156 171 197 l60 28 1003 0 1003 0 24 -24 c47 -47 43 -112 -9 -150 -27 -21 -36 -21 -1007 -24 -900 -2 -982 -4 -1006 -19 -56 -37 -53 51 -53 -1713 l0 -1630 2360 0 2360 0 0 1630 c0 1764 3 1676 -53 1713 -24 15 -103 17 -947 19 l-922 3 -29 33 c-40 44 -40 93 0 133 l29 29 939 0 938 0 60 -28 c85 -41 143 -107 171 -197 11 -35 13 -10 13 188 l1 227 -2560 0 -2560 0 1 -227z m2684 21 c14 -9 30 -33 36 -54 31 -102 -104 -167 -171 -83 -66 85 44 197 135 137z m1402 -554 c60 -40 60 -130 0 -170 -19 -13 -227 -15 -1492 -18 -808 -2 -1489 0 -1512 3 -91 13 -128 107 -66 169 13 14 34 27 46 29 12 2 692 4 1512 3 1285 -1 1493 -3 1512 -16z m-180 -512 c80 -29 171 -120 203 -203 22 -56 24 -76 28 -321 2 -164 0 -288 -7 -334 -27 -169 -139 -284 -305 -313 -81 -14 -178 8 -253 58 -240 159 -197 533 73 635 70 26 184 29 239 6 21 -9 42 -16 48 -16 6 0 8 36 5 99 -5 112 -22 151 -78 188 -39 27 -116 31 -160 8 -44 -23 -79 -76 -86 -129 -12 -87 -73 -129 -142 -100 -50 20 -67 70 -52 148 25 128 102 224 216 271 79 33 189 34 271 3z m-2365 -477 c228 -88 308 -354 165 -548 l-35 -48 34 -45 c116 -152 87 -381 -64 -496 -90 -69 -132 -77 -368 -78 -156 -1 -217 2 -235 12 -55 29 -54 16 -57 572 -2 283 0 534 3 557 6 45 42 89 78 97 12 2 112 3 222 2 177 -3 206 -6 257 -25z m813 3 c33 -22 48 -71 34 -111 -13 -38 -42 -59 -95 -69 -52 -9 -88 -31 -108 -64 -14 -25 -16 -77 -16 -443 0 -228 -3 -428 -6 -446 -17 -81 -117 -111 -168 -50 l-27 31 3 467 3 466 25 50 c33 68 108 139 172 164 65 24 150 27 183 5z m685 -15 c152 -72 237 -249 196 -410 -63 -245 -341 -353 -549 -214 -178 118 -214 372 -75 530 82 93 157 125 283 122 73 -2 101 -7 145 -28z"></path><path id="p798MnqPL" d="M3694 3161 c-91 -56 -107 -176 -33 -250 62 -63 152 -65 218 -5 81 72 70 199 -22 254 -42 26 -121 26 -163 1z"></path><path id="pqmEj4nR" d="M1180 3025 l0 -156 153 3 c133 3 155 5 179 23 33 24 68 91 68 130 0 39 -35 106 -68 130 -24 18 -46 20 -179 23 l-153 3 0 -156z"></path><path id="p5jE77c1P" d="M1180 2505 l0 -156 153 3 c133 3 155 5 179 23 33 24 68 91 68 130 0 39 -35 106 -68 130 -24 18 -46 20 -179 23 l-153 3 0 -156z"></path><path id="pn6oh5CHf" d="M2800 3157 c-92 -61 -97 -187 -10 -260 77 -65 194 -36 240 59 38 78 7 169 -71 209 -43 23 -120 19 -159 -8z"></path><path id="p1Clwd2BXx" d="M200 835 c0 -352 3 -375 53 -408 25 -16 175 -17 2307 -17 2132 0 2282 1 2307 17 50 33 53 56 53 408 l0 325 -2360 0 -2360 0 0 -325z m541 16 c38 -38 39 -87 4 -129 -47 -55 -130 -44 -161 21 -47 99 79 186 157 108z m492 4 c35 -36 41 -70 20 -113 -45 -92 -183 -62 -183 40 0 85 104 132 163 73z m498 -4 c39 -40 40 -89 1 -132 -46 -51 -125 -40 -158 22 -50 97 79 188 157 110z m2780 0 c40 -40 40 -89 0 -133 l-29 -33 -999 0 c-936 0 -1001 1 -1026 18 -28 18 -46 67 -39 104 2 12 16 34 29 48 l26 25 1004 0 1005 0 29 -29z"></path><path id="pdyGmBiMy" d="M1 238 l-1 -238 2560 0 2560 0 -1 238 c0 206 -2 232 -13 197 -19 -61 -41 -97 -83 -137 -52 -50 -110 -78 -185 -88 -84 -13 -4472 -13 -4556 0 -133 19 -228 99 -268 225 -11 35 -13 9 -13 -197z"></path></g></g></svg>
                        </a>
                    </div>

                    <div class="md:flex md:items-center md:gap-12">
                        <nav aria-label="Global" class="hidden lg:block">
                            <ul class="flex items-center gap-6 text-sm list-none">
                                <li>
                                <a class="text-gray-500 transition hover:text-gray-500/75" target="_blank" href="https://www.nicolas-frey.com"> About </a>
                                </li>

                                <li>
                                <a class="text-gray-500 transition hover:text-gray-500/75" href="/feed"> Feed </a>
                                </li>

                                <li>
                                <a class="text-gray-500 transition hover:text-gray-500/75" href="/subscribe"> Subscribe </a>
                                </li>

                                <li>
                                <a class="text-gray-500 transition hover:text-gray-500/75" href="/profile"> Profile </a>
                                </li>
                            </ul>
                        </nav>

                        <div class="flex items-center gap-4">
                            <Show when=move || !logged_in.get() fallback=move || view!{
                                <div class="sm:flex sm:gap-4">
                                    <a
                                    class="rounded-md bg-red-600 px-5 py-2.5 md:text-2xl lg:text-sm font-medium text-white shadow"
                                    href="/logout"
                                    >
                                    Logout
                                    </a>
                                    <div class="hidden sm:flex">
                                        <a class="text-gray-500 md:text-2xl lg:text-sm px-5 py-2.5">
                                        {move || {
                                            user.get().unwrap_or_default().name
                                        }}
                                        </a>
                                    </div>
                                </div>
                            }>
                                <div class="sm:flex sm:gap-4">
                                    <a
                                    class="rounded-md bg-black px-5 py-2.5 md:text-2xl lg:text-sm font-medium text-white shadow"
                                    href="/login"
                                    >
                                    Login
                                    </a>

                                    <div class="hidden sm:flex">
                                    <a
                                        class="rounded-md bg-gray-100 px-5 py-2.5 md:text-2xl lg:text-sm font-medium text-black"
                                        href="/register"
                                    >
                                        Register
                                    </a>
                                    </div>
                                </div>
                            </Show>

                            <div class="block lg:hidden">
                                <button
                                    class="rounded-full h-16 w-16 bg-black border-none p-2 text-white transition"
                                    on:click=move |_| set_dropdown(!dropdown.get())
                                >
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="size-5"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                    stroke-width="2"
                                >
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16" />
                                </svg>
                                </button>
                                <Show when=move || dropdown.get()>
                                    <DropDown actions={
                                        vec![
                                            ("About", "https://www.nicolas-frey.com", Some("_blank")),
                                            ("Feed", "/feed", None),
                                            ("Subscribe", "/subscribe", None),
                                            ("Profile", "/profile", None),
                                        ]
                                    }/>
                                </Show>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </header>
    }
}
