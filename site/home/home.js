// Lists the documentation versions from docs/versions.json, written by
// site/build_site.py. Without it, the static master link stays.

(function () {
    "use strict";

    const list = document.getElementById("versions");
    if (list === null) {
        return;
    }

    fetch("docs/versions.json", { cache: "no-cache" })
        .then((response) => (response.ok ? response.json() : null))
        .then((manifest) => {
            const versions = Array.isArray(manifest && manifest.versions) ? manifest.versions : [];
            if (versions.length === 0) {
                return;
            }
            const items = versions.map((version) => {
                const item = document.createElement("li");
                const link = document.createElement("a");
                link.href = "docs/" + encodeURIComponent(version.name) + "/";
                link.textContent = version.title || version.name;
                item.appendChild(link);
                if (version.name === manifest.default) {
                    const badge = document.createElement("span");
                    badge.className = "badge";
                    badge.textContent = "default";
                    item.appendChild(badge);
                }
                return item;
            });
            list.replaceChildren(...items);
        })
        .catch(() => {});
})();
