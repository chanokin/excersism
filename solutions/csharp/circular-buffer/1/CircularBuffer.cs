public class CircularBuffer<T>
{
    private T[] buffer;
    private int write_index = 0;
    private int read_index = 0;
    private bool cleared = true;
    public CircularBuffer(int capacity)
    {
        
        buffer = new T[capacity];
    }

    public T Read()
    {
        if (cleared) {
            throw new InvalidOperationException("empty buffer can't be accessed");
        }
            
        int _read_index = read_index % buffer.Length;
        read_index += 1;

        // we have read all entries
        if (read_index == write_index) {
            cleared = true;
        }

        return buffer[_read_index];
    }

    public void Write(T value)
    {
        // check if full
        if ((write_index - read_index) >= buffer.Length) {
            throw new InvalidOperationException("Full buffer!");
        }
        
        buffer[write_index % buffer.Length] = value;
        write_index = write_index + 1;
        cleared = false;
    }

    public void Overwrite(T value)
    {
        // check if not full
        if ((write_index - read_index) < buffer.Length) {
            Write(value);
            return;
        }
        
        buffer[read_index % buffer.Length] = value;
        read_index = read_index + 1;
        write_index = write_index + 1;
    }

    public void Clear()
    {
        write_index = 0;
        read_index = 0;
        cleared = true;
    }
}