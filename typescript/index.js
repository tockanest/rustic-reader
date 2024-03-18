"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var rustic_reader_1 = require("rustic-reader");
var TypedRusticReader = /** @class */ (function () {
    function TypedRusticReader() {
        this.reader = new rustic_reader_1.RusticReader();
    }
    TypedRusticReader.prototype.connect = function () {
        this.reader.connect();
    };
    TypedRusticReader.prototype.getReaderName = function () {
        return this.reader.getReaderName(this.connect);
    };
    return TypedRusticReader;
}());
var reader = new TypedRusticReader();
var connection = reader.connect();
var readerName = reader.getReaderName();
console.log(readerName);
