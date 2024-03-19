import {connect, getReaderName, readNdef} from "rustic-reader";

class TypedRusticReader {
    private reader: () => void;

    constructor() {
        this.reader = connect();
    }

    connection() {
        return this.reader;
    }

    getReaderName(connection: () => void) {
        return getReaderName(connection);
    }
    
    readNdef(connection: () => void, blockNumber: number) {
        return readNdef(connection, blockNumber);
    }
}

const reader = new TypedRusticReader();
const readerName = reader.getReaderName(reader.connection());
console.log(readerName);
const ndef = reader.readNdef(reader.connection(), 4);
console.log(ndef);
const str = ndef.toString("utf8");
console.log(str)

