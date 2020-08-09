import { clearHtml, replacePage } from "../common.js";
import { setCurrentPage } from "../index.js";

export function onFetchError() {
    clearHtml();

    const errorDisplay = document.createElement("div");
    errorDisplay.id = "error-display";
    errorDisplay.textContent = "Failed to load";
    replacePage(errorDisplay);
    setCurrentPage("error");

    console.error("text is undefined: getTextFromUrl failed?");
}