import { replacePage, requestUrl, getJsonFromUrl, getCoverUrl } from "../common.js";
import { onFetchError } from "./error.js";
import { showVideosPage } from "./videos.js";
import { setCurrentPage } from "../index.js";

export async function showCollectionsPage() {
    // Get the response from server
    const json = await getJsonFromUrl(requestUrl).catch(() => undefined);

    // Verify the response
    if (json == undefined) {
        onFetchError();
        return;
    }

    // Create page
    const listElement = document.createElement("div");
    listElement.id = "ml-grid";

    // Sort media groups alphabetically
    json.media_list.sort((a, b) => {
        let x = a.name.toLowerCase();
        let y = b.name.toLowerCase();
        if (x < y) { return -1; }
        if (x > y) { return 1; }
        return 0;
    });

    // create items
    for (const item of json.media_list) {
        const elem = createMediaListItem(item);
        listElement.appendChild(elem);
    }

    replacePage(listElement);
    setCurrentPage("collections");
}

function createMediaListItem(item) {
    const container = document.createElement("div");
    container.id = item.uid;
    container.className = "ml-groupitem";
    container.addEventListener("click", () => {
        // Push state describing the location on the page
        if (history.state == undefined) {
            history.pushState(null, "", "#" + item.uid);
        } else {
            history.replaceState(null, "", "#" + item.uid);
        }
        // Push state describing the next page
        history.pushState(null, item.name, "?collection=" + item.uid);
        window.external["notify"]("nav/" + window.location.href);
        // Load the next page
        showVideosPage(item.uid);
    });

    // Create cover image element
    const coverImage = document.createElement("img");
    coverImage.className = "cover";
    coverImage.src = getCoverUrl(item.uid);
    container.appendChild(coverImage);

    // Create title text element
    const title = document.createElement("span");
    title.textContent = item.name
    title.className = "title";
    container.appendChild(title);

    return container;
}
