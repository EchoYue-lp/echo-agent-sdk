/** A2A JWT configuration and claims values without owning token verification. */
export class JwtConfig {
    algorithm;
    verificationKey;
    issuer;
    audience;
    constructor(algorithm, verificationKey, issuer, audience) {
        this.algorithm = algorithm;
        this.verificationKey = verificationKey;
        this.issuer = issuer;
        this.audience = audience;
    }
    static hs256(secret) {
        if (typeof secret !== "string")
            throw new TypeError("JWT secret must be text");
        return new JwtConfig("HS256", secret, null, null);
    }
    static rs256(publicKey) {
        if (typeof publicKey !== "string" || !looksLikeRsaPublicKey(publicKey)) {
            throw new Error("invalid RSA public key");
        }
        return new JwtConfig("RS256", publicKey, null, null);
    }
    static disabled() {
        return new JwtConfig(null, null, null, null);
    }
    withIssuer(issuer) {
        if (typeof issuer !== "string")
            throw new TypeError("JWT issuer must be text");
        return new JwtConfig(this.algorithm, this.verificationKey, issuer, this.audience);
    }
    withAudience(audience) {
        if (typeof audience !== "string")
            throw new TypeError("JWT audience must be text");
        return new JwtConfig(this.algorithm, this.verificationKey, this.issuer, audience);
    }
    isEnabled() {
        return this.algorithm !== null;
    }
    toString() {
        return `JwtConfig { enabled: ${this.isEnabled()}, algorithm: ${this.algorithm ?? "none"}, issuer: ${this.issuer ?? "null"}, audience: ${this.audience ?? "null"}, verification_key: [redacted] }`;
    }
}
export class JwtClaims {
    iss;
    sub;
    aud;
    exp;
    nbf;
    iat;
    jti;
    extra;
    constructor(iss = null, sub = null, aud = null, exp = null, nbf = null, iat = null, jti = null, extra = {}) {
        this.iss = iss;
        this.sub = sub;
        this.aud = aud;
        this.exp = exp;
        this.nbf = nbf;
        this.iat = iat;
        this.jti = jti;
        this.extra = extra;
    }
    subject() {
        return this.sub ?? undefined;
    }
}
function looksLikeRsaPublicKey(value) {
    const trimmed = value.trim();
    return (trimmed.startsWith("-----BEGIN PUBLIC KEY-----")
        && trimmed.endsWith("-----END PUBLIC KEY-----"))
        || (trimmed.startsWith("-----BEGIN RSA PUBLIC KEY-----")
            && trimmed.endsWith("-----END RSA PUBLIC KEY-----"));
}
//# sourceMappingURL=jwt_values.js.map