export interface CatalogRoute {
    readonly family: string;
    readonly method: string | null;
    readonly operation: string | null;
    readonly handler_operation?: string | null;
    readonly route: string;
    readonly signature_digests?: readonly string[];
    readonly operation_signatures?: readonly {
        readonly operation: string;
        readonly signature_digests: readonly string[];
    }[];
    readonly required_feature?: string | null;
    readonly required_features?: readonly string[];
    readonly feature_semantics?: string;
}
export interface ResolvedOperation {
    readonly operation: string;
    readonly family: string;
    readonly method: string;
    readonly signature: string;
}
export interface CatalogOperation {
    /** Stable identity scoped by the catalog family. */
    readonly identity: string;
    readonly operation: string;
    readonly family: string;
    readonly method: string;
    readonly route: string;
    readonly signature: string;
    readonly signature_digests: readonly string[];
    readonly required_feature: string | null;
    readonly required_features: readonly string[];
    readonly feature_semantics: string;
}
export interface CatalogFamily {
    readonly family: string;
    readonly methods: readonly string[];
    readonly operations: readonly CatalogOperation[];
    readonly required_feature: string | null;
}
export interface CatalogDocument {
    readonly schema_version: number;
    readonly extension_protocol_version: number;
    readonly total_items: number;
    readonly routes: readonly CatalogRoute[];
    readonly families: readonly {
        readonly family: string;
        readonly methods: readonly string[];
        readonly operations: readonly string[];
        readonly required_feature?: string | null;
    }[];
}
export declare class FacadeCatalog {
    readonly document: CatalogDocument;
    readonly contractDigest: string;
    readonly sourceContractDigest: string;
    private readonly operationRoutes;
    private readonly familyRoutes;
    private readonly operationList;
    private readonly familyList;
    constructor(path?: string);
    operations(): readonly CatalogOperation[];
    families(): readonly CatalogFamily[];
    signature(operation: string): string;
    familySignature(method: string, operation: string): string;
    resolve(operation: string): ResolvedOperation;
    route(operation: string): CatalogRoute;
    private toCatalogOperation;
}
