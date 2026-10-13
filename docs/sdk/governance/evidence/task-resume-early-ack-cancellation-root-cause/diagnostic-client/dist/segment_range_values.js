/** Message-cache segment range values without owning cache state. */
export class SegmentRange {
    start;
    end;
    constructor(start = 0n, end = 0n) {
        this.start = start;
        this.end = end;
        if (start < 0n || end < 0n)
            throw new RangeError("segment range bounds must be non-negative");
        Object.freeze(this);
    }
    len() {
        return this.end > this.start ? this.end - this.start : 0n;
    }
    isEmpty() {
        return this.len() === 0n;
    }
}
//# sourceMappingURL=segment_range_values.js.map