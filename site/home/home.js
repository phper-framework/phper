// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

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
