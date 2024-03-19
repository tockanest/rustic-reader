declare module "rustic-reader" {
    export function connect(): () =>  void;
    export function getReaderName(connection: () => void): string;
    export function readBlocks(connection: () => void, blockNumber: number): Buffer;
    export function readNdef(connection: () => void): Buffer;
}