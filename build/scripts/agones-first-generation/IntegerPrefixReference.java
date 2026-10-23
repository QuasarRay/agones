// Character data and bounded comparison only; native runtime parsing stays in Rust.
import java.io.*;
import java.security.*;
import java.util.*;

public class IntegerPrefixReference {
  public static void main(String[] args) throws Exception {
    var zeroes = new ArrayList<Integer>();
    for (int unit = 0; unit <= 0xffff; unit++) {
      if (Character.digit((char) unit, 10) == 0) {
        for (int digit = 0; digit < 10; digit++) {
          if (Character.digit((char) (unit + digit), 10) != digit) throw new AssertionError("noncontiguous decimal block");
        }
        zeroes.add(unit);
      }
    }
    var digest = MessageDigest.getInstance("SHA-256");
    try (var data = new DataOutputStream(new DigestOutputStream(OutputStream.nullOutputStream(), digest))) {
      for (int unit = 0; unit <= 0xffff; unit++) {
        try {
          int value = Integer.parseInt(new String(new char[]{'1', (char) unit, '2'}));
          data.writeByte(1); data.writeInt(value);
        } catch (NumberFormatException failure) { data.writeByte(0); }
      }
    }
    System.out.println("{\"decimalZeroes\":" + zeroes + ",\"utf16Cases\":65536,\"outputSha256\":\"" + HexFormat.of().formatHex(digest.digest()) + "\"}");
  }
}
