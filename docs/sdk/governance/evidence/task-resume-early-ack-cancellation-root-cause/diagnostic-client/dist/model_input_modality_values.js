/** Content modalities accepted by one configured model. */
export const ModelInputModality = Object.freeze({
    Text: "text",
    Image: "image",
    Audio: "audio",
    Video: "video",
    textOnly() {
        return Object.freeze([this.Text]);
    },
    allSupported() {
        return Object.freeze([this.Text, this.Image, this.Audio, this.Video]);
    },
});
//# sourceMappingURL=model_input_modality_values.js.map