// Build-time comparison only. The native CPG runtime does not call Java.
import java.io.*;
import java.nio.charset.StandardCharsets;
import java.security.*;
import java.util.HexFormat;

public class UtilityUtf8Reference {
  public static void main(String[] args) throws Exception {
    var digest = MessageDigest.getInstance("SHA-256");
    long count = 0;
    try (var input = new DataInputStream(new FileInputStream(args[0]));
         var output = new DataOutputStream(new DigestOutputStream(OutputStream.nullOutputStream(), digest))) {
      while (input.available() != 0) {
        int size = input.readUnsignedByte();
        byte[] bytes = input.readNBytes(size);
        if (bytes.length != size) throw new EOFException();
        String decoded = new String(bytes, StandardCharsets.UTF_8);
        output.writeInt(decoded.length());
        for (int i = 0; i < decoded.length(); i++) output.writeChar(decoded.charAt(i));
        count++;
      }
    }
    System.out.println("{\"cases\":" + count + ",\"utf16_output_sha256\":\"" + HexFormat.of().formatHex(digest.digest()) + "\"}");
  }
}
