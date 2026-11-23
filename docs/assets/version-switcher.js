// Adds a documentation version dropdown to the mdBook menu bar.
//
// Each version is a separate book under `docs/<version>/`, and the versions
// are listed in `docs/versions.json`, both written by site/build_site.py.
// When that file is missing, as under `mdbook serve`, nothing is shown.

(function () {
    "use strict";

    // `path_to_root` is defined by mdBook on every page: "" for pages at the
    // book root, "../" one level down and so on. Resolve it against the base
    // URL, which the 404 page sets to `site-url`.
    if (typeof path_to_root === "undefined") {
        return;
    }
    const bookRoot = new URL(path_to_root === "" ? "./" : path_to_root, document.baseURI);
    const segments = bookRoot.pathname.split("/").filter(Boolean);
    if (segments.length === 0) {
        return;
    }
    const current = decodeURIComponent(segments[segments.length - 1]);
    const docsRoot = new URL("../", bookRoot);

    // The path of this page inside the book, such as `module/hooks.html`.
    function currentPage() {
        const path = window.location.pathname;
        return path.startsWith(bookRoot.pathname) ? path.slice(bookRoot.pathname.length) : "";
    }

    // Goes to the same page of another version, or to its first page when
    // the page does not exist there.
    async function switchTo(version) {
        const versionRoot = new URL(encodeURIComponent(version) + "/", docsRoot);
        const page = currentPage();
        let destination = versionRoot.href;
        if (page !== "") {
            const samePage = new URL(page, versionRoot);
            try {
                const response = await fetch(samePage, { method: "HEAD" });
                if (response.ok) {
                    destination = samePage.href + window.location.hash;
                }
            } catch (error) {
                // Fall back to the first page.
            }
        }
        window.location.href = destination;
    }

    function render(manifest) {
        const versions = Array.isArray(manifest && manifest.versions) ? manifest.versions : [];
        if (!versions.some((version) => version.name === current)) {
            return;
        }
        const buttons = document.querySelector("#mdbook-menu-bar .right-buttons");
        if (buttons === null) {
            return;
        }

        const select = document.createElement("select");
        select.className = "version-switcher";
        select.setAttribute("aria-label", "Documentation version");
        select.title = "Documentation version";
        for (const version of versions) {
            const option = document.createElement("option");
            option.value = version.name;
            option.textContent = version.title || version.name;
            option.selected = version.name === current;
            select.appendChild(option);
        }
        select.addEventListener("change", () => {
            select.disabled = true;
            switchTo(select.value);
        });
        buttons.insertBefore(select, buttons.firstChild);
    }

    fetch(new URL("versions.json", docsRoot), { cache: "no-cache" })
        .then((response) => (response.ok ? response.json() : null))
        .then(render)
        .catch(() => {});
})();
