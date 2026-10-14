/** Pagination metadata projection without owning collection state. */
export class PageInfo {
    nextCursor;
    truncated;
    totalKnown;
    total;
    returned;
    constructor(nextCursor = null, truncated = false, totalKnown = false, total = null, returned = 0n) {
        this.nextCursor = nextCursor;
        this.truncated = truncated;
        this.totalKnown = totalKnown;
        this.total = total;
        this.returned = returned;
    }
    applyTo(result) {
        let updated = result
            .withTruncated(result.truncated || this.truncated)
            .withMeta("page.truncated", String(this.truncated))
            .withMeta("page.total_known", String(this.totalKnown))
            .withMeta("page.returned", this.returned.toString());
        if (this.total !== null)
            updated = updated.withMeta("page.total", this.total.toString());
        if (this.nextCursor !== null) {
            updated = updated.withMeta("page.next_cursor", this.nextCursor);
            const continuation = JSON.stringify({
                next_cursor: this.nextCursor,
                returned: this.returned.toString(),
                total: this.total === null ? null : this.total.toString(),
                total_known: this.totalKnown,
                truncated: true,
            });
            updated = updated.withOutput(`${updated.output}\n[page]${continuation}`);
        }
        return updated;
    }
}
//# sourceMappingURL=page_info_values.js.map