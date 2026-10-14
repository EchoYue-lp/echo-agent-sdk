/** A2A JWT configuration and claims values without owning token verification. */
export declare class JwtConfig {
    private readonly algorithm;
    private readonly verificationKey;
    readonly issuer: string | null;
    readonly audience: string | null;
    private constructor();
    static hs256(secret: string): JwtConfig;
    static rs256(publicKey: string): JwtConfig;
    static disabled(): JwtConfig;
    withIssuer(issuer: string): JwtConfig;
    withAudience(audience: string): JwtConfig;
    isEnabled(): boolean;
    toString(): string;
}
export declare class JwtClaims {
    readonly iss: string | null;
    readonly sub: string | null;
    readonly aud: string | null;
    readonly exp: bigint | null;
    readonly nbf: bigint | null;
    readonly iat: bigint | null;
    readonly jti: string | null;
    readonly extra: Readonly<Record<string, unknown>>;
    constructor(iss?: string | null, sub?: string | null, aud?: string | null, exp?: bigint | null, nbf?: bigint | null, iat?: bigint | null, jti?: string | null, extra?: Readonly<Record<string, unknown>>);
    subject(): string | undefined;
}
