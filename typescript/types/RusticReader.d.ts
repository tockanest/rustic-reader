declare module "rustic-reader" {
    export function connect(): () =>  void;
    export function getReaderName(connection: () => void): string;

    export default class RusticReader {
        connect(): () => void;
        getReaderName(connection: void): string;
    }
}