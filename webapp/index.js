import { showCollectionsPage } from "./pages/collections.js";
import { showVideosPage } from "./pages/videos.js";
import { getRoot } from "./common.js"

// If notify isn't implemented, make a dummy implementation
if (window.external["notify"] == undefined) {
    window.external["notify"] = (p) => { console.log("notify: " + p) }
}

// Handle window navigation
window.onpopstate = (event) => {
    if (event.state != null) {
        window.location.hash = event.state;
    }
    load();
}

async function load() {
    let url = new URL(window.location.href);
    let collection = url.searchParams.get("collection");

    // Load the requested page
    if (collection != undefined) {
        await showVideosPage(collection);
    } else {
        await showCollectionsPage();

        // Scroll
        if (window.location.hash != "") {
            // Scroll to the specified hash
            let element = window.document.querySelector(window.location.hash);
            if (element != undefined) {
                element.scrollIntoView();
            }
        } else {
            // Scroll to top
            getRoot().scrollTo({ top: 0, behavior: "smooth" });
        }
    }
}

// Load the page
load();