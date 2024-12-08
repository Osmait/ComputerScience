import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

public class main {
  public static void main(String[] args) throws IOException {
    byte[] file = Files.readAllBytes(Paths.get("../input.txt"));
    String content = new String(file, "UTF-8");
    System.out.println(content);
  }
}
