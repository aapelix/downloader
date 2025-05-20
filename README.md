## Forked from minecraft-rs/downloader. This crate adds features like fabric, forge etc. launchers, linux support and more

# MC Downloader

Download minecraft client and libraries from rust.

## Usage

Download the client and libraries:

```rust
let path = "./.minecraft".to_string();
let version = "1.21.5".to_string();

match ClientDownloader::new() {
    Ok(downloader) => {
        println!("Start Download Minecraft {version} version in {path}");
        downloader
            .download_version(
                &version,
                &PathBuf::from(path),
                None,
                None,
                Some(Launcher::Fabric),
                Some("0.16.14"),
                None,
            )
            .unwrap();
    }
    Err(e) => println!("{e:?}"),
}

```

## Contribution

Feel free to contribute to the development of the library.
