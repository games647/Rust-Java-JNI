package pack.age;

import java.nio.file.Path;
import java.nio.file.Paths;

public class Test {

    public static native void helloWorld();

    public static native void getInfo();

    public static void main(String[] args) {
        //append system dependent library extension
        String libName = System.mapLibraryName("rust_jni_test");

        //path to the library based from the working dir
        Path path = Paths.get(libName);

        //load it using a custom path. System.loadLibrary would load it from the JAVA_LIB_PATH
        //or using the system properties -Djava.library.path=CUSTOM_LIB_PATH
        System.load(path.toAbsolutePath().toString());

        Test.helloWorld();
        Test.getInfo();
    }
}