export type JsonSchemaSpec = Readonly<{
    name: string;
    schema: unknown;
    strict: boolean;
}>;
export type ResponseFormat = Readonly<{
    type: "text";
}> | Readonly<{
    type: "json_object";
}> | Readonly<{
    type: "json_schema";
    jsonSchema: JsonSchemaSpec;
}>;
/** Response format values; JSON schema validation remains provider-owned. */
export declare const ResponseFormat: Readonly<{
    text(): ResponseFormat;
    jsonObject(): ResponseFormat;
    jsonSchema(name: string, schema: unknown): ResponseFormat;
    isJson(format: ResponseFormat): boolean;
}>;
