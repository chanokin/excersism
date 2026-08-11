using System;

public static class TelemetryBuffer
{
    public static byte[] ToBuffer(long reading)
    {
        var result = new byte[9];
        byte num_bytes = 2;
        result[0] = num_bytes;
        
        if (reading == 0)
            return result;


        var tmp = reading switch {
                < Int32.MinValue => (
                    (byte)(256 - 8), BitConverter.GetBytes((Int64)reading)),
                < Int16.MinValue => (
                    (byte)(256 - 4), BitConverter.GetBytes((Int32)reading)),
                < 0 => (
                    (byte)(256 - 2), BitConverter.GetBytes((Int16)reading)),
                > UInt32.MaxValue => (
                    (byte)(256 - 8), BitConverter.GetBytes((Int64)reading)),
                > Int32.MaxValue => (
                    (byte)4, BitConverter.GetBytes((UInt32)reading)),
                > UInt16.MaxValue => (
                    (byte)(256 - 4), BitConverter.GetBytes((Int32)reading)),
                >= Int16.MaxValue => (
                    (byte)2, BitConverter.GetBytes((UInt16)reading)),
                _ => (
                    (byte)(256 - 2), BitConverter.GetBytes((Int16)reading))
        };

        result[0] = tmp.Item1;
        byte[] buffer = tmp.Item2;
        for (int index = 0; index < 8; index += 1) { 
            byte current_byte = 0;
            if (index < buffer.Length) {
                 current_byte = buffer[index];
            }

            result[1 + index] = current_byte;
        }
                
        return result;
    }

    public static long FromBuffer(byte[] buffer)
    {
        if (buffer.Length != 9) return 0;
        
        byte check = buffer[0];
        
        if (check > 8 && check < 247) return 0;

        if (check > 8) {
            check = (byte)(256 - (int)check);
            if (check == 2) {
                return (long)BitConverter.ToInt16(buffer, 1); 
            }
            if (check == 4) {
                return (long)BitConverter.ToInt32(buffer, 1); 
            }
        }

        return BitConverter.ToInt64(buffer, 1);
    }
}
