<a name="readme-top"></a>
# ytb-downloader

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

A Rust library for downloading videos from Youtube, choosing between all the available formats.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- GETTING STARTED -->
## Getting Started

Add `ytb-downloader` to Cargo.toml and import it:

```
use ytb-downloader::*;
```

Note that the library uses async/await so
```
tokio = { version = "1.27.0", features = ["macros", "rt-multi-thread", "fs"] }
```
is needed as well. These are the necessary features of Tokio for `ytb-downloader` but you can add more if you need to.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->
## Usage

The examples below use error_chain for error handling, that's why the Result doesn't also have an error type parameter.


Downloading a video using the `download_video!` macro:
```rust
#[macro_use] extern crate ytb_downloader;
use ytb_downloader::*;
use errors::*;

[tokio::main]
async fn main() -> Result<()> {
  let source = get_available_sources("https://www.youtube.com/watch?v=pqhfyrW_BEA").await?
      .into_iter().next().unwrap();
  const OUTPUT_FILE: &str = "download.m4a"; 
  download_video!(&source, OUTPUT_FILE).await;
  Ok(())
}
```

Downloading a video using the actual function call:
```rust
use ytb_downloader::*;
use errors::*;

[tokio::main]
async fn main() -> Result<()> {
  let source = get_available_sources("https://www.youtube.com/watch?v=pqhfyrW_BEA").await?
      .into_iter().next().unwrap();
  const OUTPUT_FILE: &str = "download.m4a"; 
  download_video(&source, OUTPUT_FILE, None).await;
  Ok(())
}
```

Downloading a video with a chunk size of 10240 bytes:
```rust
use ytb_downloader::*;
use errors::*;

[tokio::main]
async fn main() -> Result<()> {
  let source = get_available_sources("https://www.youtube.com/watch?v=pqhfyrW_BEA").await?
      .into_iter().next().unwrap();
  const OUTPUT_FILE: &str = "download.m4a"; 
  download_video(&source, OUTPUT_FILE, Some(10240)).await;
  Ok(())
}
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- CONTRIBUTING -->
## Contributing

Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Pavlos Smith - paulsmith4561+at+gmail.com

<p align="right">(<a href="#readme-top">back to top</a>)</p>
