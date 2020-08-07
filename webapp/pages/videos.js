import { replacePage, getJsonFromUrl, getGroupUrl, getCoverUrl } from "../common.js";
import { onFetchError } from "./error.js";

const store = {};

export async function showVideosPage(uid) {
    const json = await getJsonFromUrl(getGroupUrl(uid)).catch(() => undefined);

    if (json == undefined) {
        onFetchError();
        return;
    }

    const grid = document.createElement("div");
    grid.appendChild(createCoverDisplay(uid));
    grid.appendChild(createVideoList(uid, json.videos));
    grid.id = "mg-grid";
    replacePage(grid);
}

function createVideoList(groupId, videos) {
    const listElement = document.createElement("div");
    listElement.id = "mg-videos";

    // TODO: sort videos

    for (let i = 0; i < videos.length; i++) {
        const elem = createMediaItem(i, groupId, videos[i]);
        listElement.appendChild(elem);
    }

    return listElement;
}

function createMediaItem(index, groupId, details) {
    const container = document.createElement("button");
    container.className = "mg-videos-item";
    container.addEventListener("click", () => {
        store[watched(groupId, index)] = true;
        container.className = "mg-videos-item mg-videos-item-watched";
        startPlayer(groupId, details.name);
    });
    container.onmousedown = (event) => {
        if (event.which == 3) {
            if (isWatched(groupId, index)) {
                store[watched(groupId, index)] = false;
                container.className = "mg-videos-item";
            } else {
                store[watched(groupId, index)] = true;
                container.className = "mg-videos-item mg-videos-item-watched";
            }

        }
    }

    if (isWatched(groupId, index)) {
        container.className += " mg-videos-item-watched";
    }

    const seqnum = document.createElement("span");
    seqnum.textContent = (index + 1) + "";
    seqnum.className = "seqnum";
    container.appendChild(seqnum);

    const title = document.createElement("span");
    title.textContent = getTitle(details.name);
    title.className = "title";
    container.appendChild(title);

    return container;
}

function getTitle(videoName) {
    let title = videoName;
    const extIndex = title.lastIndexOf('.');
    if (extIndex > 0) {
        title = title.substring(0, extIndex);
    }
    return title;
}

function createCoverDisplay(uid) {
    const leftContainer = document.createElement("div");
    leftContainer.appendChild(createShadowElement());
    leftContainer.id = "mg-cover";
    leftContainer.style.backgroundImage = "url(\"" + getCoverUrl(uid) + "\")";
    return leftContainer;
}

function createShadowElement() {
    const shadow = document.createElement("div");
    shadow.id = "mg-cover-shadow";
    return shadow;
}

function watched(groupId, index) {
    return "watched." + groupId + "." + index;
}

function isWatched(groupId, index) {
    return store[watched(groupId, index)];
}

function startPlayer(groupId, videoName) {
    window.external["notify"]("play/".concat(groupId, "/", videoName));
    // TODO
}