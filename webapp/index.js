import { showCollectionsPage } from "./pages/collections.js";
import { showVideosPage } from "./pages/videos.js";
import { getRoot } from "./common.js";

let currentPage = "none";
export function setCurrentPage(pageName) {
    currentPage = pageName;
}

// If notify isn't implemented, make a dummy implementation
if (window.external["notify"] == undefined) {
    window.external["notify"] = (p) => { console.log("notify: " + p) }
}

// Handle window navigation
window.onpopstate = (_) => {
    load();
}

async function load() {
    let url = new URL(window.location.href);
    let collection = url.searchParams.get("collection");

    // Load the requested page
    if (collection != undefined) {
        if (currentPage !== "videos") {
            await showVideosPage(collection);
        }
    } else {
        if (currentPage !== "collections") {
            await showCollectionsPage();
        }

        // Scroll
        if (window.location.hash != "") {
            // Scroll to the specified hash
            scrollToElement(window.location.hash);
        } else {
            // Scroll to top
            // Needs overflow: auto; on root to work.
            getRoot().scrollTo({ top: 0, behavior: "smooth" });
        }
    }
}

// Scroll to element by id
function scrollToElement(id) {
    let element = window.document.querySelector(id);
    if (element != undefined) {
        element.scrollIntoView();
    }
}

// Load the page
load();