declare type CallbackFunction = () => Record<string, any>;

declare function createStyles(callback: CallbackFunction): string;

export { createStyles };
