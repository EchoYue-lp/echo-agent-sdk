/** Content modalities accepted by one configured model. */
export declare const ModelInputModality: Readonly<{
    readonly Text: "text";
    readonly Image: "image";
    readonly Audio: "audio";
    readonly Video: "video";
    readonly textOnly: () => readonly ModelInputModality[];
    readonly allSupported: () => readonly ModelInputModality[];
}>;
export type ModelInputModality = "text" | "image" | "audio" | "video";
