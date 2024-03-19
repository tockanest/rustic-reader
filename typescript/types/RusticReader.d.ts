declare module "rustic-reader" {
    export function connect(): () =>  void;
    export function getReaderName(connection: () => void): string;
    export function readNdef(connection: () => void, blockNumber: number): any;
}