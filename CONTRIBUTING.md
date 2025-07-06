# Contributing to yolo_io

Welcome to yolo_io! We're excited that you're interested in contributing to our YOLO computer vision dataset management library. This guide will help you get started with contributing effectively to our Rust project.

## Getting started with yolo_io development

This project handles YOLO format validation, file pairing, data quality reporting, and project export. Whether you're fixing bugs, adding features, or improving documentation, this guide covers everything you need to know about our development workflow.

### Prerequisites and environment setup

**Install Rust and essential tools:**
```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add essential components
rustup component add rustfmt clippy rust-analyzer rust-src rust-docs

# Install development tools
cargo install cargo-audit cargo-outdated cargo-tarpaulin cargo-llvm-cov

# Verify installation
rustc --version
cargo --version
```

**Platform-specific requirements:**
- **Windows**: Microsoft C++ Build Tools or Visual Studio
- **Linux**: `build-essential` package
- **macOS**: Xcode command line tools

**Clone and setup the project:**
```bash
git clone https://github.com/your-username/yolo_io.git
cd yolo_io

# Run tests to verify everything works
cargo test

# Check code formatting and linting
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

### Development workflow overview

Our development process emphasizes code quality, comprehensive testing, and clear documentation. Every contribution should include tests, follow our coding standards, and maintain backwards compatibility unless explicitly marked as breaking changes.

## Code quality standards and tooling

### Automatic code formatting with rustfmt

**Configuration:** We use a custom `rustfmt.toml` with these settings:
```toml
max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Unix"
use_field_init_shorthand = true
use_try_shorthand = true
imports_granularity = "Module"
group_imports = "StdExternalCrate"
```

**Commands:**
```bash
# Format all code
cargo fmt

# Check formatting without changing files
cargo fmt --check

# Format specific files
cargo fmt src/lib.rs src/export.rs
```

### Linting with clippy

**Configuration:** We enforce strict linting with these settings in `clippy.toml`:
```toml
msrv = "1.70.0"
avoid-breaking-exported-api = false
```

**Commands:**
```bash
# Run clippy with our standards
cargo clippy --all-targets --all-features -- -D warnings

# Fix automatically fixable issues
cargo clippy --fix --allow-dirty --allow-staged

# Run clippy on specific features
cargo clippy --no-default-features --features serde
```

### Pre-commit quality checks

**Install pre-commit hooks:**
```bash
# Install pre-commit
pip install pre-commit
pre-commit install

# Run checks manually
pre-commit run --all-files
```

Our pre-commit configuration runs:
- **Code formatting** with rustfmt
- **Linting** with clippy
- **Test execution** to catch breaking changes
- **Documentation checks** to ensure examples work

## Testing requirements and patterns

### Unit testing with rstest framework

We use `rstest` for powerful testing patterns. **Key testing requirements:**

**Basic test structure:**
```rust
use rstest::*;

#[fixture]
fn sample_yolo_data() -> YoloData {
    YoloData::new("0 0.5 0.5 0.3 0.3")
}

#[rstest]
fn test_yolo_parsing(sample_yolo_data: YoloData) {
    assert_eq!(sample_yolo_data.class_id, 0);
    assert_eq!(sample_yolo_data.x_center, 0.5);
}

#[rstest]
#[case(0, 0.5, 0.5, 0.3, 0.3)]
#[case(1, 0.2, 0.8, 0.4, 0.2)]
fn test_yolo_validation(
    #[case] class_id: u32,
    #[case] x: f32,
    #[case] y: f32,
    #[case] w: f32,
    #[case] h: f32,
) {
    let result = YoloData::new_with_values(class_id, x, y, w, h);
    assert!(result.is_ok());
}
```

**Testing commands:**
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_yolo_parsing

# Run integration tests only
cargo test --test integration_tests

# Run tests with all features
cargo test --all-features
```

### Error handling testing patterns

Since we use `thiserror` for error handling, **test error conditions thoroughly:**

```rust
#[test]
fn test_invalid_yolo_format() {
    let result = YoloData::parse("invalid format");
    assert!(matches!(result, Err(YoloError::InvalidFormat { .. })));
}

#[test]
fn test_coordinate_out_of_range() {
    let result = YoloData::parse("0 1.5 0.5 0.3 0.3");
    assert!(matches!(result, Err(YoloError::CoordinateOutOfRange { .. })));
}
```

### File I/O testing with tempfile

**Test file operations safely:**
```rust
use tempfile::tempdir;

#[test]
fn test_dataset_export() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.txt");
    
    let dataset = create_test_dataset();
    dataset.export_to_file(&output_path).unwrap();
    
    let content = std::fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("0 0.5 0.5 0.3 0.3"));
}
```

### Performance testing with criterion

**Add benchmarks for performance-critical code:**
```rust
// In benches/performance.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_yolo_parsing(c: &mut Criterion) {
    let data = "0 0.5 0.5 0.3 0.3";
    c.bench_function("yolo_parse", |b| {
        b.iter(|| YoloData::parse(black_box(data)))
    });
}

criterion_group!(benches, benchmark_yolo_parsing);
criterion_main!(benches);
```

```bash
# Run benchmarks
cargo bench

# Generate HTML reports
cargo bench -- --output-format html
```

### Code coverage requirements

**We aim for 80%+ test coverage:**
```bash
# Generate coverage report
cargo llvm-cov --html

# Check coverage percentage
cargo llvm-cov --summary-only

# Coverage with doctests
cargo llvm-cov --doctests
```

## Documentation standards

### API documentation requirements

**Every public item must have documentation:**
```rust
/// Parses YOLO format annotation string into structured data
///
/// YOLO format consists of: `class_id x_center y_center width height`
/// where all coordinates are normalized to [0.0, 1.0]
///
/// # Arguments
///
/// * `line` - A string slice containing the YOLO format annotation
///
/// # Returns
///
/// Returns `Ok(YoloData)` on successful parsing, or `Err(YoloError)`
/// if the format is invalid or coordinates are out of range
///
/// # Errors
///
/// * `YoloError::InvalidFormat` - If the line doesn't contain exactly 5 fields
/// * `YoloError::CoordinateOutOfRange` - If any coordinate is outside [0.0, 1.0]
/// * `YoloError::InvalidClassId` - If class_id is not a valid integer
///
/// # Examples
///
/// ```rust
/// use yolo_io::YoloData;
///
/// let data = YoloData::parse("0 0.5 0.5 0.3 0.3")?;
/// assert_eq!(data.class_id, 0);
/// assert_eq!(data.x_center, 0.5);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn parse(line: &str) -> Result<YoloData, YoloError> {
    // Implementation
}
```

### Documentation testing

**All code examples in documentation are tested:**
```bash
# Test documentation examples
cargo test --doc

# Generate and view documentation
cargo doc --open

# Check documentation without building
cargo doc --no-deps --document-private-items
```

## Project-specific development guidelines

### Computer vision dataset handling patterns

**Memory-efficient processing for large datasets:**
```rust
// Process datasets in chunks to manage memory
pub struct DatasetProcessor {
    chunk_size: usize,
}

impl DatasetProcessor {
    pub fn process_large_dataset(&self, path: &Path) -> Result<(), DatasetError> {
        let reader = BufReader::new(File::open(path)?);
        
        for chunk in reader.lines().chunks(self.chunk_size) {
            let batch: Result<Vec<_>, _> = chunk.collect();
            self.process_batch(batch?)?;
        }
        
        Ok(())
    }
}
```

### Performance considerations for file I/O

**Always use buffered I/O for frequent operations:**
```rust
use std::io::{BufReader, BufWriter};

// Efficient reading
let file = File::open("annotations.txt")?;
let reader = BufReader::new(file);

// Efficient writing
let file = File::create("output.txt")?;
let mut writer = BufWriter::new(file);
writer.flush()?; // Always flush explicitly
```

### Error handling with thiserror

**Follow our error handling patterns:**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatasetError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Invalid YOLO format in {file}:{line}: {message}")]
    InvalidFormat { file: String, line: usize, message: String },
    
    #[error("Dataset validation failed: {0}")]
    ValidationError(String),
}
```

### Cross-platform file handling

**Use standard library abstractions:**
```rust
use std::path::{Path, PathBuf};

// Cross-platform path handling
fn get_annotation_path(image_path: &Path) -> PathBuf {
    let mut annotation_path = image_path.to_path_buf();
    annotation_path.set_extension("txt");
    annotation_path
}
```

## Pull request guidelines

### Before submitting a pull request

**Complete this checklist:**

- [ ] **Tests added** for new functionality
- [ ] **All tests pass** (`cargo test`)
- [ ] **Code formatted** (`cargo fmt`)
- [ ] **No clippy warnings** (`cargo clippy`)
- [ ] **Documentation updated** for public APIs
- [ ] **Examples work** (`cargo test --doc`)
- [ ] **Benchmarks added** for performance-critical code
- [ ] **Changelog updated** if user-facing changes

### Pull request process

1. **Create a focused branch** from `main`
2. **Write clear commit messages** describing the change
3. **Include tests** that verify the fix or feature
4. **Update documentation** as needed
5. **Submit PR** with clear description

### Commit message format

**Use conventional commits:**
```
feat: add support for YOLO v8 format
fix: handle empty annotation files gracefully
docs: improve API documentation for export module
perf: optimize file pairing algorithm
test: add integration tests for large datasets
```

### Code review expectations

**Reviews focus on:**
- **Correctness**: Logic handles edge cases properly
- **Performance**: No obvious inefficiencies for large datasets
- **Safety**: Proper error handling and validation
- **Testing**: Adequate test coverage for changes
- **Documentation**: Clear examples and API docs

## Development commands reference

### Essential daily commands

```bash
# Development cycle
cargo check                              # Quick syntax check
cargo test                               # Run all tests
cargo clippy                             # Check for issues
cargo fmt                                # Format code

# Documentation and examples
cargo doc --open                         # View generated docs
cargo test --doc                         # Test documentation examples
cargo run --example basic_usage          # Run example

# Performance and quality
cargo bench                              # Run benchmarks
cargo llvm-cov --html                    # Generate coverage report
cargo audit                              # Check for vulnerabilities
```

### Feature development workflow

```bash
# Start new feature
git checkout -b feature/yolo-v8-support

# Development cycle
cargo check                              # Fast compilation check
cargo test test_yolo_v8                  # Run specific tests
cargo test --                           # Run all tests
cargo clippy --all-targets --all-features -- -D warnings

# Before committing
cargo fmt --check                        # Verify formatting
cargo test --release                     # Test release build
cargo doc --no-deps                      # Check documentation builds

# Commit and push
git add -A
git commit -m "feat: add YOLO v8 format support"
git push origin feature/yolo-v8-support
```

## Getting help and community

### Resources for contributors

- **Documentation**: Generated docs at `cargo doc --open`
- **Examples**: Working examples in `examples/` directory
- **Tests**: Comprehensive test suite demonstrates usage patterns
- **Issues**: Check existing issues before creating new ones

### Code of conduct

We follow the [Rust Code of Conduct](https://www.rust-lang.org/conduct.html). Be respectful, inclusive, and collaborative. Focus on constructive feedback and helping each other improve.

### Communication guidelines

- **Ask questions** in issues or discussions
- **Be specific** when reporting bugs or requesting features
- **Provide context** about your use case
- **Help others** by answering questions when you can

## Project architecture and design patterns

### Key design principles

**Type safety and validation**: We use Rust's type system to prevent invalid states. All input validation happens at boundaries, and internal APIs work with validated data structures.

**Error propagation**: Errors are handled explicitly using `Result` types. We provide rich error context using `thiserror` to help users understand and fix issues.

**Memory efficiency**: For large datasets, we use streaming patterns and avoid loading entire files into memory. Buffer sizes are tuned for performance.

**Cross-platform compatibility**: All file operations use standard library abstractions. Path handling works consistently across Windows, macOS, and Linux.

### Contributing to core modules

**Understanding the codebase structure:**
- `yolo_file.rs` - Core YOLO format parsing and validation
- `pairing.rs` - Logic for matching annotation files with images
- `export.rs` - Dataset export functionality
- `report.rs` - Data quality reporting and validation
- `types.rs` - Shared data structures and type definitions
- `file_utils.rs` - File I/O utilities and cross-platform operations

Each module has comprehensive unit tests and clear public APIs. Study existing patterns before adding new functionality.

## Performance and optimization guidelines

### Profiling and benchmarking

**Before optimizing, measure performance:**
```bash
# Profile with flamegraph
cargo install flamegraph
cargo bench --bench dataset_processing -- --profile-time=5

# Memory profiling
cargo install cargo-profdata
cargo profdata -- merge -sparse default.profraw -o default.profdata
```

### Common optimization patterns

**Use appropriate data structures:**
- `Vec<T>` for sequential access
- `HashMap<K, V>` for key-value lookups
- `BTreeMap<K, V>` for sorted data
- `HashSet<T>` for uniqueness checks

**Minimize allocations:**
- Use `&str` instead of `String` when possible
- Reuse buffers for repeated operations
- Consider `Cow<str>` for conditional cloning

**Optimize I/O operations:**
- Always use `BufReader` and `BufWriter`
- Process files in chunks for large datasets
- Use `async` for I/O-bound operations with multiple files

---

Thank you for contributing to yolo_io! Your contributions help make computer vision dataset management more efficient and reliable for everyone. Whether you're fixing bugs, adding features, or improving documentation, every contribution makes a difference.