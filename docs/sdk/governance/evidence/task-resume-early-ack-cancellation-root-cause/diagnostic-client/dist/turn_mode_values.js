/** Agent turn stream modes projected as local immutable values. */
export const TurnMode = Object.freeze({
    Chat: "chat",
    Execute: "execute",
});
export function turnModeAsStr(mode) {
    if (mode !== TurnMode.Chat && mode !== TurnMode.Execute)
        throw new TypeError("invalid turn mode");
    return mode;
}
//# sourceMappingURL=turn_mode_values.js.map