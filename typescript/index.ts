import {connect, getReaderName} from "rustic-reader";

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
}

const reader = new TypedRusticReader();
const readerName = reader.getReaderName(reader.connection());
console.log(readerName);