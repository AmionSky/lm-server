export const requestUrl = "http://192.168.1.101:11277/";

export async function getJsonFromUrl(url) {
    return fetchTimeout(url, 4000).then(res => {
        if (res.ok) {
            return res.json();
        } else {
            throw "Non-ok return";
        }
    });
}

export function replacePage(to) {
    clearHtml();
    getRoot().appendChild(to);
}

export function getRoot() {
    return document.getElementById("window-content");
}

export function clearHtml() {
    const root = getRoot();
    if (root) {
        while (root.firstChild) {
            root.firstChild.remove();
        }
    }
}

export function getCoverUrl(groupId) {
    return requestUrl.concat("cover/", groupId);
}

export function getGroupUrl(groupId) {
    return requestUrl.concat("group/", groupId);
}

export function getVideoUrl(groupId, videoName) {
    return requestUrl.concat("video/", groupId, "/", encodeURI(videoName));
}

export function getSubUrl(groupId, videoName) {
    return requestUrl.concat("sub/", groupId, "/", encodeURI(videoName));
}

function fetchTimeout(url, timeout) {
    const req = fetch(url);
    const tmo = new Promise((_, reject) => {
        return setTimeout(() => reject(new Error('request timeout')), timeout);
    });

    return Promise.race([req, tmo]);
}