// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="index.html"><strong aria-hidden="true">1.</strong> LALRPOP</a></li><li class="chapter-item expanded "><a href="crash_course.html"><strong aria-hidden="true">2.</strong> Crash course on parsers</a></li><li class="chapter-item expanded "><a href="quick_start_guide.html"><strong aria-hidden="true">3.</strong> Quick start guide</a></li><li class="chapter-item expanded "><a href="cheatsheet.html"><strong aria-hidden="true">4.</strong> Cheatsheet</a></li><li class="chapter-item expanded "><a href="tutorial/index.html"><strong aria-hidden="true">5.</strong> Tutorial</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tutorial/001_adding_lalrpop.html"><strong aria-hidden="true">5.1.</strong> Adding LALRPOP to your project</a></li><li class="chapter-item expanded "><a href="tutorial/002_paren_numbers.html"><strong aria-hidden="true">5.2.</strong> Parsing parenthesized numbers</a></li><li class="chapter-item expanded "><a href="tutorial/003_type_inference.html"><strong aria-hidden="true">5.3.</strong> Type inference</a></li><li class="chapter-item expanded "><a href="tutorial/004_full_expressions.html"><strong aria-hidden="true">5.4.</strong> Handling full expressions</a></li><li class="chapter-item expanded "><a href="tutorial/005_building_asts.html"><strong aria-hidden="true">5.5.</strong> Building ASTs</a></li><li class="chapter-item expanded "><a href="tutorial/006_macros.html"><strong aria-hidden="true">5.6.</strong> Macros</a></li><li class="chapter-item expanded "><a href="tutorial/007_fallible_actions.html"><strong aria-hidden="true">5.7.</strong> Fallible actions</a></li><li class="chapter-item expanded "><a href="tutorial/008_error_recovery.html"><strong aria-hidden="true">5.8.</strong> Error recovery</a></li><li class="chapter-item expanded "><a href="tutorial/009_state_parameter.html"><strong aria-hidden="true">5.9.</strong> Passing state parameter</a></li></ol></li><li class="chapter-item expanded "><a href="lexer_tutorial/index.html"><strong aria-hidden="true">6.</strong> Controlling the lexer</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="lexer_tutorial/001_lexer_gen.html"><strong aria-hidden="true">6.1.</strong> LALRPOP&#39;s lexer generator</a></li><li class="chapter-item expanded "><a href="lexer_tutorial/002_raw_delimited_content.html"><strong aria-hidden="true">6.2.</strong> Lexing raw delimited content</a></li><li class="chapter-item expanded "><a href="lexer_tutorial/003_writing_custom_lexer.html"><strong aria-hidden="true">6.3.</strong> Writing a custom lexer</a></li><li class="chapter-item expanded "><a href="lexer_tutorial/004_token_references.html"><strong aria-hidden="true">6.4.</strong> Using tokens with references</a></li><li class="chapter-item expanded "><a href="lexer_tutorial/005_external_lib.html"><strong aria-hidden="true">6.5.</strong> Using an external library</a></li><li class="chapter-item expanded "><a href="lexer_tutorial/006_error_recovery_custom_lexer.html"><strong aria-hidden="true">6.6.</strong> Error recovery with custom lexer</a></li></ol></li><li class="chapter-item expanded "><a href="advanced_setup.html"><strong aria-hidden="true">7.</strong> Advanced setup</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="generate_in_source.html"><strong aria-hidden="true">7.1.</strong> Generate in source tree</a></li><li class="chapter-item expanded "><a href="conditional-compilation.html"><strong aria-hidden="true">7.2.</strong> Conditional compilation</a></li><li class="chapter-item expanded "><a href="location_tracking.html"><strong aria-hidden="true">7.3.</strong> Location Tracking</a></li></ol></li><li class="chapter-item expanded "><li class="spacer"></li><li class="chapter-item expanded affix "><a href="misc/contributors.html">Contributors</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
