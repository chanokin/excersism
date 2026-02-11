import kotlin.collections.ArrayDeque

// TODO: implement proper exceptions to complete the task
class EmptyBufferException : Exception("Empty buffer") 
class BufferFullException : Exception("Buffer full")

class CircularBuffer<T>(val _size: Int) {
    var size: Int = _size
    var read_index: Int = 0;
    var write_index: Int = 0
    // type here is Any__?__ to allow for Null valued array
    var buffer: Array<Any?> = arrayOfNulls(_size)

    fun is_empty(): Boolean {
        return (this.write_index <= this.read_index);
    }

    fun is_full(): Boolean {
        return (this.write_index - this.read_index) >= this.size;
    }
    
    fun read() : T {
        if (this.is_empty()) {
            throw EmptyBufferException();
        }
        var value: T = buffer[this.read_index % this.size] as T;
        this.read_index += 1;
        return value;
    }

    fun write(value: T) {
        if (this.is_full()) {
            throw BufferFullException();
        }
        this.buffer[this.write_index % this.size] = value as Any?;
        this.write_index += 1;

    }

    fun overwrite(value: T) {
        if (! this.is_full()) {
            // no need to overwrite as the thing still has some space
            this.write(value);
            return;
        } 

        this.buffer[this.read_index % this.size] = value as Any?;
        this.read_index += 1;
        this.write_index += 1;
        
    }

    fun clear() {
        if (this.is_empty()) {
            return
        }

        this.read_index = this.write_index;
    }
}