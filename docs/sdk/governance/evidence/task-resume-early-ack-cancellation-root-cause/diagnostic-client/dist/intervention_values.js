export const InterventionResult = Object.freeze({
    allow() {
        return Object.freeze({ block: false, cancel: false });
    },
    block(reason) {
        if (reason.trim().length === 0)
            throw new TypeError("block reason must not be empty");
        return Object.freeze({ block: true, blockReason: reason, cancel: false });
    },
    inject(context) {
        return Object.freeze({ block: false, injectedContext: context, cancel: false });
    },
    cancel() {
        return Object.freeze({ block: false, cancel: true });
    },
    modifyArgs(args) {
        return Object.freeze({ block: false, cancel: false, modifiedArgs: args });
    },
});
//# sourceMappingURL=intervention_values.js.map