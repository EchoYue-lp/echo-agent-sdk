/** Agent turn stream modes projected as local immutable values. */
export declare const TurnMode: Readonly<{
    readonly Chat: "chat";
    readonly Execute: "execute";
}>;
export type TurnMode = (typeof TurnMode)[keyof typeof TurnMode];
export declare function turnModeAsStr(mode: TurnMode): string;
