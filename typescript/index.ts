import {connect, getReaderName, readNdef, readBlocks} from "rustic-reader";

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
        return readNdef(connection);
    }
    
    readBlocks(connection: () => void, blockNumber: number) {
        return readBlocks(connection, blockNumber);
    }
}

const reader = new TypedRusticReader();
const readerName = reader.getReaderName(reader.connection());
console.log(readerName);
const ndef = reader.readNdef(reader.connection(), 4);
console.log("Buffer: ", ndef);
const str = ndef.toString("utf8");
const newStr = str.replace(/[^ -~]+/g, '');
const getFirstBracket = newStr.indexOf("{");
const getLastBracket = newStr.lastIndexOf("}");
const newStr2 = newStr.substring(getFirstBracket, getLastBracket + 1);
console.log(newStr2);


