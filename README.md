# Aleph Ollama Code Translator (Experimental)

An experimental library for translating source code to different programming languages using Ollama.

## Description

This library enables translation of source code files from one programming language to another using Ollama AI models capabilities. It's an experimental tool that can be helpful for quick code conversion between different languages.

## Prerequisites

- Rust installed on your system
- Ollama installed and running locally
- A compatible model (default: qwen2.5-coder)

## Installation

```bash
cargo add aleph_ollama
```

## Usage

The library can be used via command line through the provided example:

```bash
cargo run --example basic <source_file_path> <target_language>
```

### Example

```bash
cargo r --example basic test/dataset/syracuse.ml java
```

Which produces the following output:

```java
import java.util.ArrayList;
import java.util.List;

public class Syracuse {
    public static void main(String[] args) {
        int n = 7;
        List<Integer> suite = syracuse(n);
        System.out.print("Syracuse for " + n + " : ");
        for (int x : suite) {
            System.out.print(x + " ");
        }
        System.out.println();
    }

    public static List<Integer> syracuse(int n) {
        if (n <= 0) {
            throw new IllegalArgumentException("Le nombre doit être strictement positif");
        } else if (n == 1) {
            List<Integer> list = new ArrayList<>();
            list.add(1);
            return list;
        } else if (n % 2 == 0) {
            List<Integer> list = syracuse(n / 2);
            list.add(0, n);
            return list;
        } else {
            List<Integer> list = syracuse(3 * n + 1);
            list.add(0, n);
            return list;
        }
    }
}
```

## Configuration

The library can be configured through environment variables:

- `OLLAMA_MODEL`: The model to use for translation (default: "qwen2.5-coder")

## Limitations

- Translation quality depends on the model used
- Some language-specific concepts may not have direct equivalents in the target language
- The tool is experimental and may require manual adjustments of generated code

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

[Insert your license here]
