/** Response format values; JSON schema validation remains provider-owned. */
export const ResponseFormat = Object.freeze({
    text() { return Object.freeze({ type: "text" }); },
    jsonObject() { return Object.freeze({ type: "json_object" }); },
    jsonSchema(name, schema) {
        return Object.freeze({ type: "json_schema", jsonSchema: Object.freeze({ name, schema, strict: true }) });
    },
    isJson(format) { return format.type === "json_object" || format.type === "json_schema"; },
});
//# sourceMappingURL=response_format_values.js.map